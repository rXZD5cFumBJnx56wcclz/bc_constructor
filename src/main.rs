use bc_pack_indicators::FUNCS_EXTRACT_ARGS as FA_I;
use bc_pack_orders_collectors::FUNCS_EXTRACT_ARGS as FA_O;
use bc_pack_signals_ready::FUNCS_EXTRACT_ARGS as FA_S;
use bc_pack_signals_train::FUNCS_EXTRACT_ARGS as FA_T;
use bc_utils_lg::structs::settings::settings_from_json;
use clap::Parser;
use tokio;

use bc_constructor::backtest::*;
use bc_constructor::cli::{Cli, Commands};
use bc_constructor::settings::settings_cli_sync;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let s = settings_cli_sync(
        settings_from_json(cli.paths.clone().settings).unwrap(),
        &cli,
    );
    // fa
    let commands = cli.commands;
    dbg!(&commands);
    if let Some(commands) = commands {
        match commands {
            Commands::Run => {}
            Commands::Backtest => backtest_multi(
                &s,
                &FA_I(),
                &FA_S(),
                &FA_T(),
                &FA_O(),
                &cli.addition_flugs,
                &cli.addition_args,
            )
            .await
            .unwrap(),
            Commands::Update => {}
            Commands::Bench => {}
        }
    }
    // Ok(())
}
