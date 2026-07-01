use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr;

use bc_indicators::indicators::ready_imports::Indicator;
use bc_pack_indicators::FUNCS_EXTRACT_ARGS as FA_I;
use bc_pack_signals_ready::FUNCS_EXTRACT_ARGS as FA_R;
use bc_pack_signals_train::FUNCS_EXTRACT_ARGS as FA_T;
use bc_signals::ready::ready_trait::SignalReady;
use bc_signals::train::train_trait::SignalTrain;
use bc_utils_lg::settings::{SETTINGS, SETTINGS_IND, SETTINGS_SIGNAL};
use bc_utils_lg::types::maps::FUNCS_EXTRACT_ARGS_TYPE as FA;

use crate::indicators::{Indicators, IndicatorsGateway};
use crate::signals_ready::{SignalsReady, SignalsReadyGateway};
use crate::signals_train::{SignalsTrain, SignalsTrainGateway};
use crate::trade::statistics::StatCollector;
use crate::trade::utils_cell::orders_create;
use crate::trade::structs::TradeCell;

#[derive(Default)]
pub struct GWValues<'a> {
    pub indicators: Indicators<'a>,
    pub signals_ready: SignalsReady<'a>,
    pub signals_train: SignalsTrain<'a>,
}

impl<'a> GWValues<'a> {
    pub fn new(
        s: &'a SETTINGS,
        funcs_args_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
        funcs_args_signals_ready: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
        funcs_args_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
        src: &[Vec<f64>],
    ) -> Self {
        let bind = Indicators::new(&s.indications, funcs_args_indicators, src);
        Self {
            signals_ready: SignalsReady::new(
                &s.signals_ready,
                &s.indications,
                funcs_args_signals_ready,
                src,
                &bind.indicators_without_bf,
            ),
            signals_train: SignalsTrain::new(
                &s.signals_train,
                &s.indications,
                funcs_args_signals_train,
                src,
                &bind.indicators_without_bf,
            ),
            indicators: bind,
        }
    }
}

pub struct TradeData<'a> {
    pub gw_values: GWValues<'a>,
    pub indicators_gateway: IndicatorsGateway<'a>,
    pub signals_ready_gateway: SignalsReadyGateway<'a>,
    pub signals_train_gateway: SignalsTrainGateway<'a>,
    pub cell: RefCell<TradeCell>,
    pub symbol: &'a str,
    s: &'a SETTINGS,
    _pin: PhantomPinned,
}

// backtest
// let file_reader = FileReader::(s);
// let exch = file_reader.get_exch();
// let mut trade_data_collector = TradeDataCollector::default();
// let src = exch.src(num + trade_data_collector.indicators.w_max());
// buffer mut = buffer::new(file_reader.src_or(&src[..buffer_size]));
// trade_data_collector.update_buffer(&buffer);
// let stat_collector = StatCollector::new(s);

// for series in &src[buffer_size..] {
// trade_data_collector.update(series);
// stat_collector.write_any_data_column();
// }

impl<'a> TradeData<'a> {
    pub fn new(
        src: &[Vec<f64>],
        s: &'a SETTINGS,
        symbol: &'a str,
    ) -> Pin<Box<Self>> {
        let mut res = Box::pin(Self {
            gw_values: GWValues::new(s, &FA_I(), &FA_R(), &FA_T(), src),
            cell: RefCell::new(TradeCell::new(
                s.trade.capital,
                src[src.len() - 1].to_vec(),
                src[src.len() - 2].to_vec(),
            )),
            symbol: symbol,
            s: s,
            indicators_gateway: IndicatorsGateway::new(std::ptr::null(), &s.indications),
            signals_ready_gateway: SignalsReadyGateway::new(
                ptr::null(),
                ptr::null(),
                &s.signals_ready,
                &s.indications,
            ),
            signals_train_gateway: SignalsTrainGateway::new(
                ptr::null(),
                ptr::null(),
                &s.signals_train,
                &s.indications,
            ),
            _pin: PhantomPinned,
        });
        let outer_mut = unsafe { Pin::as_mut(&mut res).get_unchecked_mut() };
        outer_mut.indicators_gateway.indicators = &outer_mut.gw_values.indicators;
        outer_mut.signals_ready_gateway.indicators = &outer_mut.gw_values.indicators;
        outer_mut.signals_train_gateway.indicators = &outer_mut.gw_values.indicators;
        outer_mut.signals_ready_gateway.signals_ready = &outer_mut.gw_values.signals_ready;
        outer_mut.signals_train_gateway.signals_train = &outer_mut.gw_values.signals_train;
        res
    }
    pub fn update(
        self: Pin<&Self>,
        buffer: &[Vec<f64>],
        stat_collector: Option<&mut StatCollector<'a>>,
    ) {
        let indications = self.indicators_gateway.indications_series(buffer);
        let orders = orders_create(
            &self.s.trade,
            self.cell.borrow().borrow(),
            self.symbol,
            &indications,
            &self
                .signals_ready_gateway
                .signals_series(&indications, buffer),
            buffer,
        );
        self.as_ref().get_ref().cell.borrow_mut().step(
            buffer.last().unwrap(),
            &buffer[buffer.len() - 2],
            orders,
            &self.as_ref().get_ref().s.trade,
            stat_collector,
        );
    }
    pub fn update_bf(
        &mut self,
        buffer: &[Vec<f64>],
    ) {
        self.gw_values
            .indicators
            .update_bf(buffer, &(&*self.s).indications, &FA_I());
        self.gw_values.signals_ready.update_bf(
            buffer,
            &self.s,
            &FA_R(),
            &self.gw_values.indicators.indicators_without_bf,
        );
        self.gw_values.signals_train.update_bf(
            buffer,
            &self.s,
            &FA_T(),
            &self.gw_values.indicators.indicators_without_bf,
        );
    }
}
