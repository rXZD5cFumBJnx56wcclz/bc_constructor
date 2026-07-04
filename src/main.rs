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
            Commands::Backtest => backtest_multi(&s, &cli.addition_flugs, &cli.addition_args)
                .await
                .unwrap(),
            Commands::Update => {}
            Commands::Bench => {}
        }
    }
    // Ok(())
}
