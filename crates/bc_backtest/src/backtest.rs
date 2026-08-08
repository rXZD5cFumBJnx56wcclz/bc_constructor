use std::error::Error;

use bc_indicators::main_trait::*;
use bc_order_filters::main_trait::OrderFilter;
use bc_orders_collectors::main_trait::OrderCollector;
use bc_runtime_components::buffer::ToBuff;
use bc_runtime_components::trade_runtime::TradeRuntime;
use bc_signals::main_trait::*;
use bc_signals_train::main_trait::*;
use bc_statistics::stat_collector::StatCollector;
use bc_utils_lg::structs::settings::*;
use bc_utils_lg::types::maps::PACK as FA;
use bc_utils_state::main_trait::UtilState;

pub fn backtest<'a>(
    symbol: String,
    s: &'a SETTINGS,
    src: Vec<Vec<f64>>,
    w_max: usize,
    fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
    fa_signals: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
    fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
    fa_order_filters: &FA<SETTINGS_ORDER_FILTER, Box<dyn OrderFilter>>,
    fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
    fa_utils_state: &FA<SETTINGS_UTIL_STATE, Box<dyn UtilState>>,
) -> Result<StatCollector<'a>, Box<dyn Error>> {
    let mut stat_collector = StatCollector::new(symbol.clone());
    let mut buffer = src[..w_max].to_buff();
    let mut trade_runtime = TradeRuntime::new(
        &mut buffer,
        s,
        fa_indicators,
        fa_signals,
        fa_signals_train,
        fa_order_filters,
        fa_orders_collectors,
        fa_utils_state,
    );

    for series in src.into_iter().skip(w_max) {
        buffer.update(series);
        trade_runtime.as_mut().step(&mut buffer, &symbol);
        trade_runtime.as_mut().execute(&mut buffer)?;
        stat_collector.push(
            Some(trade_runtime.state.trade_state.clone()),
            Some(trade_runtime.state.indications.clone()),
            Some(trade_runtime.state.signals.clone()),
            // fix
            None,
        );
        trade_runtime.as_mut().clear();
    }
    Ok(stat_collector)
}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use pretty_assertions::assert_eq as assert_eq_pr;

//     use bc_indicators_gw::gw::get_w_max;

//     use bc_indicators::prelude::BF_INDICATOR;
//     use bc_pack_indicators::PACK as FA_I;
//     use bc_pack_order_filters::PACK as FA_OF;
//     use bc_pack_orders_collectors::PACK as FA_OC;
//     use bc_pack_signals::PACK as FA_R;
//     use bc_pack_signals_train::PACK as FA_T;
//     use bc_pack_utils_state::PACK as FA_U;

//     #[test]
//     fn backtest_res_1() {
//         assert_eq_pr!(
//             backtest(
//                 "".to_string(),
//                 &S,
//                 SRC.clone(),
//                 get_w_max(&S.indications, &FA_I()),
//                 &FA_I(),
//                 &FA_R(),
//                 &FA_T(),
//                 &FA_OF()
//                 &FA_OC()
//                 &FA_OU()
//             )
//             .cells
//             .last()
//             .unwrap()
//             .src,
//             SRC_EL.to_vec(),
//         );
//     }
// }
