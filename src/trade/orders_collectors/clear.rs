use std::cell::RefCell;

use bc_utils_lg::types::maps::MAP;

use crate::trade::orders_collector::OrderCollector;
use crate::trade::structs::{Order, Position};

pub struct CLEAR {}

impl OrderCollector for CLEAR {
    fn collect_orders(
        &self,
        orders: &RefCell<MAP<String, Order>>,
        positions: &RefCell<MAP<String, Position>>,
    ) {
        if positions.borrow().is_empty() {
            orders.borrow_mut().clear();
        }
    }
}
