use std::sync::LazyLock;

use bc_constructor::settings::SETTINGS_ORDER_PLACE;
use bc_constructor::trade::structs::{Order, Position, TradeCell};
use bc_constructor::trade::utils_cell::{qty_and_commission, qty_pnl};
use bc_signals::ready::ready_trait::Signal;

use bc_constructor::settings::SETTINGS_STRATEGY;

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
const SRC_EL_L2: [f64; 9] = [1.9; 9];
const SRC_EL_L1: [f64; 9] = [2.02; 9];
const SRC_EL_L: [f64; 9] = [2.124; 9];
const SRC_EL: [f64; 9] = [1.8; 9];
static SRC: LazyLock<Vec<Vec<f64>>> = LazyLock::new(|| {
    vec![SRC_EL_L2.to_vec(), SRC_EL_L1.to_vec(), SRC_EL_L.to_vec(), SRC_EL.to_vec()]
});

#[test]
fn trade_cell_step_res_1() {
    let mut cell = TradeCell::new(S.capital);
    let (qty_market, commission_market) = qty_and_commission(&S, &SIGNAL, "market", 0., 0.);
    let triggers = vec![
        Order::new(
            "symbol".to_string(),
            "buy".to_string(),
            *SIGNAL,
            0.,
            0.3,
            S.leverage,
            Default::default(),
            "market".to_string(),
            Default::default(),
            Default::default(),
            "last".to_string(),
            SRC[1][1],
            1,
            true,
            "sdlfsdkl2312".to_string(),
            "1".to_string(),
            true,
        ),
        Order::new(
            "symbol".to_string(),
            "buy".to_string(),
            *SIGNAL,
            0.,
            0.8,
            S.leverage,
            Default::default(),
            "market".to_string(),
            Default::default(),
            Default::default(),
            "last".to_string(),
            SRC[2][1],
            1,
            true,
            "sdlfsdkl22".to_string(),
            "1".to_string(),
            true,
        ),
        Order::new(
            "symbol".to_string(),
            "buy".to_string(),
            *SIGNAL,
            0.,
            1.,
            S.leverage,
            Default::default(),
            "market".to_string(),
            Default::default(),
            Default::default(),
            "last".to_string(),
            SRC[3][1],
            2,
            true,
            "wesdlfsdkl2312".to_string(),
            "1".to_string(),
            true,
        ),
    ];
    cell.step(
        &SRC[0],
        &SRC[0],
        vec![Order::new(
            "symbol".to_string(),
            "buy".to_string(),
            *SIGNAL,
            qty_market,
            0.,
            S.leverage,
            SRC[0][1],
            "market".to_string(),
            triggers[..2].to_vec(),
            triggers[1..].to_vec(),
            Default::default(),
            Default::default(),
            Default::default(),
            false,
            "sdlfsdkl2312".to_string(),
            "1".to_string(),
            true,
        )],
        &S,
        None,
    );
    let mut res = TradeCell::new(S.capital - commission_market - qty_market);
    res.push_position(Position::new(
        "symbol".to_string(),
        "buy".to_string(),
        qty_market,
        S.leverage,
        SRC[0][1],
        "1".to_string(),
        true,
    ));
    res.push_triggers_orders(triggers.clone());
    assert_eq!(cell, res);
    cell.step(&SRC[1], &SRC[0], Default::default(), &S, None);
    res.positions
        .borrow_mut()
        .entry("1".to_string())
        .and_modify(|p| {
            let qty_sub = p.qty * 0.3;
            p.qty -= qty_sub;
            res.capital -= qty_sub * S.commission_market * S.leverage;
            res.capital += qty_sub;
            res.capital += qty_pnl(&S, qty_sub, p.avg_open_price, SRC[1][4], &p.position_idx);
        });
    res.trigger_orders
        .borrow_mut()
        .remove(triggers[0].order_link_id.as_str());
    assert_eq!(cell, res);
    cell.step(&SRC[2], &SRC[1], Default::default(), &S, None);
    res.trigger_orders
        .borrow_mut()
        .remove(triggers[1].order_link_id.as_str());
    res.positions
        .borrow_mut()
        .entry("1".to_string())
        .and_modify(|p| {
            let qty_sub = p.qty * 0.8;
            p.qty -= qty_sub;
            res.capital -= qty_sub * S.commission_market * S.leverage;
            res.capital += qty_sub;
            res.capital += qty_pnl(&S, qty_sub, p.avg_open_price, SRC[2][4], &p.position_idx);
        });
    assert_eq!(cell, res);
    cell.step(&SRC[3], &SRC[2], Default::default(), &S, None);
    res.trigger_orders
        .borrow_mut()
        .remove(triggers[2].order_link_id.as_str());
    res.positions
        .borrow_mut()
        .entry("1".to_string())
        .and_modify(|p| {
            let qty_sub = p.qty * 1.;
            p.qty -= qty_sub;
            res.capital -= qty_sub * S.commission_market * S.leverage;
            res.capital += qty_sub;
            res.capital += qty_pnl(&S, qty_sub, p.avg_open_price, SRC[3][4], &p.position_idx);
        });
    res.positions.borrow_mut().remove("1");
    assert_eq!(cell, res);
}
