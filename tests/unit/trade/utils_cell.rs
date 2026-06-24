use std::sync::LazyLock;

use bc_constructor::settings::SETTINGS_ORDER_PLACE;
use bc_constructor::trade::structs::{Order, Position, TradeCell};
use bc_signals::ready::ready_trait::Signal;

use bc_constructor::settings::SETTINGS_STRATEGY;
use bc_constructor::trade::utils_cell::*;

static S: LazyLock<SETTINGS_STRATEGY> = LazyLock::new(|| SETTINGS_STRATEGY {
    signal_hold: 0.,
    signal_short: -1.,

    signal_long: 1.,
    commission_market: 0.00055,
    commission_limit: 0.0002,
    leverage: 10.,
    capital: 100.,
    percent_of_capital: 0.01,
    order_place_settings: SETTINGS_ORDER_PLACE {
        stoploss: vec![((1., 0.), (0.5, 0.))],
        ..Default::default()
    },
    ..Default::default()
});
static SIGNAL: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));
const SRC: [f64; 9] = [2.124; 9];
const SRC_L: [f64; 9] = [2.02; 9];

#[test]
fn qty_and_commision_res_1() {
    let qty = S.capital * S.percent_of_capital;
    assert_eq!(
        (qty, qty * S.commission_market * S.leverage),
        qty_and_commission(&S, &SIGNAL, "market", 0., 0.),
    );
    assert_eq!(
        (qty, qty * S.commission_limit * S.leverage),
        qty_and_commission(&S, &SIGNAL, "limit", 0., 0.),
    );
}

#[test]
fn price_in_real_time_res_1() {
    let src = &[1., 1., 2., 2., 3.];
    assert_eq!(src[4], price_is_real_time(true, src));
    assert_eq!(src[1], price_is_real_time(false, src));
}

#[test]
fn price_with_type_res_1() {
    let src = &[1., 1., 2., 2., 3.];
    assert_eq!(src[1], price_with_type(&*S, src, "last"));
}

#[test]
fn position_idx_res_1() {
    assert_eq!("1".to_string(), position_idx(&S, &SIGNAL),);
}

#[test]
fn price_crossed_res_1() {
    assert_eq!((true, 1), price_crossed(1.3, 1.2, 1.22, 1.18),);
}

#[test]
fn qty_pnl_res_1() {
    assert_eq!(qty_pnl(&S, 10., 2., 3., "1"), 50.);
}

#[test]
fn modify_positions_res_1() {
    let mut res = TradeCell::new(S.capital);
    let (qty, commission) = qty_and_commission(&S, &SIGNAL, "market", 0., 0.);
    let price = SRC[1];
    res.push_position(Position::new(
        "".to_string(),
        "buy".to_string(),
        qty,
        S.leverage,
        price,
        "1".to_string(),
        true,
    ));
    res.capital -= qty + commission;
    let mut cell = TradeCell::new(S.capital);
    modify_positions(
        &S,
        &mut cell,
        &Order::new(
            "".to_string(),
            "buy".to_string(),
            *SIGNAL,
            qty,
            0.,
            S.leverage,
            price,
            "market".to_string(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            false,
            "1203".to_string(),
            "1".to_string(),
            true,
        ),
        SRC[4],
    );
    assert_eq!(cell, res,);
}

#[test]
fn modify_positions_or_not_res_1() {
    let mut res = TradeCell::new(S.capital);
    let (qty, commission) = qty_and_commission(&S, &SIGNAL, "market", 0., 0.);
    let (_, commission_limit) = qty_and_commission(&S, &SIGNAL, "limit", 0., 0.);
    let price = SRC[1];
    res.push_position(Position::new(
        "".to_string(),
        "buy".to_string(),
        qty * 2.,
        S.leverage,
        price,
        "1".to_string(),
        true,
    ));
    res.capital -= qty * 2. + commission + commission_limit;
    let mut cell = TradeCell::new(S.capital);
    modify_positions(
        &S,
        &mut cell,
        &Order::new(
            "".to_string(),
            "buy".to_string(),
            *SIGNAL,
            qty,
            0.,
            S.leverage,
            price,
            "market".to_string(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            false,
            "1203".to_string(),
            "1".to_string(),
            true,
        ),
        SRC[4],
    );
    modify_positions_or_not(
        &S,
        &SRC,
        &SRC_L,
        &mut cell,
        &Order::new(
            "".to_string(),
            "buy".to_string(),
            *SIGNAL,
            qty,
            0.,
            S.leverage,
            price,
            "limit".to_string(),
            Default::default(),
            Default::default(),
            "last".to_string(),
            price,
            1,
            false,
            "124241".to_string(),
            "1".to_string(),
            true,
        ),
    );
    assert_eq!(cell, res);
}
