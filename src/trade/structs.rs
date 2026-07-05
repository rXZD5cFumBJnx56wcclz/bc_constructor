use bc_utils_lg::structs::{settings::SETTINGS_TRADE, trade::*};

use crate::orders_collectors::OrdersCollectorsGateway;
use crate::trade::statistics::StatCollector;
use crate::trade::utils_cell::*;

pub trait StepCell {
    fn step(
        &mut self,
        // 0time 1open 2high 3low 4close 5volume 6turnover 7price_index 8price_mark
        src: &[f64],
        src_l: &[f64],
        orders: Vec<Order>,
        settings: &SETTINGS_TRADE,
        order_collector_gw: &OrdersCollectorsGateway,
        stat_collector: Option<&mut StatCollector>,
    );
}

impl StepCell for TradeCell {
    fn step(
        &mut self,
        src: &[f64],
        src_l: &[f64],
        orders: Vec<Order>,
        settings: &SETTINGS_TRADE,
        order_collector_gw: &OrdersCollectorsGateway,
        stat_collector: Option<&mut StatCollector>,
    ) {
        self.src = src.to_vec();
        self.src_l = src_l.to_vec();
        for order in orders {
            // recursive iteration is not required, as no such orders will be created
            for sl in order.sl.iter().cloned() {
                self.trigger_orders
                    .borrow_mut()
                    .insert(sl.order_link_id.clone(), sl);
            }
            for tp in order.tp.iter().cloned() {
                self.trigger_orders
                    .borrow_mut()
                    .insert(tp.order_link_id.clone(), tp);
            }
            if order.is_trigger() {
                self.trigger_orders
                    .borrow_mut()
                    .insert(order.order_link_id.clone(), order);
            } else if order.is_limit() {
                self.limit_orders
                    .borrow_mut()
                    .insert(order.order_link_id.clone(), order);
            } else {
                self.market_orders
                    .borrow_mut()
                    .insert(order.order_link_id.clone(), order);
            }
        }
        for market_order in self.market_orders.clone().borrow().values() {
            modify_positions(settings, self, market_order);
        }
        for trigger_order in self.trigger_orders.clone().borrow().values() {
            modify_positions_or_not(settings, self, trigger_order);
        }
        for limit_order in self.limit_orders.clone().borrow().values() {
            modify_positions_or_not(settings, self, limit_order);
        }
        order_collector_gw.collect_orders(&self);
        if let Some(stat_collector) = stat_collector {
            stat_collector.push(self.clone());
        }
        self.market_orders.borrow_mut().clear();
        self.trigger_orders.borrow_mut().retain(|_, v| v.is_active);
        self.limit_orders.borrow_mut().retain(|_, v| v.is_active);
        self.positions.borrow_mut().retain(|_, v| v.is_active);
    }
}

pub trait IsActive {
    fn is_active(&self) -> bool;
}

impl IsActive for Position {
    fn is_active(&self) -> bool {
        self.is_active
    }
}

impl IsActive for Order {
    fn is_active(&self) -> bool {
        self.is_active
    }
}
