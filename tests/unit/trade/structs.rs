use bc_constructor::orders_collectors::{OrdersCollectors, OrdersCollectorsGateway};

use crate::unit::trade::prelude::*;

#[test]
fn trade_cell_step_res_1() {
    let mut cell = TradeCell::new(S.capital, SRC_EL_L2.to_vec(), SRC_EL_L3.to_vec());
    let order_collectors = OrdersCollectors::new(&S.order_collectors, &FA_O());
    let orders_collectors_gw = OrdersCollectorsGateway::new(&order_collectors);
    let (qty_market, commission_market) =
        qty_and_commission(&S, cell.capital, &SIGNAL, "market", 0., 0.);
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
            Some("last".to_string()),
            Some(SRC[2][1]),
            Some(1),
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
            Some("last".to_string()),
            Some(SRC[3][1]),
            Some(1),
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
            Some("last".to_string()),
            Some(SRC[4][1]),
            Some(2),
            true,
            "wesdlfsdkl2312".to_string(),
            "1".to_string(),
            true,
        ),
    ];
    cell.step(
        &SRC[1],
        &SRC[0],
        vec![Order::new(
            "symbol".to_string(),
            "buy".to_string(),
            *SIGNAL,
            qty_market,
            0.,
            S.leverage,
            Some(SRC[1][1]),
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
        &orders_collectors_gw,
        None,
    );
    let mut res = TradeCell::new(
        S.capital - commission_market - qty_market,
        SRC_EL_L2.to_vec(),
        SRC_EL_L3.to_vec(),
    );
    res.push_position(Position::new(
        "symbol".to_string(),
        "buy".to_string(),
        qty_market,
        S.leverage,
        SRC[1][1],
        "1".to_string(),
        true,
    ));
    res.push_triggers_orders(triggers.clone());
    assert_eq_pr!(cell, res);
    cell.step(
        &SRC[2],
        &SRC[1],
        Default::default(),
        &S,
        &orders_collectors_gw,
        None,
    );
    res.positions
        .borrow_mut()
        .entry("1".to_string())
        .and_modify(|p| {
            let qty_sub = p.qty * 0.3;
            p.qty -= qty_sub;
            res.capital -= qty_sub * S.commission_market * S.leverage;
            res.capital += qty_sub;
            res.capital += qty_pnl(
                S.leverage,
                qty_sub,
                p.avg_open_price,
                SRC[2][4],
                &p.position_idx,
            );
        });
    res.trigger_orders
        .borrow_mut()
        .remove(triggers[0].order_link_id.as_str());
    res.src = SRC[2].to_vec();
    res.src_l = SRC[1].to_vec();
    assert_eq_pr!(cell, res);
    cell.step(
        &SRC[3],
        &SRC[2],
        Default::default(),
        &S,
        &orders_collectors_gw,
        None,
    );
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
            res.capital += qty_pnl(
                S.leverage,
                qty_sub,
                p.avg_open_price,
                SRC[3][4],
                &p.position_idx,
            );
        });
    res.src = SRC[3].to_vec();
    res.src_l = SRC[2].to_vec();
    assert_eq_pr!(cell, res);
    cell.step(
        &SRC[4],
        &SRC[3],
        Default::default(),
        &S,
        &orders_collectors_gw,
        None,
    );
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
            res.capital += qty_pnl(
                S.leverage,
                qty_sub,
                p.avg_open_price,
                SRC[4][4],
                &p.position_idx,
            );
        });
    res.positions.borrow_mut().remove("1");
    res.src = SRC[4].to_vec();
    res.src_l = SRC[3].to_vec();
    assert_eq_pr!(cell, res);
}
