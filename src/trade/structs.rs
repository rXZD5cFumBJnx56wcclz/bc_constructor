use std::cell::RefCell;

use bc_signals::ready::ready_trait::Signal;
use bc_utils_lg::types::maps::MAP;

use crate::trade::utils_cell::*;
use crate::{settings::SETTINGS_STRATEGY, trade::statistics::StatCollector};

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub symbol: String,
    pub side: String,
    pub signal: Signal,
    pub qty: f64,
    pub qty_percent_of_position: f64,
    pub leverage: f64,
    pub price: f64,
    pub type_: String,
    pub tp: Vec<Order>,
    pub sl: Vec<Order>,
    pub trigger_by: String,
    pub trigger_price: f64,
    pub trigger_direction: usize,
    pub is_reduce: bool,
    pub order_link_id: String,
    pub position_idx: String,
    pub is_active: bool,
}

impl Order {
    pub fn new(
        symbol: String,
        side: String,
        signal: Signal,
        qty: f64,
        qty_percent_of_position: f64,
        leverage: f64,
        price: f64,
        type_: String,
        tp: Vec<Order>,
        sl: Vec<Order>,
        trigger_by: String,
        trigger_price: f64,
        trigger_direction: usize,
        is_reduce: bool,
        order_link_id: String,
        position_idx: String,
        is_active: bool,
    ) -> Self {
        Self {
            symbol,
            side,
            signal,
            qty,
            qty_percent_of_position,
            leverage,
            price,
            type_,
            tp,
            sl,
            trigger_by,
            trigger_price,
            trigger_direction,
            is_reduce,
            order_link_id,
            position_idx,
            is_active,
        }
    }
    pub fn is_limit(&self) -> bool {
        self.type_ == "limit"
    }
    pub fn is_market(&self) -> bool {
        self.type_ == "market"
    }
    pub fn is_trigger(&self) -> bool {
        self.trigger_price != 0.0
    }
    pub fn set_is_active(
        &mut self,
        is_active: bool,
    ) {
        self.is_active = is_active;
    }
    pub fn get_order_qty(
        &self,
        position_qty: f64,
    ) -> f64 {
        self.qty_percent_of_position * position_qty + self.qty
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub symbol: String,
    pub side: String,
    pub qty: f64,
    pub leverage: f64,
    pub avg_open_price: f64,
    pub position_idx: String,
    pub is_active: bool,
}

impl Position {
    pub fn new(
        symbol: String,
        side: String,
        qty: f64,
        leverage: f64,
        avg_open_price: f64,
        position_idx: String,
        is_active: bool,
    ) -> Self {
        Self {
            symbol,
            side,
            qty,
            leverage,
            avg_open_price,
            position_idx,
            is_active,
        }
    }
    pub fn set_qty(
        &mut self,
        qty: f64,
    ) {
        self.qty = qty;
    }
    pub fn set_avg_open_price(
        &mut self,
        avg_open_price: f64,
    ) {
        self.avg_open_price = avg_open_price;
    }
    pub fn set_is_active(
        &mut self,
        is_active: bool,
    ) {
        self.is_active = is_active;
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TradeCell {
    // reffcell
    pub capital: f64,
    // key: order_link_id
    pub trigger_orders: RefCell<MAP<String, Order>>,
    pub limit_orders: RefCell<MAP<String, Order>>,
    // key: position_idx
    pub positions: RefCell<MAP<String, Position>>,
}

impl TradeCell {
    pub fn new(capital: f64) -> Self {
        Self { capital: capital, ..Self::default() }
    }
    pub fn push_position(
        &mut self,
        position: Position,
    ) {
        self.positions
            .borrow_mut()
            .insert(position.position_idx.clone(), position);
    }
    pub fn push_trigger_order(
        &mut self,
        order: Order,
    ) {
        self.trigger_orders
            .borrow_mut()
            .insert(order.order_link_id.clone(), order);
    }
    pub fn push_limit_order(
        &mut self,
        order: Order,
    ) {
        self.limit_orders
            .borrow_mut()
            .insert(order.order_link_id.clone(), order);
    }
    pub fn push_triggers_orders<T: IntoIterator<Item = Order>>(
        &mut self,
        orders: T,
    ) {
        for order in orders {
            self.push_trigger_order(order);
        }
    }
    pub fn push_limits_orders<T: IntoIterator<Item = Order>>(
        &mut self,
        orders: T,
    ) {
        for order in orders {
            self.push_limit_order(order);
        }
    }
}

impl TradeCell {
    pub fn step(
        &mut self,
        // 0time 1open 2high 3low 4close 5volume 6turnover 7price_index 8price_mark
        src: &[f64],
        src_l: &[f64],
        orders: Vec<Order>,
        settings: &SETTINGS_STRATEGY,
        stat_collector: Option<&mut StatCollector>,
    ) {
        for order in orders {
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
                modify_positions(settings, self, &order, src[4]);
            }
        }
        for trigger_order in self.trigger_orders.clone().borrow().values() {
            modify_positions_or_not(settings, src, src_l, self, trigger_order);
        }
        for limit_order in self.limit_orders.clone().borrow().values() {
            modify_positions_or_not(settings, src, src_l, self, limit_order);
        }
        self.trigger_orders.borrow_mut().retain(|_, v| v.is_active);
        self.limit_orders.borrow_mut().retain(|_, v| v.is_active);
        self.positions.borrow_mut().retain(|_, v| v.is_active);

        if let Some(stat_collector) = stat_collector {
            stat_collector.push(self.clone());
        }
    }
}
