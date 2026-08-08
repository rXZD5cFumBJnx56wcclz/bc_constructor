// use bc_constructor::cli::*;
// use bc_constructor::cli_processing::{check_addition_args_flags, settings_cli_sync};
// use bc_constructor::main_funcs::backtest_multi_main;
// use bc_pack_indicators::PACK as FA_I;
// use bc_pack_orders_collectors::PACK as FA_O;
// use bc_pack_signals::PACK as FA_R;
// use bc_pack_signals_train::PACK as FA_T;
// use bc_pack_symbol_filters::PACK as FA_SF;
// use bc_utils_lg::structs::settings::settings_from_json;
// use clap::Parser;
use tokio;

#[tokio::main]
async fn main() {
    // let cli = Cli::parse();
    // let s = settings_cli_sync(
    //     settings_from_json(cli.paths.clone().settings).unwrap(),
    //     &cli,
    // );
    // // fa
    // let commands = &cli.commands;
    // dbg!(&commands);
    // if let Some(commands) = commands {
    //     match commands {
    //         Commands::Run => {}
    //         Commands::Backtest(backtest_args_flags) => backtest_multi_main(
    //             &s,
    //             &cli,
    //             &FA_I(),
    //             &FA_R(),
    //             &FA_T(),
    //             &FA_O(),
    //             &FA_SF(),
    //             backtest_args_flags,
    //         )
    //         .await
    //         .unwrap(),
    //         Commands::Update => {}
    //         Commands::Bench => {}
    //     }
    // }
}
