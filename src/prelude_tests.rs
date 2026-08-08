#![allow(unused_imports)]
#[cfg(test)]
pub mod prelude {
    pub use std::error::Error;
    pub use std::sync::LazyLock;

    pub use crate::trade::StepState;
    pub use crate::utils_cell::*;
    pub use bc_pack_indicators::PACK as FA_I;
    pub use bc_pack_orders_collectors::PACK as FA_O;
    pub use bc_pack_signals::PACK as FA_R;
    pub use bc_pack_signals_train::PACK as FA_T;
    pub use crate::prelude_tests::prelude::*;
    pub use bc_utils_lg::structs::settings::*;
    pub use bc_utils_lg::types::maps::*;
    pub use bc_utils_lg::{
        structs::{
            signals::Signal,
            trade::{Order, Position, TradeState},
        },
        types::maps::MAP,
    };
    pub use pretty_assertions::assert_eq as assert_eq_pr;

    pub static SIGNAL: LazyLock<Signal> = LazyLock::new(|| Signal::new(1.0, 1.0));

    pub static S: LazyLock<SETTINGS> = LazyLock::new(|| SETTINGS {
        indications: SETTINGS_INDS::from_iter([
            (
                "rsi_1".to_string(),
                SETTINGS_IND {
                    key: "rsi".to_string(),
                    kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
                    kwargs_f64: MAP::default(),
                    kwargs_string: MAP::default(),
                    used_src: vec![SETTINGS_USED_USIZE { index: 1, sub_from_last_i: 0 }],
                    used_ind: vec![],
                    procedure_used: vec![],
                },
            ),
            (
                "rsi_2".to_string(),
                SETTINGS_IND {
                    key: "rsi".to_string(),
                    kwargs_usize: MAP::from_iter([("window".to_string(), 3)]),
                    kwargs_f64: MAP::default(),
                    kwargs_string: MAP::default(),
                    used_src: vec![SETTINGS_USED_USIZE { index: 1, sub_from_last_i: 0 }],
                    used_ind: vec![],
                    procedure_used: vec![],
                },
            ),
            (
                "rma_1".to_string(),
                SETTINGS_IND {
                    key: "rma".to_string(),
                    kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
                    kwargs_f64: MAP::default(),
                    kwargs_string: MAP::default(),
                    used_src: vec![],
                    used_ind: vec!["rsi_1".to_string()],
                    procedure_used: vec![],
                },
            ),
            (
                "avg_1".to_string(),
                SETTINGS_IND {
                    key: "avg".to_string(),
                    kwargs_usize: MAP::from_iter([]),
                    kwargs_f64: MAP::default(),
                    kwargs_string: MAP::default(),
                    used_src: vec![
                        SETTINGS_USED_USIZE { index: 1, sub_from_last_i: 0 },
                        SETTINGS_USED_USIZE { index: 4, sub_from_last_i: 2 },
                    ],
                    used_ind: vec!["rma_1".to_string()],
                    procedure_used: vec![],
                },
            ),
            (
                "repeat_1".to_string(),
                SETTINGS_IND {
                    key: "repeat".to_string(),
                    kwargs_f64: MAP::from_iter([("value".to_string(), 1.0)]),
                    ..Default::default()
                },
            ),
            (
                "repeat_2".to_string(),
                SETTINGS_IND {
                    key: "repeat".to_string(),
                    kwargs_f64: MAP::from_iter([("value".to_string(), 2.0)]),
                    ..Default::default()
                },
            ),
            (
                "minus_1".to_string(),
                SETTINGS_IND {
                    key: "minus".to_string(),
                    used_ind: vec!["repeat_1".to_string(), "repeat_2".to_string()],
                    procedure_used: vec![1, 0],
                    ..Default::default()
                },
            ),
            (
                "trend_ma_1".to_string(),
                SETTINGS_IND {
                    key: "trend_ma".to_string(),
                    used_src: vec![SETTINGS_USED_USIZE { index: 1, sub_from_last_i: 0 }],
                    ..Default::default()
                },
            ),
            (
                "repeat_1".to_string(),
                SETTINGS_IND {
                    key: "repeat".to_string(),
                    kwargs_f64: MAP::from_iter([("value".to_string(), 1.0)]),
                    used_src: vec![SETTINGS_USED_USIZE { index: 1, sub_from_last_i: 0 }],
                    ..Default::default()
                },
            ),
        ]),
        signals_train: SETTINGS_SIGNALS::from_iter([(
            "mm_1".to_string(),
            SETTINGS_SIGNAL {
                key: "mm".to_string(),
                kwargs_usize: MAP::from_iter([("window".to_string(), 3)]),
                used_src: vec![SETTINGS_USED_USIZE { index: 1, ..Default::default() }],
                ..Default::default()
            },
        )]),
        signals: SETTINGS_SIGNALS::from_iter([
            (
                "convert_1".to_string(),
                SETTINGS_SIGNAL {
                    key: "convert".to_string(),
                    used_ind: vec!["trend_ma_1".to_string(), "repeat_1".to_string()],
                    ..Default::default()
                },
            ),
            (
                "change_1".to_string(),
                SETTINGS_SIGNAL {
                    key: "change_signal".to_string(),
                    used_signals: vec!["convert_1".to_string()],
                    ..Default::default()
                },
            ),
            (
                "invert_1".to_string(),
                SETTINGS_SIGNAL {
                    key: "invert".to_string(),
                    used_signals: vec!["change_1".to_string()],
                    ..Default::default()
                },
            ),
            (
                "th_1".to_string(),
                SETTINGS_SIGNAL {
                    key: "th".to_string(),
                    used_src: vec![
                        SETTINGS_USED_USIZE { index: 1, ..Default::default() },
                        SETTINGS_USED_USIZE { index: 2, ..Default::default() },
                        SETTINGS_USED_USIZE { index: 3, ..Default::default() },
                    ],
                    kwargs_f64: MAP::from_iter([
                        ("th_min".to_string(), 0.0001),
                        ("th_max".to_string(), 0.0001),
                        ("limit".to_string(), 9999.),
                    ]),
                    kwargs_usize: MAP::from_iter([
                        ("index_normal".to_string(), 0),
                        ("index_max".to_string(), 1),
                        ("index_min".to_string(), 2),
                    ]),
                    ..Default::default()
                },
            ),
        ]),
        trade: SETTINGS_TRADE {
            signal_hold: 0.,
            signal_short: -1.,
            signal_long: 1.,
            commission_market: 0.00055,
            commission_limit: 0.0002,
            leverage: 10.,
            capital: 100.,
            percent_of_capital: 0.01,
            stoploss: vec![(1., 0., 0.5)],
            order_collectors: vec![SETTINGS_ORDER_COLLECTOR {
                key: "clear".to_string(),
                ..Default::default()
            }],
            // market_entry_signals: SETTINGS_ORDERS_SIGNALS {
            //     signals: "th_1".to_string(),
            //     ..Default::default()
            // },
            // limit_entry_signals: SETTINGS_ORDERS_SIGNALS {
            //     signals: "th_1".to_string(),
            //     limit_price: Some("rsi_1".to_string()),
            //     ..Default::default()
            // },
            trigger_limit_entry_signals: SETTINGS_ORDERS_SIGNALS {
                signals: "th_1".to_string(),
                limit_price: Some("rsi_1".to_string()),
                trigger_price: Some("rsi_2".to_string()),
                ..Default::default()
            },
            ..Default::default()
        },
        indications_stat_values: SETTINGS_INDS::from_iter([(
            "profit_factor_1".to_string(),
            SETTINGS_IND {
                key: "profit_factor".to_string(),
                used_src: vec![SETTINGS_USED_USIZE { index: 10, ..Default::default() }],
                ..Default::default()
            },
        )]),
        ..Default::default()
    });

    pub fn set_order_link_id(v: &mut Order) {
        v.order_link_id = "".to_string();
    }
}
