use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr;

use bc_indicators::indicator_traits::Indicator;
use bc_orders_collectors::main_trait::OrderCollector;
use bc_pack_indicators::FUNCS_EXTRACT_ARGS as FA_I;
use bc_pack_signals_ready::FUNCS_EXTRACT_ARGS as FA_R;
use bc_pack_signals_train::FUNCS_EXTRACT_ARGS as FA_T;
use bc_signals::ready::ready_trait::SignalReady;
use bc_signals::train::train_trait::SignalTrain;
use bc_utils_lg::types::maps::MAP;
use bc_utils_lg::{
    structs::{
        settings::{SETTINGS, SETTINGS_IND, SETTINGS_ORDER_COLLECTOR, SETTINGS_SIGNAL},
        trade::TradeCell,
    },
    types::maps::FUNCS_EXTRACT_ARGS_TYPE as FA,
};

use crate::indicators::{Indicators, IndicatorsGateway};
use crate::orders_collectors::{OrdersCollectors, OrdersCollectorsGateway};
use crate::signals_ready::{SignalsReady, SignalsReadyGateway};
use crate::signals_train::{SignalsTrain, SignalsTrainGateway};
use crate::trade::{statistics::StatCollector, structs::StepCell, utils_cell::orders_create};

#[derive(Default)]
pub struct GWValues<'a> {
    pub indicators: Indicators<'a>,
    pub signals_ready: SignalsReady<'a>,
    pub signals_train: SignalsTrain<'a>,
    pub orders_collectors: OrdersCollectors,
}

impl<'a> GWValues<'a> {
    pub fn new(
        s: &'a SETTINGS,
        fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
        fa_signals_ready: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
        fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
        fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
        src: &[Vec<f64>],
    ) -> Self {
        let bind = Indicators::new(&s.indications, fa_indicators, src);
        Self {
            signals_ready: SignalsReady::new(
                &s.signals_ready,
                &s.indications,
                fa_signals_ready,
                src,
                &bind.indicators_without_bf,
            ),
            signals_train: SignalsTrain::new(
                &s.signals_train,
                &s.indications,
                fa_signals_train,
                src,
                &bind.indicators_without_bf,
            ),
            indicators: bind,
            orders_collectors: OrdersCollectors::new(
                &s.trade.order_collectors,
                fa_orders_collectors,
            ),
        }
    }
}

pub struct TradeData<'a> {
    pub gw_values: GWValues<'a>,
    pub indicators_gateway: IndicatorsGateway<'a>,
    pub signals_ready_gateway: SignalsReadyGateway<'a>,
    pub signals_train_gateway: SignalsTrainGateway<'a>,
    pub orders_collectors_gateway: OrdersCollectorsGateway<'a>,
    pub cell: RefCell<TradeCell>,
    pub symbol: &'a str,
    s: &'a SETTINGS,
    _pin: PhantomPinned,
}

impl<'a> TradeData<'a> {
    pub fn new(
        src: &[Vec<f64>],
        s: &'a SETTINGS,
        symbol: &'a str,
        fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
        fa_signals_ready: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
        fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
        fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
    ) -> Pin<Box<Self>> {
        let mut res = Box::pin(Self {
            // fa change on fa args in new
            gw_values: GWValues::new(
                s,
                fa_indicators,
                fa_signals_ready,
                fa_signals_train,
                fa_orders_collectors,
                src,
            ),
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
            orders_collectors_gateway: OrdersCollectorsGateway::new(
                ptr::null(),
                &s.trade.order_collectors,
            ),
            _pin: PhantomPinned,
        });
        let outer_mut = unsafe { Pin::as_mut(&mut res).get_unchecked_mut() };
        outer_mut.indicators_gateway.indicators = &outer_mut.gw_values.indicators;
        outer_mut.signals_ready_gateway.indicators = &outer_mut.gw_values.indicators;
        outer_mut.signals_train_gateway.indicators = &outer_mut.gw_values.indicators;
        outer_mut.signals_ready_gateway.signals_ready = &outer_mut.gw_values.signals_ready;
        outer_mut.signals_train_gateway.signals_train = &outer_mut.gw_values.signals_train;
        outer_mut.orders_collectors_gateway.order_collectors =
            &outer_mut.gw_values.orders_collectors;
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
            &self.get_ref().orders_collectors_gateway,
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

pub struct AfterTradeData<'a> {
    pub indicators_values: Indicators<'a>,
    pub indicators_columns: Indicators<'a>,
    pub indicators_gateway_values: IndicatorsGateway<'a>,
    pub indicators_gateway_columns: IndicatorsGateway<'a>,
    _pin: PhantomPinned,
}

impl<'a> AfterTradeData<'a> {
    pub fn new(
        s: &'a SETTINGS,
        src: &[Vec<f64>],
        fa: &FA<SETTINGS_IND, Box<dyn Indicator>>,
    ) -> Pin<Box<Self>> {
        let mut res = Box::pin(Self {
            indicators_values: Indicators::new(&s.indications_stat_values, fa, src),
            indicators_columns: Indicators::new(&s.indications_stat_values, fa, src),
            indicators_gateway_values: IndicatorsGateway {
                indicators: ptr::null(),
                settings: &s.indications_stat_values,
            },
            indicators_gateway_columns: IndicatorsGateway {
                indicators: ptr::null(),
                settings: &s.indications_stat_columns,
            },
            _pin: PhantomPinned,
        });
        let outer_pin = unsafe { res.as_mut().get_unchecked_mut() };
        outer_pin.indicators_gateway_values.indicators = &outer_pin.indicators_values;
        outer_pin.indicators_gateway_columns.indicators = &outer_pin.indicators_columns;
        res
    }
    pub fn to_stat_values(
        &self,
        data: &[Vec<f64>],
    ) -> MAP<String, f64> {
        self.indicators_gateway_values
            .indications_series(data)
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect()
    }
    pub fn to_stat_columns(
        &self,
        data: &[Vec<f64>],
    ) -> MAP<String, Vec<f64>> {
        self.indicators_gateway_columns
            .indications_vec(data)
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect()
    }
}
