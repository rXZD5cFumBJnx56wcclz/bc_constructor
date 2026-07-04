pub use std::sync::LazyLock;

pub use bc_constructor::trade::structs::StepCell;
pub use bc_constructor::trade::utils_cell::*;
pub use bc_pack_orders_collectors::FUNCS_EXTRACT_ARGS as FA_O;
pub use bc_utils_lg::{
    structs::{
        settings::{SETTINGS_ORDER_COLLECTOR, SETTINGS_TRADE},
        signals::Signal,
        trade::{Order, Position, TradeCell},
    },
    types::maps::MAP,
};
pub use pretty_assertions::assert_eq as assert_eq_pr;

pub static S: LazyLock<SETTINGS_TRADE> = LazyLock::new(|| SETTINGS_TRADE {
    signal_hold: 0.,
    signal_short: -1.,
    signal_long: 1.,
    commission_market: 0.00055,
    commission_limit: 0.0002,
    leverage: 10.,
    capital: 100.,
    percent_of_capital: 0.01,
    stoploss: vec![(1., 0., 0.5)],
    order_collectors: vec![SETTINGS_ORDER_COLLECTOR {
        key: "clear".to_string(),
        ..Default::default()
    }],
    market_entry_orders_signals: vec!["th_1".to_string()],
    ..Default::default()
});

pub static SIGNAL: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
pub const SRC_EL_L3: [f64; 9] = [1.91; 9];
pub const SRC_EL_L2: [f64; 9] = [1.9; 9];
pub const SRC_EL_L1: [f64; 9] = [2.02; 9];
pub const SRC_EL_L: [f64; 9] = [2.124; 9];
pub const SRC_EL: [f64; 9] = [1.8; 9];
pub static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    vec![
        SRC_EL_L3.to_vec(),
        SRC_EL_L2.to_vec(),
        SRC_EL_L1.to_vec(),
        SRC_EL_L.to_vec(),
        SRC_EL.to_vec(),
    ]
});

pub fn set_order_link_id(v: &mut Order) {
    v.order_link_id = "".to_string();
    for tp in v.tp.iter_mut() {
        set_order_link_id(tp);
    }
    for sl in v.sl.iter_mut() {
        set_order_link_id(sl);
    }
}
