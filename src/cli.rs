use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(flatten)]
    pub strategy: Option<Strategy>,
    #[command(flatten)]
    pub paths: Paths,
    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Run,
    #[command(alias("upd"))]
    Update,
    #[command(alias("bt"))]
    Backtest,
    Bench,
}

#[derive(Args)]
pub struct Strategy {
    #[arg(long, short, num_args=1..)]
    pub symbols: Option<Vec<String>>,
    #[arg(long, short)]
    pub klines_qty: Option<usize>,
    #[arg(long, short)]
    pub timeframe: Option<String>,
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
    pub leverage: Option<f64>,
    #[arg(long)]
    pub capital: Option<f64>,
    #[arg(long)]
    pub percent_of_capital: Option<f64>,
    #[arg(long)]
    pub amount_of_capital: Option<f64>,
    #[arg(long)]
    pub mode_trade: Option<String>,
    #[arg(long)]
    pub hedge_mode: Option<bool>,
    #[arg(long)]
    pub max_entry: Option<usize>,
    #[arg(long)]
    pub max_exit: Option<usize>,
    #[arg(long, num_args=1..)]
    pub symbols_black_list: Option<Vec<String>>,
    #[arg(long, num_args=1..)]
    pub coins: Option<Vec<String>>,
    #[arg(long, num_args=1..)]
    pub coins_black_list: Option<Vec<String>>,
    #[arg(long)]
    pub market_mult_of_probability_qty: Option<f64>,
    #[arg(long)]
    pub limit_mult_of_probability_qty: Option<f64>,
    #[arg(long, num_args=1..)]
    pub markets_entry_orders_signals: Option<Vec<String>>,
    #[arg(long, num_args=1..)]
    pub markets_exit_orders_signals: Option<Vec<String>>,
    #[arg(long)]
    pub work_in_real_time: Option<bool>,
}

#[derive(Args)]
pub struct Paths {
    #[arg(long, short, default_value = "./settings.json")]
    pub settings: PathBuf,
    #[arg(long, default_value = "target/bc_constructor/backtests")]
    pub backtest_path: PathBuf,
    #[arg(long, default_value = "target/bc_constructor/train_models")]
    pub train_model_path: PathBuf,
    #[arg(long)]
    pub script_backtest_path: Option<PathBuf>,
    #[arg(long)]
    pub script_stat_path: Option<PathBuf>,
    #[arg(long)]
    pub exch_data_path: Option<PathBuf>,
}
