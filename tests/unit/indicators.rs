use bc_indicators::indicators::ready_imports::Indicator;
use bc_indicators::indicators::{rma::RMA, rsi::RSI};
use bc_utils::nums::{coll_nz, round_f};
use bc_utils_lg::statics::prices::{CLOSE, OPEN, OPEN_LAST, SRC_NOMAP, SRC_TRANSPOSE};
use bc_utils_lg::types::maps::MAP;

use bc_constructor::indicators::*;
use bc_constructor::map::indicators::*;
use bc_constructor::settings::{SETTINGS_IND, SETTINGS_INDS, SETTINGS_USED_SRC};

#[test]
fn indication_res_1() {
    let settings = SETTINGS_INDS::from_iter([
        (
            "rsi_1".to_string(),
            SETTINGS_IND {
                key: "rsi".to_string(),
                kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![SETTINGS_USED_SRC { index: 0, sub_from_last_i: 0 }],
                used_ind: vec![],
                order_used: vec![],
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
                order_used: vec![],
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
                    SETTINGS_USED_SRC { index: 0, sub_from_last_i: 0 },
                    SETTINGS_USED_SRC { index: 3, sub_from_last_i: 2 },
                ],
                used_ind: vec!["rma_1".to_string()],
                order_used: vec![],
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
                order_used: vec![1, 0],
                ..Default::default()
            },
        ),
    ]);
    let ind_without_bf = get_indicators_from_settings_without_bf(&settings, &FUNCS_EXTRACT_ARGS());
    let ind_bf = get_indicators_from_settings(
        &settings,
        &FUNCS_EXTRACT_ARGS(),
        &SRC_TRANSPOSE,
        &ind_without_bf,
    );
    let indicators_gw = IndicatorsGateway::new(&ind_bf, &ind_without_bf, &settings);
    let res_1 = indicators_gw.get_indications_from_settings(&SRC_TRANSPOSE);
    let res_2 = (RMA::new(2).ind_f(
        &RSI::new(2)
            .ind_vec(&OPEN.into_iter().map(|v| vec![v]).collect::<Vec<Vec<f64>>>())
            .into_iter()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>(),
    ) + CLOSE[47]
        + OPEN_LAST)
        / 3.;
    assert_eq!(round_f(res_1["avg_1"], &4,), round_f(res_2, &4,),);
    assert_eq!(res_1["minus_1"], 1.0);
}

#[test]
fn indications_vec_res_1() {
    let settings = SETTINGS_INDS::from_iter([(
        "rsi_1".to_string(),
        SETTINGS_IND {
            key: "rsi".to_string(),
            kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
            kwargs_f64: MAP::default(),
            kwargs_string: MAP::default(),
            used_src: vec![SETTINGS_USED_SRC { index: 0, sub_from_last_i: 0 }],
            used_ind: vec![],
            order_used: vec![],
        },
    )]);
    let ind_without_bf = get_indicators_from_settings_without_bf(&settings, &FUNCS_EXTRACT_ARGS());
    let ind_bf = get_indicators_from_settings(
        &settings,
        &FUNCS_EXTRACT_ARGS(),
        &SRC_TRANSPOSE,
        &ind_without_bf,
    );
    let indicators_gw = IndicatorsGateway::new(&ind_bf, &ind_without_bf, &settings);
    let res_1 = indicators_gw.get_indications_vec_from_settings(&SRC_TRANSPOSE)["rsi_1"].clone();
    let res_2 = RSI::new(2).ind_vec(&SRC_NOMAP);
    assert_eq!(
        coll_nz::<Vec<f64>, _, _>(&res_1, 0.0),
        coll_nz::<Vec<f64>, _, _>(&res_2, 0.0)
    );
}
