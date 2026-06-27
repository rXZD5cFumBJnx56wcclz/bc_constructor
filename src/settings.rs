#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;
use std::{error::Error, path::PathBuf};

use bc_utils_lg::types::maps::{MAP, MAP_LINK};
use serde_json5::from_reader;

use crate::cli::Strategy;

pub fn settings_from_json(dir: PathBuf) -> Result<SETTINGS, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(dir)?);
    from_reader(&mut reader).map_err(|e| Box::new(e) as Box<dyn Error>)
}

pub fn settings_modify(
    mut settings: SETTINGS,
    modify: Option<Strategy>,
) -> SETTINGS {
    if let Some(modify) = modify {
        settings.strategy.symbols = modify.symbols.unwrap_or(settings.strategy.symbols);
        settings.strategy.klines_qty = modify.klines_qty.unwrap_or(settings.strategy.klines_qty);
        settings.strategy.timeframe = modify.timeframe.unwrap_or(settings.strategy.timeframe);
        settings.strategy.signal_hold = modify.signal_hold.unwrap_or(settings.strategy.signal_hold);
        settings.strategy.signal_short = modify
            .signal_short
            .unwrap_or(settings.strategy.signal_short);
        settings.strategy.signal_long = modify.signal_long.unwrap_or(settings.strategy.signal_long);
        settings.strategy.commission_market = modify
            .commission_market
            .unwrap_or(settings.strategy.commission_market);
        settings.strategy.commission_limit = modify
            .commission_limit
            .unwrap_or(settings.strategy.commission_limit);
        settings.strategy.leverage = modify.leverage.unwrap_or(settings.strategy.leverage);
        settings.strategy.capital = modify.capital.unwrap_or(settings.strategy.capital);
        settings.strategy.percent_of_capital = modify
            .percent_of_capital
            .unwrap_or(settings.strategy.percent_of_capital);
        settings.strategy.amount_of_capital = modify
            .amount_of_capital
            .unwrap_or(settings.strategy.amount_of_capital);
        settings.strategy.mode_trade = modify.mode_trade.unwrap_or(settings.strategy.mode_trade);
        settings.strategy.hedge_mode = modify.hedge_mode.unwrap_or(settings.strategy.hedge_mode);
        settings.strategy.max_entry = modify.max_entry.unwrap_or(settings.strategy.max_entry);
        settings.strategy.max_exit = modify.max_exit.unwrap_or(settings.strategy.max_exit);
        settings.strategy.symbols_black_list = modify
            .symbols_black_list
            .unwrap_or(settings.strategy.symbols_black_list);
        settings.strategy.coins = modify.coins.unwrap_or(settings.strategy.coins);
        settings.strategy.coins_black_list = modify
            .coins_black_list
            .unwrap_or(settings.strategy.coins_black_list);
        settings.strategy.market_mult_of_probability_qty = modify
            .market_mult_of_probability_qty
            .unwrap_or(settings.strategy.market_mult_of_probability_qty);
        settings.strategy.limit_mult_of_probability_qty = modify
            .limit_mult_of_probability_qty
            .unwrap_or(settings.strategy.limit_mult_of_probability_qty);
        settings.strategy.markets_entry_orders_signals = modify
            .markets_entry_orders_signals
            .unwrap_or(settings.strategy.markets_entry_orders_signals);
        settings.strategy.markets_exit_orders_signals = modify
            .markets_exit_orders_signals
            .unwrap_or(settings.strategy.markets_exit_orders_signals);
        settings.strategy.work_in_real_time = modify
            .work_in_real_time
            .unwrap_or(settings.strategy.work_in_real_time);
    }
    settings
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SETTINGS_USED_SRC {
    pub index: usize,
    pub sub_from_last_i: usize,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SETTINGS_IND {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_SRC>,
    pub used_ind: Vec<String>,
    pub order_used: Vec<usize>,
}
pub type SETTINGS_INDS = MAP_LINK<String, SETTINGS_IND>;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SETTINGS_SIGNAL {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_SRC>,
    pub used_ind: Vec<String>,
    pub used_signals: Vec<String>,
    pub order_used_src: Vec<usize>,
    pub order_used_signals: Vec<usize>,
}
pub type SETTINGS_SIGNALS = MAP_LINK<String, SETTINGS_SIGNAL>;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SETTINGS_EXCH {
    pub url: String,
    pub key: String,
    pub secret: String,
    pub exchange: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SETTINGS_MSG {
    pub key: String,
    pub chat: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SETTINGS_ORDER_COLLECTOR {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    // (1: key, 2: key_ind)
    pub used_signals_ready: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct SETTINGS_ORDER_PLACE {
    // 1: percent_of_position, 2: amount_of_position, 3: percent_of_entry_price
    pub stoploss: Vec<(f64, f64, f64)>,
    pub takeprofit: Vec<(f64, f64, f64)>,
    // not work
    pub slippage_tolerance_type: String,
    // not work
    pub slippage_tolerance: (f64, f64),
    pub time_in_force: String,
}

impl Default for SETTINGS_ORDER_PLACE {
    fn default() -> Self {
        Self {
            stoploss: Default::default(),
            takeprofit: Default::default(),
            slippage_tolerance_type: "percent".to_string(),
            slippage_tolerance: Default::default(),
            time_in_force: "GTC".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct SETTINGS_STRATEGY {
    pub klines_qty: usize,
    pub timeframe: String,
    pub signal_hold: f64,
    pub signal_short: f64,
    pub signal_long: f64,
    pub commission_market: f64,
    pub commission_limit: f64,
    pub leverage: f64,
    pub capital: f64,
    pub percent_of_capital: f64,
    pub amount_of_capital: f64,
    // not work
    pub mode_trade: String,
    // not work
    pub hedge_mode: bool,
    // not work
    pub max_entry: usize,
    // not work
    pub max_exit: usize,
    pub symbols: Vec<String>,
    pub symbols_black_list: Vec<String>,
    pub coins: Vec<String>,
    pub coins_black_list: Vec<String>,
    pub market_mult_of_probability_qty: f64,
    pub limit_mult_of_probability_qty: f64,
    pub markets_entry_orders_signals: Vec<String>,
    pub markets_exit_orders_signals: Vec<String>,
    // (1: signal, 2: key_ind_for_price)
    pub limits_entry_orders_signals: Vec<(String, String)>,
    pub limits_exit_orders_signals: Vec<(String, String)>,
    pub triggers_market_entry_orders_signals: Vec<(String, String)>,
    pub triggers_market_exit_orders_signals: Vec<(String, String)>,
    pub triggers_limit_entry_orders_signals: Vec<(String, String)>,
    pub triggers_limit_exit_orders_signals: Vec<(String, String)>,
    pub order_collectors: Vec<SETTINGS_ORDER_COLLECTOR>,
    pub order_place_settings: SETTINGS_ORDER_PLACE,
    // not work
    pub work_in_real_time: bool,
}

impl Default for SETTINGS_STRATEGY {
    fn default() -> Self {
        Self {
            klines_qty: 50_000,
            timeframe: "1".to_string(),
            signal_hold: 0.,
            signal_short: -1.,
            signal_long: 1.,
            commission_market: 0.001,
            commission_limit: 0.001,
            leverage: 1.,
            capital: 1000.,
            percent_of_capital: 0.01,
            amount_of_capital: 0.,
            mode_trade: "isolated".to_string(),
            hedge_mode: true,
            max_entry: usize::MAX,
            max_exit: usize::MAX,
            symbols: Default::default(),
            symbols_black_list: Default::default(),
            coins: Default::default(),
            coins_black_list: Default::default(),
            market_mult_of_probability_qty: 1.,
            limit_mult_of_probability_qty: 1.,
            markets_entry_orders_signals: Default::default(),
            markets_exit_orders_signals: Default::default(),
            limits_entry_orders_signals: Default::default(),
            limits_exit_orders_signals: Default::default(),
            triggers_market_entry_orders_signals: Default::default(),
            triggers_market_exit_orders_signals: Default::default(),
            triggers_limit_entry_orders_signals: Default::default(),
            triggers_limit_exit_orders_signals: Default::default(),
            order_collectors: vec![SETTINGS_ORDER_COLLECTOR {
                key: "clear".to_string(),
                ..Default::default()
            }],
            order_place_settings: Default::default(),
            work_in_real_time: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct SETTINGS_FILES_PATH {
    pub script_backtest: String,
    pub script_stat: String,
    pub backtest: String,
    pub exch_data: String,
    pub train_model: String,
}

impl Default for SETTINGS_FILES_PATH {
    fn default() -> Self {
        Self {
            script_backtest: Default::default(),
            script_stat: Default::default(),
            // /23_00_24_24_06_2026/report.html
            // /23_00_24_24_06_2026/SUIUSDT/data.dat
            // /23_00_24_24_06_2026/SUIUSDT/data.dat
            // /23_00_24_24_06_2026/SUIUSDT/stat_value.dat
            // /23_00_24_24_06_2026/SUIUSDT/stat_columns.dat
            // /23_00_24_24_06_2026/SUIUSDT/script_data.plt
            // /23_00_24_24_06_2026/SUIUSDT/script_stat.plt
            // /23_00_24_24_06_2026/SUIUSDT/backtest.svg
            // /23_00_24_24_06_2026/SUIUSDT/capital.svg
            // /23_00_24_24_06_2026/SUIUSDT/stat.svg
            backtest: "target/bc_constructor/backtests".to_string(),
            exch_data: Default::default(),
            train_model: "target/bc_constructor/train_models".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SETTINGS {
    pub exch: SETTINGS_EXCH,
    pub indications: SETTINGS_INDS,
    pub signals_train: SETTINGS_SIGNALS,
    pub signals_ready: SETTINGS_SIGNALS,
    pub strategy: SETTINGS_STRATEGY,
    pub files_path: SETTINGS_FILES_PATH,
    pub indications_stat_value: SETTINGS_INDS,
    pub indications_stat_values: SETTINGS_INDS,
}
