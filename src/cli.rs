use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Clone, Debug)]
#[command(version)]
pub struct Cli {
    #[command(flatten)]
    pub addition_args: Option<AdditionArgs>,
    #[command(flatten)]
    pub addition_flugs: Option<AdditionFlags>,
    #[command(flatten)]
    pub trade: Option<Trade>,
    #[command(flatten)]
    pub paths: Paths,
    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    Run,
    #[command(alias("upd"))]
    Update,
    #[command(alias("bt"))]
    Backtest,
    Bench,
}

#[derive(Args, Clone, Debug)]
pub struct AdditionFlags {
    #[arg(long)]
    pub save_data: bool,
    #[arg(long)]
    pub clear: bool,
}

#[derive(Args, Clone, Debug)]
pub struct AdditionArgs {}

#[derive(Args, Clone, Debug)]
pub struct Trade {
    #[arg(long)]
    pub category: Option<String>,
    #[arg(long)]
    pub account_type: Option<String>,
    #[arg(long)]
    pub klines_qty: Option<usize>,
    #[arg(long)]
    pub timeframe: Option<String>,
    #[arg(long)]
    pub leverage: Option<f64>,
    #[arg(long)]
    pub mode_trade: Option<String>,
    #[arg(long)]
    pub hedge_mode: Option<bool>,
    #[arg(long)]
    pub symbols_time_update_ms: Option<usize>,
    #[arg(long, num_args=1..)]
    pub symbols: Option<Vec<String>>,
    #[arg(long, num_args=1..)]
    pub symbols_black_list: Option<Vec<String>>,
    #[arg(long, num_args=1..)]
    pub coins: Option<Vec<String>>,
    #[arg(long, num_args=1..)]
    pub coins_black_list: Option<Vec<String>>,
    #[arg(long)]
    pub slippage_tolerance_type: Option<String>,
    #[arg(long)]
    pub time_in_force: Option<String>,
    #[arg(long)]
    pub signal_hold: Option<f64>,
    #[arg(long)]
    pub signal_short: Option<f64>,
    #[arg(long)]
    pub signal_long: Option<f64>,
    #[arg(long)]
    pub commission_market: Option<f64>,
    #[arg(long)]
    pub commission_limit: Option<f64>,
    #[arg(long)]
    pub capital: Option<f64>,
    #[arg(long)]
    pub percent_of_capital: Option<f64>,
    #[arg(long)]
    pub amount_of_capital: Option<f64>,
    #[arg(long)]
    pub max_entry: Option<usize>,
    #[arg(long)]
    pub max_exit: Option<usize>,
    #[arg(long)]
    pub market_mult_of_probability_qty: Option<f64>,
    #[arg(long)]
    pub limit_mult_of_probability_qty: Option<f64>,
    #[arg(long, num_args=1..)]
    pub market_entry_orders_signals: Option<Vec<String>>,
    #[arg(long, num_args=1..)]
    pub market_exit_orders_signals: Option<Vec<String>>,
    #[arg(long)]
    pub trigger_by: Option<String>,
    #[arg(long)]
    pub work_in_real_time: Option<bool>,
}

#[derive(Args, Clone, Debug)]
pub struct Paths {
    #[arg(long, short, default_value = "./settings.json")]
    pub settings: PathBuf,
    #[arg(long, default_value = "target/bc_constructor/backtests")]
    pub backtest: PathBuf,
    #[arg(long, default_value = "target/bc_constructor/train_models")]
    pub train_model: PathBuf,
    #[arg(long)]
    pub script_backtest: Option<PathBuf>,
    #[arg(long)]
    pub script_stat: Option<PathBuf>,
    #[arg(long)]
    pub src_data: Option<PathBuf>,
}
