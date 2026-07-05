use std::cell::RefCell;

use bc_constructor::trade::statistics::{NumsExt, StatCollector, StatData};
use bc_utils::nums::nz_coll;
use bc_utils_lg::types::maps::MAP_LINK;

use crate::unit::trade::prelude::*;

static ST: LazyLock<fn() -> StatCollector<'static>> = LazyLock::new(|| {
    || {
        let mut bind = StatCollector::new("".to_string(), &S);
        bind.cells.extend_from_slice(&[
            TradeCell {
                capital: 100.,
                src: SRC_EL_L.to_vec(),
                src_l: SRC_EL_L1.to_vec(),
                trigger_orders: RefCell::new(MAP::from_iter([(
                    "1".to_string(),
                    Order::new(
                        "".to_string(),
                        "buy".to_string(),
                        *SIGNAL,
                        S.capital * S.percent_of_capital,
                        0.,
                        S.leverage,
                        Some(3.),
                        "limit".to_string(),
                        Default::default(),
                        Default::default(),
                        None,
                        None,
                        None,
                        false,
                        "".to_string(),
                        "1".to_string(),
                        true,
                    ),
                )])),
                positions: RefCell::new(MAP::from_iter([(
                    "1".to_string(),
                    Position::new(
                        "".to_string(),
                        "buy".to_string(),
                        S.capital * S.percent_of_capital,
                        S.leverage,
                        1.7,
                        "1".to_string(),
                        true,
                    ),
                )])),
                market_orders: RefCell::new(MAP::from_iter([(
                    "1".to_string(),
                    Order::new(
                        "".to_string(),
                        "".to_string(),
                        *SIGNAL,
                        S.capital * S.percent_of_capital,
                        0.,
                        S.leverage,
                        None,
                        "market".to_string(),
                        Default::default(),
                        Default::default(),
                        None,
                        None,
                        None,
                        false,
                        "1".to_string(),
                        "1".to_string(),
                        true,
                    ),
                )])),
                ..Default::default()
            },
            TradeCell {
                capital: 100.,
                src: SRC_EL.to_vec(),
                src_l: SRC_EL_L.to_vec(),
                positions: RefCell::new(MAP::from_iter([(
                    "1".to_string(),
                    Position::new(
                        "".to_string(),
                        "buy".to_string(),
                        S.capital * S.percent_of_capital,
                        S.leverage,
                        1.7,
                        "1".to_string(),
                        false,
                    ),
                )])),
                market_orders: RefCell::new(MAP::from_iter([(
                    "1".to_string(),
                    Order::new(
                        "".to_string(),
                        "".to_string(),
                        *SIGNAL,
                        S.capital * S.percent_of_capital,
                        0.,
                        S.leverage,
                        None,
                        "market".to_string(),
                        Default::default(),
                        Default::default(),
                        None,
                        None,
                        None,
                        true,
                        "1".to_string(),
                        "1".to_string(),
                        true,
                    ),
                )])),
                ..Default::default()
            },
        ]);
        bind
    }
});

#[test]
fn to_all_res_1() {
    assert_eq_pr!(
        vec![1., 0.,],
        nz_coll::<Vec<f64>, _, _>(
            &StatCollector::to_all(&[vec![1., f64::NAN,], vec![1., 2.,]]),
            0.
        )
    )
}

#[test]
fn to_any_res_1() {
    assert_eq_pr!(
        vec![1., 2.,],
        StatCollector::to_any(&[vec![1., f64::NAN,], vec![1., 2.,]])
    )
}

#[test]
fn to_some_res_1() {
    assert_eq_pr!(
        vec![SRC_EL_L[1], 0.0],
        nz_coll::<Vec<_>, _, _>(&ST().to_some(|v| v.trigger_orders.borrow(), true), 0.)
    )
}

#[test]
fn to_capital_res_1() {
    assert_eq_pr!(vec![100., 100.,], ST().to_capital())
}

