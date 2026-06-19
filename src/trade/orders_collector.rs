use std::{any::Any, cell::RefCell};

use bc_utils_lg::types::maps::MAP;

use crate::trade::structs::{Order, Position};

pub trait OrderCollector: Any {
    fn collect_orders(
        &self,
        orders: &RefCell<MAP<String, Order>>,
        positions: &RefCell<MAP<String, Position>>,
    );
}
