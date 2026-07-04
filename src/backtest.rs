use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use bc_exch_api_funcs::bybit::{exch_struct::BYBIT, market::src::Src};
use bc_indicators::indicator_traits::Indicator;
use bc_orders_collectors::main_trait::OrderCollector;
use bc_pack_indicators::FUNCS_EXTRACT_ARGS;
use bc_signals::ready::ready_trait::SignalReady;
use bc_signals::train::train_trait::SignalTrain;
use bc_utils::other::transpose;
use bc_utils_lg::{
    structs::settings::{SETTINGS, SETTINGS_IND, SETTINGS_ORDER_COLLECTOR, SETTINGS_SIGNAL},
    types::maps::FUNCS_EXTRACT_ARGS_TYPE as FA,
};

use crate::{
    buffer::Buffer,
    cli::{AdditionArgs, AdditionFlags},
    file_wr::FileWR,
    indicators::get_w_max,
    settings::{check_addition_args, check_addition_flags},
    trade::{
        statistics::StatCollector,
        trade_data::{AfterTradeData, TradeData},
    },
};

pub async fn backtest(
    symbol: &str,
    s: &SETTINGS,
    fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
    fa_signals_ready: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
    fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
    fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
    addition_flags: &Option<AdditionFlags>,
    addition_args: &Option<AdditionArgs>,
    time: u64,
) -> Result<(), Box<dyn Error>> {
    let exch = BYBIT::new(s);
    let file_wr = FileWR::new(&s.files_path);
    let mut stat_collector = StatCollector::new(symbol.to_string(), &s.trade);
    let w_max = get_w_max(&s.indications, &FUNCS_EXTRACT_ARGS());
    let src = file_wr.src_or(exch.src_a(symbol, s.trade.klines_qty + w_max, 0, 0).await?);
    check_addition_flags(addition_flags, &src, &file_wr)?;
    check_addition_args(addition_args)?;
    let buffer = Buffer::new(src);
    let trade_data = TradeData::new(
        transpose(buffer[..w_max].to_vec()).as_slice(),
        s,
        symbol,
        fa_indicators,
        fa_signals_ready,
        fa_signals_train,
        fa_orders_collectors,
    );
    for i in w_max..buffer.len() {
        trade_data
            .as_ref()
            .update(&buffer[i - w_max..i], Some(&mut stat_collector));
    }
    let stat_collector_data = stat_collector.to_data();
    let stat_collector_data_vec = stat_collector_data.to_vec();
    let after_trade_data = AfterTradeData::new(s, &stat_collector_data_vec[0], fa_indicators);
    file_wr.backtest_write(
        &stat_collector_data,
        &after_trade_data.to_stat_columns(&stat_collector_data_vec[0]),
        &after_trade_data.to_stat_values(&stat_collector_data_vec[0]),
        symbol,
        time,
    )?;
    dbg!(stat_collector.cells.last().unwrap());
    Ok(())
}

pub async fn backtest_multi(
    s: &SETTINGS,
    fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
    fa_signals_ready: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
    fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
    fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
    addition_flags: &Option<AdditionFlags>,
    addition_args: &Option<AdditionArgs>,
) -> Result<(), Box<dyn Error>> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    for symbol in if s.trade.symbols_filter.is_none() {
        s.trade
            .symbols
            .iter()
            .filter(|v| {
                !s.trade.symbols_black_list.contains(v)
                    && !s.trade.coins_black_list.iter().any(|v2| v.contains(v2))
                    && if !s.trade.coins.is_empty() {
                        s.trade.coins.iter().any(|v3| v.contains(v3))
                    } else {
                        true
                    }
            })
            .collect::<Vec<&String>>()
    } else {
        Default::default()
    }
    .iter()
    {
        dbg!(symbol);
        backtest(
            symbol,
            s,
            fa_indicators,
            fa_signals_ready,
            fa_signals_train,
            fa_orders_collectors,
            addition_flags,
            addition_args,
            time,
        )
        .await?;
    }
    Ok(())
}
