use bc_signals::ready::ready_trait::Signal;
use bc_utils_lg::{settings::SETTINGS_TRADE, types::maps::MAP};
use uuid::Uuid;

use crate::trade::structs::{Order, Position, TradeCell};

pub fn qty(
    s: &SETTINGS_TRADE,
    capital: f64,
    signal: &Signal,
    type_: &str,
    position_qty: f64,
    qty_percent_of_position: f64,
) -> f64 {
    let qty_not_mult_prob = capital * s.percent_of_capital
        + s.amount_of_capital
        + (position_qty * qty_percent_of_position);
    match type_ {
        "market" => qty_not_mult_prob * (s.market_mult_of_probability_qty * signal.probability),
        "limit" => qty_not_mult_prob * (s.limit_mult_of_probability_qty * signal.probability),
        _ => qty_not_mult_prob,
    }
}

pub fn qty_and_commission(
    s: &SETTINGS_TRADE,
    capital: f64,
    signal: &Signal,
    type_: &str,
    position_qty: f64,
    qty_percent_of_position: f64,
) -> (f64, f64) {
    let qty_not_mult_prob = capital * s.percent_of_capital
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
    s: &SETTINGS_TRADE,
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
    s: &SETTINGS_TRADE,
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

pub fn side_signal(
    s: &SETTINGS_TRADE,
    signal: f64,
) -> String {
    if signal == s.signal_long {
        "buy".to_string()
    } else if signal == s.signal_short {
        "sell".to_string()
    } else {
        Default::default()
    }
}

pub fn modify_positions(
    s: &SETTINGS_TRADE,
    cell: &mut TradeCell,
    order: &Order,
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
                    cell.src[4],
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
            let (qty, commission) =
                qty_and_commission(s, cell.capital, &order.signal, &order.type_, 0.0, 0.0);
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
    s: &SETTINGS_TRADE,
    cell: &mut TradeCell,
    order: &Order,
) {
    if {
        let trigger_price = price_with_type(&s, &cell.src, &order.trigger_by);
        let last = price_with_type(&s, &cell.src_l, &order.trigger_by);
        let (crossed, direction) = price_crossed(trigger_price, order.trigger_price, last, last);
        order.is_trigger() && crossed && direction == order.trigger_direction
    } || {
        order.is_limit()
            && price_crossed(
                price_is_real_time(s.work_in_real_time, &cell.src),
                order.price,
                cell.src[2],
                cell.src[3],
            )
            .0
    } {
        if order.is_limit() && order.price != order.trigger_price {
            cell.limit_orders
                .borrow_mut()
                .insert(order.order_link_id.clone(), order.clone());
        } else if order.is_market() || order.price == order.trigger_price {
            modify_positions(s, cell, &order);
        }
        cell.trigger_orders
            .borrow_mut()
            .entry(order.order_link_id.clone())
            .and_modify(|v| v.set_is_active(false));
    }
}

pub fn trigger_direction(
    last_price: f64,
    trigger_price: f64,
) -> usize {
    if last_price < trigger_price {
        1
    } else {
        2
    }
}

pub fn tp_sl_orders(
    tp_or_sl: &str,
    s_key: fn(&SETTINGS_TRADE) -> &Vec<(f64, f64, f64)>,
    s: &SETTINGS_TRADE,
    symbol: &str,
    side: &str,
    price_is_real_time: f64,
    signal: &Signal,
    position_idx: &str,
    trigger_direction: usize,
) -> Vec<Order> {
    s_key(s)
        .iter()
        .map(
            |(percent_of_position, amount_of_position, percent_of_entry_price)| {
                Order::new(
                    symbol.to_string(),
                    side.to_string(),
                    *signal,
                    *amount_of_position,
                    *percent_of_position,
                    s.leverage,
                    0.,
                    "market".to_string(),
                    Default::default(),
                    Default::default(),
                    "last".to_string(),
                    price_is_real_time
                        * (1.
                            + *percent_of_entry_price
                                * if position_idx == "1" {
                                    1.
                                } else {
                                    -1.
                                }
                                * if tp_or_sl == "tp" {
                                    1.
                                } else {
                                    -1.
                                }),
                    trigger_direction,
                    true,
                    Uuid::new_v4().to_string(),
                    position_idx.to_string(),
                    true,
                )
            },
        )
        .collect()
}

pub fn order_create(
    s: &SETTINGS_TRADE,
    cell: &TradeCell,
    symbol: &str,
    price_limit: f64,
    price_trigger: f64,
    signal: &Signal,
    src: &[f64],
    type_order: &str,
    is_reduce: bool,
) -> Order {
    let position_idx = position_idx(&s, signal);
    let side = side_signal(&s, signal.signal);
    let position_not_created = cell.positions.borrow().get(&position_idx).is_none();
    let price_is_real_time = price_is_real_time(s.work_in_real_time, src);
    Order::new(
        symbol.to_string(),
        side.clone(),
        *signal,
        qty(s, cell.capital, signal, type_order, 0., 0.),
        0.,
        s.leverage,
        price_limit,
        type_order.to_string(),
        if position_not_created {
            tp_sl_orders(
                "tp",
                |v| &v.takeprofit,
                s,
                symbol,
                &side,
                price_is_real_time,
                signal,
                &position_idx,
                1,
            )
        } else {
            Default::default()
        },
        if position_not_created {
            tp_sl_orders(
                "sl",
                |v| &v.stoploss,
                s,
                symbol,
                &side,
                price_is_real_time,
                signal,
                &position_idx,
                2,
            )
        } else {
            Default::default()
        },
        s.trigger_by.clone(),
        price_trigger,
        trigger_direction(price_is_real_time, price_trigger),
        is_reduce,
        Uuid::new_v4().to_string(),
        position_idx,
        true,
    )
}

pub fn orders_market_extern<'a>(
    vec: &mut Vec<Order>,
    s: &SETTINGS_TRADE,
    cell: &TradeCell,
    symbol: &str,
    signals_ready_series: &MAP<&'a str, Signal>,
    buffer: &[Vec<f64>],
) {
    for market_entry in &s.market_entry_orders_signals {
        vec.push(order_create(
            &s,
            &cell,
            &symbol,
            0.,
            0.,
            &signals_ready_series[market_entry.as_str()],
            buffer.last().unwrap(),
            "market",
            false,
        ));
    }
    for market_exit in &s.market_exit_orders_signals {
        vec.push(order_create(
            &s,
            &cell,
            &symbol,
            0.,
            0.,
            &signals_ready_series[market_exit.as_str()],
            buffer.last().unwrap(),
            "market",
            true,
        ));
    }
}