#[test]
fn to_pnl_res_1() {
    assert_eq_pr!(
        vec![
            qty_pnl(
                S.leverage,
                S.capital * S.percent_of_capital,
                1.7,
                SRC_EL_L[1],
                "1"
            ),
            qty_pnl(
                S.leverage,
                S.capital * S.percent_of_capital,
                1.7,
                SRC_EL[1],
                "1"
            )
        ],
        ST().to_pnl()
    )
}

#[test]
fn to_entry_res_1() {
    assert_eq_pr!(
        vec![SRC_EL_L[1], 0.],
        nz_coll::<Vec<f64>, _, _>(&ST().to_entry(), 0.)
    )
}

#[test]
fn to_exit_res_1() {
    assert_eq_pr!(
        vec![0., SRC_EL[1]],
        nz_coll::<Vec<f64>, _, _>(&ST().to_exit(), 0.)
    )
}

#[test]
fn to_market_orders_res_1() {
    assert_eq_pr!(
        ST().to_some(|c| c.market_orders.borrow(), true),
        ST().to_market_orders()
    )
}

#[test]
fn to_limit_orders_res_1() {
    assert_eq_pr!(
        nz_coll::<Vec<_>, _, _>(&ST().to_some(|c| c.limit_orders.borrow(), true), 0.),
        nz_coll::<Vec<_>, _, _>(&ST().to_limit_orders(), 0.,),
    )
}

#[test]
fn to_entry_and_exit_res_1() {
    assert_eq_pr!(vec![SRC_EL_L[1], SRC_EL[1],], ST().to_entry_and_exit())
}

#[test]
fn to_positions_entry_exit_res_1() {
    assert_eq_pr!(vec![SRC_EL_L[1], SRC_EL[1]], ST().to_positions_entry_exit(),)
}

#[test]
fn to_value_positions_res_1() {
    assert_eq_pr!(
        vec![S.capital * S.percent_of_capital; 2],
        ST().to_value_positions(|v| v.qty)
    )
}

#[test]
fn to_data_res_1() {
    assert_eq_pr!(
        StatData(vec![
            MAP_LINK::from_iter([
                ("time".to_string(), vec![0., 1.,]),
                ("open".to_string(), vec![SRC_EL_L[1], SRC_EL[1],]),
                ("high".to_string(), vec![SRC_EL_L[2], SRC_EL[2],]),
                ("low".to_string(), vec![SRC_EL_L[3], SRC_EL[3],]),
                ("close".to_string(), vec![SRC_EL_L[4], SRC_EL[4],]),
                ("volume".to_string(), vec![SRC_EL_L[5], SRC_EL[5],]),
                ("turnover".to_string(), vec![SRC_EL_L[6], SRC_EL[6],]),
                ("capital".to_string(), ST().to_capital()),
                ("entry".to_string(), vec![SRC_EL_L[1], 0.,]),
                ("exit".to_string(), vec![0., SRC_EL[1],]),
                (
                    "pnl".to_string(),
                    nz_coll::<Vec<_>, _, _>(
                        &StatCollector::to_all(&[ST().to_pnl(), ST().to_exit(),]),
                        0.
                    ),
                ),
                ("qty".to_string(), ST().to_value_positions(|v| v.qty)),
            ]),
            MAP_LINK::from_iter([
                ("time".to_string(), vec![0., 1.,]),
                (
                    "positions_entry_exit".to_string(),
                    vec![SRC_EL_L[1], SRC_EL[1]]
                )
            ])
        ]),
        {
            let mut bind = ST().to_data();
            bind.0[0]["entry"] = nz_coll::<Vec<_>, _, _>(&bind.0[0]["entry"], 0.);
            bind.0[0]["exit"] = nz_coll::<Vec<_>, _, _>(&bind.0[0]["exit"], 0.);
            bind.0[0]["pnl"] = nz_coll::<Vec<_>, _, _>(&bind.0[0]["pnl"], 0.);
            bind
        }
    )
}

#[test]
fn del_nan_res_1() {
    assert_eq_pr!(
        vec![(0usize, 1.,)],
        vec![1., f64::NAN]
            .into_iter()
            .del_nan(0)
            .collect::<Vec<(usize, f64)>>()
    )
}
