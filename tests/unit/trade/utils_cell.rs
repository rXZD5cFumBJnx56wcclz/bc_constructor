use crate::unit::trade::prelude::*;

#[test]
fn qty_and_commision_res_1() {
    let qty = S.capital * S.percent_of_capital;
    assert_eq_pr!(
        (qty, qty * S.commission_market * S.leverage),
        qty_and_commission(&S, S.capital, &SIGNAL, "market", 0., 0.),
    );
    assert_eq_pr!(
        (qty, qty * S.commission_limit * S.leverage),
        qty_and_commission(&S, S.capital, &SIGNAL, "limit", 0., 0.),
    );
}

#[test]
fn price_in_real_time_res_1() {
    let src = &[1., 1., 2., 2., 3.];
    assert_eq_pr!(src[4], price_is_real_time(true, src));
    assert_eq_pr!(src[1], price_is_real_time(false, src));
}

#[test]
fn price_with_type_res_1() {
    let src = &[1., 1., 2., 2., 3.];
    assert_eq_pr!(src[1], price_with_type(&*S, src, "last"));
}

#[test]
fn position_idx_res_1() {
    assert_eq_pr!("1".to_string(), position_idx(&S, &SIGNAL),);
}

#[test]
fn price_crossed_res_1() {
    assert_eq_pr!((true, 1), price_crossed(1.3, 1.2, 1.22, 1.18),);
}

#[test]
fn qty_pnl_res_1() {
    assert_eq_pr!(qty_pnl(S.leverage, 10., 2., 3., "1"), 50.);
}

#[test]
fn modify_positions_res_1() {
    let price = SRC_EL[1];
    let mut res = TradeCell::new(S.capital, SRC_EL.to_vec(), SRC_EL_L.to_vec());
    let (qty, commission) = qty_and_commission(&S, res.capital, &SIGNAL, "market", 0., 0.);
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
    let mut cell = TradeCell::new(S.capital, SRC_EL.to_vec(), SRC_EL_L.to_vec());
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
            None,
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
    );
    assert_eq_pr!(cell, res,);
}

#[test]
fn modify_positions_or_not_res_1() {
    let mut res = TradeCell::new(S.capital, SRC_EL.to_vec(), SRC_EL_L.to_vec());
    let (qty, commission) = qty_and_commission(&S, res.capital, &SIGNAL, "market", 0., 0.);
    let (_, commission_limit) = qty_and_commission(&S, res.capital, &SIGNAL, "limit", 0., 0.);
    let price = SRC_EL[1];
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
    let mut cell = TradeCell::new(S.capital, SRC_EL.to_vec(), SRC_EL_L.to_vec());
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
            None,
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
    );
    modify_positions_or_not(
        &S,
        &mut cell,
        &Order::new(
            "".to_string(),
            "buy".to_string(),
            *SIGNAL,
            qty,
            0.,
            S.leverage,
            Some(price),
            "limit".to_string(),
            Default::default(),
            Default::default(),
            Some("last".to_string()),
            Some(price),
            Some(1),
            false,
            "124241".to_string(),
            "1".to_string(),
            true,
        ),
    );
    assert_eq_pr!(cell, res);
}

#[test]
fn trigger_direction_res_1() {
    assert_eq_pr!(trigger_direction(1., 0.9), 2)
}

#[test]
fn tp_sl_orders_res_1() {
    assert_eq_pr!(
        {
            let mut bind = tp_sl_orders("sl", &S.stoploss, &S, "", "buy", 1.1, &SIGNAL, "1", 2);
            for o in &mut bind {
                set_order_link_id(o);
            }
            bind
        },
        vec![Order::new(
            "".to_string(),
            "buy".to_string(),
            *SIGNAL,
            0.,
            1.0,
            S.leverage,
            None,
            "market".to_string(),
            Default::default(),
            Default::default(),
            Some("last".to_string()),
            Some(1.1 * 0.5),
            Some(2),
            true,
            "".to_string(),
            "1".to_string(),
            true
        )]
    )
}

#[test]
fn order_create_res_1() {
    assert_eq_pr!(
        {
            let mut bind = order_create(
                &S,
                &TradeCell::new(100., SRC_EL_L.to_vec(), SRC_EL_L1.to_vec()),
                "",
                Some(1.7),
                Some(1.75),
                &SIGNAL,
                SRC_EL.as_slice(),
                "limit",
                false,
            );
            set_order_link_id(&mut bind);
            bind
        },
        {
            let mut bind = Order::new(
                "".to_string(),
                "buy".to_string(),
                *SIGNAL,
                S.amount_of_capital + S.percent_of_capital * 100.,
                0.,
                S.leverage,
                Some(1.7),
                "limit".to_string(),
                Default::default(),
                tp_sl_orders("sl", &S.stoploss, &S, "", "buy", SRC_EL[1], &SIGNAL, "1", 2),
                Some(S.trigger_by.clone()),
                Some(1.75),
                Some(2),
                false,
                "".to_string(),
                "1".to_string(),
                true,
            );
            set_order_link_id(&mut bind);
            bind
        }
    )
}

#[test]
fn orders_market_extern_res_1() {
    assert_eq_pr!(
        {
            let mut vec = vec![];
            orders_market_extern(
                &mut vec,
                &S,
                &TradeCell::new(100., SRC_EL_L.to_vec(), SRC_EL_L1.to_vec()),
                "",
                &MAP::from_iter([("th_1", Signal::new(1., 1.))]),
                &SRC,
            );
            for o in vec.iter_mut() {
                set_order_link_id(o);
            }
            vec
        },
        vec![Order::new(
            "".to_string(),
            "buy".to_string(),
            Signal::new(1., 1.),
            100. * S.percent_of_capital,
            0.,
            S.leverage,
            None,
            "market".to_string(),
            Default::default(),
            {
                let mut bind =
                    tp_sl_orders("sl", &S.stoploss, &S, "", "buy", SRC_EL[1], &SIGNAL, "1", 2);
                for o in bind.iter_mut() {
                    set_order_link_id(o);
                }
                bind
            },
            None,
            None,
            None,
            false,
            "".to_string(),
            "1".to_string(),
            true
        )]
    )
}

// #[test]
// fn orders_limit_extern_res_1() {
//     assert_eq_pr!(
//         {
//             let mut vec = vec![];
//             orders_limit_extern(
//                 &mut vec,
//                 &S,
//                 &TradeCell::new(100., SRC_EL_L.to_vec(), SRC_EL_L1.to_vec()),
//                 "",
//                 &MAP::from_iter([("th_1", Signal::new(1., 1.))]),
//                 &SRC,
//             );
//             for o in vec.iter_mut() {
//                 set_order_link_id(o);
//             }
//             vec
//         },
//         vec![Order::new(
//             "".to_string(),
//             "buy".to_string(),
//             Signal::new(1., 1.),
//             100. * S.percent_of_capital,
//             0.,
//             S.leverage,
//             None,
//             "market".to_string(),
//             Default::default(),
//             {
//                 let mut bind =
//                     tp_sl_orders("sl", &S.stoploss, &S, "", "buy", SRC_EL[1], &SIGNAL, "1", 2);
//                 for o in bind.iter_mut() {
//                     set_order_link_id(o);
//                 }
//                 bind
//             },
//             None,
//             None,
//             None,
//             false,
//             "".to_string(),
//             "1".to_string(),
//             true
//         )]
//     )
// }