pub fn orders_limit_extern<'a>(
    vec: &mut Vec<Order>,
    s: &SETTINGS_TRADE,
    cell: &TradeCell,
    symbol: &str,
    indications_series: &MAP<&'a str, f64>,
    signals_ready_series: &MAP<&'a str, Signal>,
    buffer: &[Vec<f64>],
) {
    for limit_entry in &s.limit_entry_orders_signals {
        vec.push(order_create(
            &s,
            &cell,
            &symbol,
            indications_series[limit_entry.1.as_str()],
            0.,
            &signals_ready_series[limit_entry.0.as_str()],
            buffer.last().unwrap(),
            "limit",
            false,
        ));
    }
    for limit_exit in &s.limit_exit_orders_signals {
        vec.push(order_create(
            &s,
            &cell,
            &symbol,
            indications_series[limit_exit.1.as_str()],
            0.,
            &signals_ready_series[limit_exit.0.as_str()],
            buffer.last().unwrap(),
            "limit",
            true,
        ));
    }
}

pub fn orders_trigger_extern<'a>(
    vec: &mut Vec<Order>,
    s: &SETTINGS_TRADE,
    cell: &TradeCell,
    symbol: &str,
    indications_series: &MAP<&'a str, f64>,
    signals_ready_series: &MAP<&'a str, Signal>,
    buffer: &[Vec<f64>],
) {
    for trigger_market_entry in &s.trigger_market_entry_orders_signals {
        vec.push(order_create(
            &s,
            &cell,
            &symbol,
            0.,
            indications_series[trigger_market_entry.1.as_str()],
            &signals_ready_series[trigger_market_entry.0.as_str()],
            buffer.last().unwrap(),
            "market",
            false,
        ));
    }
    for trigger_market_exit in &s.trigger_market_exit_orders_signals {
        vec.push(order_create(
            &s,
            &cell,
            &symbol,
            0.,
            indications_series[trigger_market_exit.1.as_str()],
            &signals_ready_series[trigger_market_exit.0.as_str()],
            buffer.last().unwrap(),
            "market",
            true,
        ));
    }
    for trigger_limit_entry in &s.trigger_limit_entry_orders_signals {
        vec.push(order_create(
            &s,
            &cell,
            &symbol,
            indications_series[trigger_limit_entry.1.as_str()],
            indications_series[trigger_limit_entry.2.as_str()],
            &signals_ready_series[trigger_limit_entry.0.as_str()],
            buffer.last().unwrap(),
            "limit",
            false,
        ));
    }
    for trigger_limit_exit in &s.trigger_limit_exit_orders_signals {
        vec.push(order_create(
            &s,
            &cell,
            &symbol,
            indications_series[trigger_limit_exit.1.as_str()],
            indications_series[trigger_limit_exit.2.as_str()],
            &signals_ready_series[trigger_limit_exit.0.as_str()],
            buffer.last().unwrap(),
            "limit",
            false,
        ));
    }
}

pub fn orders_create<'a>(
    s: &SETTINGS_TRADE,
    cell: &TradeCell,
    symbol: &str,
    indications_series: &MAP<&'a str, f64>,
    signals_ready_series: &MAP<&'a str, Signal>,
    buffer: &[Vec<f64>],
) -> Vec<Order> {
    let mut res = Vec::new();
    orders_market_extern(&mut res, s, cell, symbol, signals_ready_series, buffer);
    orders_limit_extern(
        &mut res,
        s,
        cell,
        symbol,
        indications_series,
        signals_ready_series,
        buffer,
    );
    orders_trigger_extern(
        &mut res,
        s,
        cell,
        symbol,
        indications_series,
        signals_ready_series,
        buffer,
    );
    res
}
