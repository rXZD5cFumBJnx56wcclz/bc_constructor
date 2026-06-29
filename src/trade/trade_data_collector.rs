use std::cell::RefCell;

use bc_indicators::indicators::ready_imports::Indicator;
use bc_signals::ready::ready_trait::SignalReady;
use bc_signals::train::train_trait::SignalTrain;
use bc_utils_lg::settings::{SETTINGS, SETTINGS_IND, SETTINGS_SIGNAL};
use bc_utils_lg::types::maps::FUNCS_EXTRACT_ARGS_TYPE as FA;

use crate::indicators::{Indicators, IndicatorsGateway};
use crate::signals_ready::{SignalReadyGateway, SignalsReady};
use crate::signals_train::{SignalTrainGateway, SignalsTrain};
use crate::trade::{
    structs::{Order, TradeCell},
    utils_cell::order_create,
};

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
// struct Buffer: klines, time, orderbook ind, cmc20, fear & gread
// step(Buffer)
pub struct TradeData<'a> {
    pub gw_values: GWValues<'a>,
    pub indicators_gateway: IndicatorsGateway<'a>,
    pub signals_ready_gateway: SignalReadyGateway<'a>,
    pub signals_train_gateway: SignalTrainGateway<'a>,
    pub cell: &'a RefCell<TradeCell>,
    pub symbol: &'a str,
    s: &'a SETTINGS,
}

impl<'a> TradeData<'a> {
    pub fn new(
        buffer: &[Vec<f64>],
        s: &'a SETTINGS,
        symbol: &'a str,
    ) -> Self {
        Self {
            gw_values,
            indicators_gateway: IndicatorsGateway::new(
                &gw_values.indicators,
                &s.indications,
            ),
            signals_ready_gateway: SignalReadyGateway::new(
                &gw_values.signals_ready,
                &gw_values.indicators,
                &s.signals_ready,
                &s.indications,
            ),
            signals_train_gateway: SignalTrainGateway::new(
                &gw_values.signals_train,
                &gw_values.indicators,
                &s.signals_train,
                &s.indications,
            ),
            cell,
            symbol,
            s,
        }
    }
    pub fn to_orders(
        &self,
        buffer: &[Vec<f64>],
    ) -> Vec<Order> {
        let mut res = Vec::new();
        let indications = self.indicators_gateway.indications_series(buffer);
        let signals_ready = self
            .signals_ready_gateway
            .signals_series(&indications, buffer);
        // let signals_train = self.signals_train_gateway.signals_series(&indications, buffer);
        for market_entry in &self.s.strategy.market_entry_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                &self.cell,
                &self.symbol,
                0.,
                0.,
                &signals_ready[market_entry.as_str()],
                buffer.last().unwrap(),
                "market",
                false,
            ));
        }
        for market_exit in &self.s.strategy.market_exit_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                &self.cell,
                &self.symbol,
                0.,
                0.,
                &signals_ready[market_exit.as_str()],
                buffer.last().unwrap(),
                "market",
                true,
            ));
        }
        for limit_entry in &self.s.strategy.limit_entry_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                &self.cell,
                &self.symbol,
                indications[limit_entry.1.as_str()],
                0.,
                &signals_ready[limit_entry.0.as_str()],
                buffer.last().unwrap(),
                "limit",
                false,
            ));
        }
        for limit_exit in &self.s.strategy.limit_exit_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                &self.cell,
                &self.symbol,
                indications[limit_exit.1.as_str()],
                0.,
                &signals_ready[limit_exit.0.as_str()],
                buffer.last().unwrap(),
                "limit",
                true,
            ));
        }
        for trigger_market_entry in &self.s.strategy.trigger_market_entry_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                &self.cell,
                &self.symbol,
                0.,
                indications[trigger_market_entry.1.as_str()],
                &signals_ready[trigger_market_entry.0.as_str()],
                buffer.last().unwrap(),
                "market",
                false,
            ));
        }
        for trigger_market_exit in &self.s.strategy.trigger_market_exit_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                &self.cell,
                &self.symbol,
                0.,
                indications[trigger_market_exit.1.as_str()],
                &signals_ready[trigger_market_exit.0.as_str()],
                buffer.last().unwrap(),
                "market",
                true,
            ));
        }
        for trigger_limit_entry in &self.s.strategy.trigger_limit_entry_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                &self.cell,
                &self.symbol,
                indications[trigger_limit_entry.1.as_str()],
                indications[trigger_limit_entry.2.as_str()],
                &signals_ready[trigger_limit_entry.0.as_str()],
                buffer.last().unwrap(),
                "limit",
                false,
            ));
        }
        for trigger_limit_exit in &self.s.strategy.trigger_limit_exit_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                &self.cell,
                &self.symbol,
                indications[trigger_limit_exit.1.as_str()],
                indications[trigger_limit_exit.2.as_str()],
                &signals_ready[trigger_limit_exit.0.as_str()],
                buffer.last().unwrap(),
                "limit",
                false,
            ));
        }
        res
    }
}
