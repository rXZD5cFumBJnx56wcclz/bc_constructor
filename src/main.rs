// use bc_trade_simulate::backtest::backtest_multi;
use tokio;

#[tokio::main]
async fn main() {
    // let cli = Cli::parse();
    // let s = settings_cli_sync(
    //     settings_from_json(cli.paths.clone().settings).unwrap(),
    //     &cli,
    // );
    // // fa
    // let commands = cli.commands;
    // dbg!(&commands);
    // if let Some(commands) = commands {
    //     match commands {
    //         Commands::Run => {}
    //         Commands::Backtest(backtest_args_flags) => {
    //             dbg!(
    //                 backtest_multi(
    //                     &s,
    //                     &FA_I(),
    //                     &FA_S(),
    //                     &FA_T(),
    //                     &FA_O(),
    //                     &backtest_args_flags
    //                 )
    //                 .await
    //                 .unwrap_err()
    //             );
    //         }
    //         Commands::Update => {}
    //         Commands::Bench => {}
    //     }
    // }
    // // Ok(())
}
