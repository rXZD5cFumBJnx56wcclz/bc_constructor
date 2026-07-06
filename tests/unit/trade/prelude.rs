pub use std::sync::LazyLock;

pub use bc_constructor::trade::structs::StepCell;
pub use bc_constructor::trade::utils_cell::*;
pub use bc_pack_orders_collectors::FUNCS_EXTRACT_ARGS as FA_O;
pub use bc_utils_lg::{
    structs::{
        signals::Signal,
        trade::{Order, Position, TradeCell},
    },
    types::maps::MAP,
};

pub use crate::unit::prelude::*;

pub static SIGNAL: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));

pub fn set_order_link_id(v: &mut Order) {
    v.order_link_id = "".to_string();
    for tp in v.tp.iter_mut() {
        set_order_link_id(tp);
    }
    for sl in v.sl.iter_mut() {
        set_order_link_id(sl);
    }
}
