use bc_orders_collectors::main_trait::OrderCollector;
use bc_utils_lg::structs::trade::TradeCell;
use bc_utils_lg::{
    structs::settings::{SETTINGS_ORDER_COLLECTOR, SETTINGS_ORDER_COLLECTORS},
    types::maps::FUNCS_EXTRACT_ARGS_TYPE,
};

#[derive(Default)]
pub struct OrdersCollectors {
    pub orders_collectors: Vec<Box<dyn OrderCollector>>,
}

impl OrdersCollectors {
    pub fn new(
        s: &SETTINGS_ORDER_COLLECTORS,
        fa: &FUNCS_EXTRACT_ARGS_TYPE<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
    ) -> Self {
        Self {
            orders_collectors: s
                .iter()
                .map(|setting| fa[setting.key.as_str()](setting))
                .collect(),
        }
    }
}

pub struct OrdersCollectorsGateway {
    pub orders_collectors: *const OrdersCollectors,
}

impl<'a> OrdersCollectorsGateway {
    pub fn new(orders_collectors: *const OrdersCollectors) -> Self {
        Self { orders_collectors }
    }
    pub fn collect_orders(
        &self,
        cell: &TradeCell,
    ) {
        for order_collector in &unsafe { &*self.orders_collectors }.orders_collectors {
            order_collector.collect_orders(cell);
        }
    }
}
