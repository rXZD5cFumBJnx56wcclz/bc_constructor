use clap::Parser;

use bc_constructor::cli::{Cli, Commands};
use bc_constructor::settings::{settings_from_json, settings_modify};

fn main() {
    let cli = Cli::parse();
    let _settings = settings_modify(
        settings_from_json(cli.paths.settings).unwrap(),
        cli.strategy,
    );
    let commands = cli.commands;
    if let Some(commands) = commands {
        match commands {
            Commands::Run => {}
            Commands::Backtest => {}
            Commands::Update => {}
            Commands::Bench => {}
        }
    }
}
