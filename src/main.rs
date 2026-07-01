use clap::Parser;

use bc_constructor::cli::{Cli, Commands};
use bc_constructor::settings::settings_cli_sync;
use bc_utils_lg::settings::settings_from_json;

fn main() {
    let cli = Cli::parse();
    let settings = settings_cli_sync(
        settings_from_json(cli.paths.clone().settings).unwrap(),
        &cli,
    );
    let commands = cli.commands;
    if let Some(commands) = commands {
        match commands {
            Commands::Run => {}
            Commands::Backtest { save_and_use: save_and_use } => {}
            Commands::Update => {}
            Commands::Bench => {}
        }
    }
}
