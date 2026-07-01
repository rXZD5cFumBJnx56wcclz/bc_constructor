use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use bc_exch_api_funcs::bybit::exch_struct::BYBIT;
use bc_exch_api_funcs::bybit::market::src::Src;
use bc_utils_lg::settings::SETTINGS;
use bc_pack_indicators::FUNCS_EXTRACT_ARGS;

use crate::cli::{AdditionArgs, AdditionFlags};
use crate::trade::statistics::StatCollector;
use crate::{buffer::Buffer, file_wr::FileWR, trade::trade_data_collector::TradeData};
use crate::indicators::get_w_max;

pub fn check_addition_flags(
    addition_flags: &Option<AdditionFlags>,
    src: &Vec<Vec<f64>>,
    file_wr: &FileWR,
) -> Result<(), Box<dyn Error>> {
    if let Some(addition_flags) = addition_flags {
        if addition_flags.save_data {
            file_wr.src_write(&src)?;
        }
    }
    Ok(())
}

pub fn check_addition_args(_addition_args: &Option<AdditionArgs>) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub async fn backtest(
    symbol: &str,
    s: &SETTINGS,
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
    // fix raw pointers
    let trade_data = TradeData::new(Buffer(buffer[..w_max].to_vec()).transpose().as_slice(), s, symbol, );
    for i in w_max..buffer.len() {
        trade_data.as_ref().update(&buffer[i - w_max..i], Some(&mut stat_collector));
    }
    file_wr.backtest_write(&stat_collector.to_data(), symbol, time)?;
    Ok(())
}

pub async fn backtest_multi(
    s: &SETTINGS,
    addition_flags: &Option<AdditionFlags>,
    addition_args: &Option<AdditionArgs>,
) -> Result<(), Box<dyn Error>> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let symbols = s.trade.symbols.iter().filter(|v| {
        !s.trade.symbols_black_list.contains(v)
        && !s.trade.coins_black_list.iter().any(|v2| v.contains(v2))
        && if !s.trade.coins.is_empty() {s.trade.coins.iter().any(|v3| v.contains(v3))} else {true}
    }).collect::<Vec<&String>>();
    if s.trade.symbols_filter.is_none() {
        dbg!(&symbols);
        for symbol in symbols {
            dbg!(symbol);
            backtest(symbol, s, addition_flags, addition_args, time).await?;
        }
    }
    Ok(())
}
