use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use bc_exch_api_funcs::{
    bybit::{
        exch_struct::BYBIT,
        market::{src::Src, symbols::Symbols},
    },
    market::klines::Kline,
};
use bc_file_wr::file_wr::FileWR;
use bc_indicators::main_trait::Indicator;
use bc_indicators_gw::gw::get_w_max;
use bc_orders_collectors::main_trait::OrderCollector;
use bc_signals::main_trait::SignalReady;
use bc_signals::train::main_trait::SignalTrain;
use bc_symbol_filters::main_trait::SymbolFilter;
use bc_symbol_filters_gw::gw::{SymbolFiltersGateway, get_map_from_settings};
use bc_trade_state::{
    backtest::{backtest, backtest_multi},
    trade_data::AfterTradeData,
};
use bc_utils_lg::{
    structs::settings::{
        SETTINGS, SETTINGS_IND, SETTINGS_ORDER_COLLECTOR, SETTINGS_SIGNAL, SETTINGS_SYMBOL_FILTER,
    },
    types::maps::{PACK as FA, MAP},
};

use crate::{
    cli::{AdditionArgsFlags, BacktestArgsFlags, Cli},
    cli_processing::{check_addition_args_flags, check_backtest_addition_args_flags},
};

pub fn symbols(
    symbols_response: &Vec<String>,
    symbols: &Vec<String>,
    symbols_black_list: &Vec<String>,
    coins: &Vec<String>,
    coins_black_list: &Vec<String>,
) -> Vec<String> {
    let symbols_is_empty = symbols.is_empty();
    let coins_is_empty = coins.is_empty();
    let symbols_empty = |v| {
        if symbols_is_empty {
            true
        } else {
            symbols.contains(v)
        }
    };
    let coins_empty = |v: &String| {
        if coins_is_empty {
            true
        } else {
            coins.iter().any(|c| v.contains(c))
        }
    };
    symbols_response
        .iter()
        .filter(|v| {
            symbols_empty(v)
                && !symbols_black_list.contains(*v)
                && coins_empty(v)
                && !coins_black_list.iter().any(|c| v.contains(c))
        })
        .cloned()
        .collect()
}

pub fn symbol_with_backtest(
    s: &SETTINGS,
    symbol: String,
    src: Vec<Vec<f64>>,
    w_max: usize,
    fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
    fa_signals: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
    fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
    fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
    fa: &FA<SETTINGS_SYMBOL_FILTER, Box<dyn SymbolFilter>>,
    symbols_gw: &SymbolFiltersGateway,
) -> Option<String> {
    let backtest = backtest(
        symbol.clone(),
        s,
        src.clone(),
        w_max,
        fa_indicators,
        fa_signals,
        fa_signals_train,
        fa_orders_collectors,
    );
    let data = backtest.to_data().to_vec();
    let ad = AfterTradeData::new(s, &data[0], fa_indicators);
    symbols_gw.symbol_filters_added(
        &src,
        &backtest.to_ind(),
        &ad.to_stat_columns(&data[0]),
        &ad.to_stat_values(&data[0]),
        fa,
        &symbol,
    )
}

pub fn symbols_with_backtest(
    s: &SETTINGS,
    src: MAP<String, Vec<Vec<f64>>>,
    w_max: usize,
    fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
    fa_signals: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
    fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
    fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
    fa: &FA<SETTINGS_SYMBOL_FILTER, Box<dyn SymbolFilter>>,
) -> Vec<String> {
    let bind = Default::default();
    let symbol_filters =
        get_map_from_settings(s.trade.symbols_filters.as_ref().unwrap_or(&bind), fa);
    let symbols_gw = SymbolFiltersGateway::new(
        &symbol_filters,
        &s.trade.symbols_filters.as_ref().unwrap_or(&bind),
    );
    src.into_iter()
        .filter_map(|(k, v)| {
            symbol_with_backtest(
                s,
                k,
                v,
                w_max,
                fa_indicators,
                fa_signals,
                fa_signals_train,
                fa_orders_collectors,
                fa,
                &symbols_gw,
            )
        })
        .collect()
}

pub async fn init_data<'a>(
    s: &'a SETTINGS,
    fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
    fa_signals: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
    fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
    fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
    fa_symbol_filters: &FA<SETTINGS_SYMBOL_FILTER, Box<dyn SymbolFilter>>,
) -> Result<(FileWR<'a>, Box<impl Src>, Vec<String>, usize), Box<dyn Error>> {
    let file_wr = FileWR::new(&s.files_path);
    let exch = Box::new(BYBIT::new(s));
    let symbols = symbols(
        &exch
            .symbols_a("", "", "")
            .await
            .unwrap()
            .into_iter()
            .map(|v| v.symbol)
            .collect::<Vec<String>>(),
        &s.trade.symbols,
        &s.trade.symbols_black_list,
        &s.trade.coins,
        &s.trade.coins_black_list,
    );
    let w_max = get_w_max(&s.indications, fa_indicators);
    let symbols_added = symbols_with_backtest(
        s,
        exch.klines_symbols_a(&symbols, s.trade.klines_qty + w_max, 0, 0)
            .await
            .unwrap(),
        w_max,
        fa_indicators,
        fa_signals,
        fa_signals_train,
        fa_orders_collectors,
        fa_symbol_filters,
    );
    Ok((file_wr, exch, symbols_added, w_max))
}

pub async fn backtest_multi_main(
    s: &SETTINGS,
    cli: &Cli,
    fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
    fa_signals: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
    fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
    fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
    fa_symbol_filters: &FA<SETTINGS_SYMBOL_FILTER, Box<dyn SymbolFilter>>,
    backtest_args_flags: &BacktestArgsFlags,
) -> Result<(), Box<dyn Error>> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (file_wr, exch, symbols, w_max) = init_data(
        s,
        fa_indicators,
        fa_signals,
        fa_signals_train,
        fa_orders_collectors,
        fa_symbol_filters,
    )
    .await?;
    check_addition_args_flags(&cli.addition_args_flags, &file_wr)?;
    let src_symbols = if dbg!(s.files_path.src.exists()) {
        file_wr.src_symbols()?
    } else {
        exch.src_symbols_a(&symbols, s.trade.klines_qty, 0, 0)
            .await?
    };
    check_backtest_addition_args_flags(backtest_args_flags, &file_wr, &src_symbols)?;
    let b = backtest_multi(
        s,
        src_symbols,
        w_max,
        fa_indicators,
        fa_signals,
        fa_signals_train,
        fa_orders_collectors,
    );
    for (symbol, backtest) in b.into_iter() {
        let ad = AfterTradeData::new(s, &backtest.to_src(), fa_indicators);
        let data = backtest.to_data();
        let data_vec = data.to_vec();
        file_wr.backtest_write(
            &data.0,
            &ad.to_stat_columns(&data_vec[0]),
            &ad.to_stat_values(&data_vec[0]),
            &symbol,
            time,
        )?;
    }
    Ok(())
}
