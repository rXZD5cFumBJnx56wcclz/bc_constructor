use std::error::Error;

use bc_runtime_components::trade_runtime::TradeRuntime;
use bc_statistics::stat_collector::StatCollector;
use bc_utils_lg::structs::settings::{SETTINGS, SETTINGS_DATA_GEN};

// pub fn data_generation<'a>(
//     symbols: &[String],
//     s: &'a SETTINGS_DATA_GEN,
// ) -> Result<StatCollector<'a>, Box<dyn Error>> {
//     let trade_data = TradeRuntime::init_with(src, s, symbol, packs, stage_end)
// }
