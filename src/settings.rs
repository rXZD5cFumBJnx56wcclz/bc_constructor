#![allow(non_camel_case_types)]

use std::error::Error;

use bc_utils_lg::structs::settings::SETTINGS;

use crate::{
    cli::{AdditionArgs, AdditionFlags, Cli},
    file_wr::FileWR,
};

pub fn settings_cli_sync(
    mut settings: SETTINGS,
    cli: &Cli,
) -> SETTINGS {
    let paths = &cli.paths;
    let trade = &cli.trade;
    if let Some(trade) = trade {
        settings.trade.category = trade.category.clone().unwrap_or(settings.trade.category);
        settings.trade.account_type = trade
            .account_type
            .clone()
            .unwrap_or(settings.trade.account_type);
        settings.trade.klines_qty = trade.klines_qty.unwrap_or(settings.trade.klines_qty);
        settings.trade.timeframe = trade.timeframe.clone().unwrap_or(settings.trade.timeframe);
        settings.trade.leverage = trade.leverage.unwrap_or(settings.trade.leverage);
        settings.trade.mode_trade = trade
            .mode_trade
            .clone()
            .unwrap_or(settings.trade.mode_trade);
        settings.trade.hedge_mode = trade.hedge_mode.unwrap_or(settings.trade.hedge_mode);
        settings.trade.symbols_time_update_ms = trade
            .symbols_time_update_ms
            .clone()
            .unwrap_or(settings.trade.symbols_time_update_ms);
        settings.trade.symbols = trade.symbols.clone().unwrap_or(settings.trade.symbols);
        settings.trade.symbols_black_list = trade
            .symbols_black_list
            .clone()
            .unwrap_or(settings.trade.symbols_black_list);
        settings.trade.coins = trade.coins.clone().unwrap_or(settings.trade.coins);
        settings.trade.coins_black_list = trade
            .coins_black_list
            .clone()
            .unwrap_or(settings.trade.coins_black_list);
        settings.trade.slippage_tolerance_type = trade
            .slippage_tolerance_type
            .clone()
            .unwrap_or(settings.trade.slippage_tolerance_type);
        settings.trade.time_in_force = trade
            .time_in_force
            .clone()
            .unwrap_or(settings.trade.time_in_force);
        settings.trade.signal_hold = trade.signal_hold.unwrap_or(settings.trade.signal_hold);
        settings.trade.signal_short = trade.signal_short.unwrap_or(settings.trade.signal_short);
        settings.trade.signal_long = trade.signal_long.unwrap_or(settings.trade.signal_long);
        settings.trade.commission_market = trade
            .commission_market
            .unwrap_or(settings.trade.commission_market);
        settings.trade.commission_limit = trade
            .commission_limit
            .unwrap_or(settings.trade.commission_limit);
        settings.trade.capital = trade.capital.unwrap_or(settings.trade.capital);
        settings.trade.percent_of_capital = trade
            .percent_of_capital
            .unwrap_or(settings.trade.percent_of_capital);
        settings.trade.amount_of_capital = trade
            .amount_of_capital
            .unwrap_or(settings.trade.amount_of_capital);
        settings.trade.max_entry = trade.max_entry.unwrap_or(settings.trade.max_entry);
        settings.trade.max_exit = trade.max_exit.unwrap_or(settings.trade.max_exit);
        settings.trade.market_mult_of_probability_qty = trade
            .market_mult_of_probability_qty
            .unwrap_or(settings.trade.market_mult_of_probability_qty);
        settings.trade.limit_mult_of_probability_qty = trade
            .limit_mult_of_probability_qty
            .unwrap_or(settings.trade.limit_mult_of_probability_qty);
        settings.trade.market_entry_orders_signals = trade
            .market_entry_orders_signals
            .clone()
            .unwrap_or(settings.trade.market_entry_orders_signals);
        settings.trade.market_exit_orders_signals = trade
            .market_exit_orders_signals
            .clone()
            .unwrap_or(settings.trade.market_exit_orders_signals);
        settings.trade.trigger_by = trade
            .trigger_by
            .clone()
            .unwrap_or(settings.trade.trigger_by);
        settings.trade.work_in_real_time = trade
            .work_in_real_time
            .unwrap_or(settings.trade.work_in_real_time);
    }
    settings.files_path.script_backtest = paths
        .script_backtest
        .clone()
        .unwrap_or(settings.files_path.script_backtest);
    settings.files_path.script_stat = paths
        .script_stat
        .clone()
        .unwrap_or(settings.files_path.script_stat);
    settings.files_path.backtest = paths.backtest.clone();
    settings.files_path.src_data = paths
        .src_data
        .clone()
        .unwrap_or(settings.files_path.src_data);
    settings.files_path.train_model = paths.train_model.clone();
    settings
}

pub fn check_addition_flags(
    addition_flags: &Option<AdditionFlags>,
    src: &Vec<Vec<f64>>,
    file_wr: &FileWR,
) -> Result<(), Box<dyn Error>> {
    if let Some(addition_flags) = addition_flags {
        if addition_flags.save_data {
            file_wr.src_write(&src)?;
        }
        if addition_flags.clear {
            file_wr.backtests_del();
        }
    }
    Ok(())
}

pub fn check_addition_args(_addition_args: &Option<AdditionArgs>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
