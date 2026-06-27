use crate::{
    indicators::{Indicators, IndicatorsGateway},
    map::{
        indicators::FUNCS_EXTRACT_ARGS_TYPE as FA_I,
        signals_ready::FUNCS_EXTRACT_ARGS_TYPE as FA_SR,
        signals_train::FUNCS_EXTRACT_ARGS_TYPE as FA_ST,
    },
    settings::SETTINGS,
    signals_ready::{SignalReadyGateway, SignalsReady},
    signals_train::{SignalTrainGateway, SignalsTrain},
    trade::{
        structs::{Order, TradeCell},
        utils_cell::order_create,
    },
};

pub struct DataInit<'a> {
    pub indicators: Indicators<'a>,
    pub signals_ready: SignalsReady<'a>,
    pub signals_train: SignalsTrain<'a>,
}

impl<'a> DataInit<'a> {
    pub fn new(
        s: &'a SETTINGS,
        funcs_args_indicators: &FA_I,
        funcs_args_signals_ready: &FA_SR,
        funcs_args_signals_train: &FA_ST,
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

pub struct TradeDataCollector<'a> {
    pub data_initialized: &'a DataInit<'a>,
    pub indicators_gateway: IndicatorsGateway<'a>,
    pub signals_ready_gateway: SignalReadyGateway<'a>,
    pub signals_train_gateway: SignalTrainGateway<'a>,
    s: &'a SETTINGS,
}

impl<'a> TradeDataCollector<'a> {
    pub fn new(
        s: &'a SETTINGS,
        data_initialized: &'a DataInit<'a>,
    ) -> Self {
        Self {
            data_initialized,
            indicators_gateway: IndicatorsGateway::new(
                &data_initialized.indicators,
                &s.indications,
            ),
            signals_ready_gateway: SignalReadyGateway::new(
                &data_initialized.signals_ready,
                &data_initialized.indicators,
                &s.signals_ready,
                &s.indications,
            ),
            signals_train_gateway: SignalTrainGateway::new(
                &data_initialized.signals_train,
                &data_initialized.indicators,
                &s.signals_train,
                &s.indications,
            ),
            s,
        }
    }
    pub fn to_orders(
        &self,
        symbol: &str,
        cell: &TradeCell,
        buffer: &[Vec<f64>],
    ) -> Vec<Order> {
        let mut res = Vec::new();
        let indications = self.indicators_gateway.indications_series(buffer);
        let signals_ready = self
            .signals_ready_gateway
            .signals_series(&indications, buffer);
        // let signals_train = self.signals_train_gateway.signals_series(&indications, buffer);
        for market_entry in &self.s.strategy.markets_entry_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                cell,
                symbol,
                0.,
                &signals_ready[market_entry.as_str()],
                buffer.last().unwrap(),
                "market",
                Default::default(),
                Default::default(),
                Default::default(),
                false,
            ));
        }
        for market_exit in &self.s.strategy.markets_exit_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                cell,
                symbol,
                0.,
                &signals_ready[market_exit.as_str()],
                buffer.last().unwrap(),
                "market",
                Default::default(),
                Default::default(),
                Default::default(),
                true,
            ));
        }
        for limit_entry in &self.s.strategy.limits_entry_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                cell,
                symbol,
                indications[limit_entry.1.as_str()],
                &signals_ready[limit_entry.0.as_str()],
                buffer.last().unwrap(),
                "limit",
                Default::default(),
                Default::default(),
                Default::default(),
                false,
            ));
        }
        for limit_exit in &self.s.strategy.limits_exit_orders_signals {
            res.push(order_create(
                &self.s.strategy,
                cell,
                symbol,
                indications[limit_exit.1.as_str()],
                &signals_ready[limit_exit.0.as_str()],
                buffer.last().unwrap(),
                "limit",
                Default::default(),
                Default::default(),
                Default::default(),
                true,
            ));
        }
        res
    }
}
