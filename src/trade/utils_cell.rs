use bc_signals::ready::ready_trait::Signal;

use crate::{
    settings::SETTINGS_STRATEGY,
    trade::structs::{Order, Position, TradeCell},
};

pub fn qty_and_commission(
    s: &SETTINGS_STRATEGY,
    signal: &Signal,
    type_: &str,
    position_qty: f64,
    qty_percent_of_position: f64,
) -> (f64, f64) {
    let qty_not_mult_prob = s.capital * s.percent_of_capital
        + s.amount_of_capital
        + (position_qty * qty_percent_of_position);
    match type_ {
        "market" => {
            let qty = qty_not_mult_prob * (s.market_mult_of_probability_qty * signal.probability);
            (qty, s.commission_market * qty * s.leverage)
        }
        "limit" => {
            let qty = qty_not_mult_prob * (s.limit_mult_of_probability_qty * signal.probability);
            (qty, s.commission_limit * qty * s.leverage)
        }
        _ => (
            qty_not_mult_prob,
            s.commission_market * qty_not_mult_prob * s.leverage,
        ),
    }
}

pub fn price_is_real_time(
    is_real_time: bool,
    src: &[f64],
) -> f64 {
    if is_real_time {
        src[4]
    } else {
        src[1]
    }
}

pub fn price_with_type(
    s: &SETTINGS_STRATEGY,
    src: &[f64],
    type_: &str,
) -> f64 {
    match type_ {
        "index" => src[7],
        "mark" => src[8],
        "last" | _ => price_is_real_time(s.work_in_real_time, src),
    }
}

pub fn position_idx(
    s: &SETTINGS_STRATEGY,
    signal: &Signal,
) -> String {
    if signal.signal == s.signal_long {
        "1".to_string()
    } else if signal.signal == s.signal_short {
        "2".to_string()
    } else {
        Default::default()
    }
}

pub fn price_crossed(
    price_1: f64,
    price_2: f64,
    high_last: f64,
    low_last: f64,
) -> (bool, usize) {
    let mut crossed = false;
    let mut direction = 0;
    if price_1 >= price_2 && price_1 >= high_last {
        crossed = true;
        direction = 1;
    } else if price_1 <= price_2 && price_1 <= low_last {
        crossed = true;
        direction = 2;
    }
    (crossed, direction)
}

pub fn qty_pnl(
    leverage: f64,
    qty: f64,
    avg_open_price: f64,
    price: f64,
    position_idx: &str,
) -> f64 {
    let res = (price - avg_open_price) / avg_open_price * qty * leverage;
    if position_idx == "1" {
        res
    } else {
        -res
    }
}

pub fn modify_positions(
    s: &SETTINGS_STRATEGY,
    cell: &mut TradeCell,
    order: &Order,
    last_price: f64,
) {
    cell.positions
        .borrow_mut()
        .entry(order.position_idx.clone())
        .and_modify(|position| {
            let order_qty = order.get_order_qty(position.qty);
            let commission = order_qty
                * if order.is_market() {
                    s.commission_market
                } else {
                    s.commission_limit
                }
                * s.leverage;
            cell.capital -= commission;
            if order.is_reduce || !s.hedge_mode && order.side != position.side {
                let qty_pnl = qty_pnl(
                    s.leverage,
                    order_qty,
                    position.avg_open_price,
                    last_price,
                    &position.position_idx,
                );
                position.qty -= order_qty;
                cell.capital += order_qty;
                cell.capital += if position.position_idx == "1".to_string() {
                    qty_pnl * 1.
                } else {
                    qty_pnl * -1.
                };
                if position.qty <= 0.0 {
                    position.set_is_active(false);
                }
            } else {
                position.avg_open_price = (order.price + position.avg_open_price) / 2.0;
                position.qty += order.qty;
                cell.capital -= order.qty;
            }
        })
        .or_insert_with(|| {
            let (qty, commission) = qty_and_commission(s, &order.signal, &order.type_, 0.0, 0.0);
            cell.capital -= commission + order.qty;
            Position::new(
                order.symbol.clone(),
                order.side.clone(),
                qty,
                order.leverage,
                order.price,
                order.position_idx.clone(),
                true,
            )
        });
}

pub fn modify_positions_or_not(
    s: &SETTINGS_STRATEGY,
    src: &[f64],
    src_l: &[f64],
    cell: &mut TradeCell,
    order: &Order,
) {
    if {
        let trigger_price = price_with_type(s, src, &order.trigger_by);
        let last = price_with_type(s, src_l, &order.trigger_by);
        let (crossed, direction) = price_crossed(trigger_price, order.trigger_price, last, last);
        order.is_trigger() && crossed && direction == order.trigger_direction
    } || {
        order.is_limit()
            && price_crossed(
                price_is_real_time(s.work_in_real_time, src),
                order.price,
                src[2],
                src[3],
            )
            .0
    } {
        if order.is_limit() && order.price != order.trigger_price {
            cell.limit_orders
                .borrow_mut()
                .insert(order.order_link_id.clone(), order.clone());
        } else if order.is_market() || order.price == order.trigger_price {
            modify_positions(s, cell, &order, src[4]);
        }
        cell.trigger_orders
            .borrow_mut()
            .entry(order.order_link_id.clone())
            .and_modify(|v| v.set_is_active(false));
    }
}
