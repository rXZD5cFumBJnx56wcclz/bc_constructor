use std::any::Any;

use bc_indicators::ready_imports::*;
use bc_indicators::trend_ma::TREND_MA;
use bc_pack_indicators::FUNCS_EXTRACT_ARGS as FUNCS_EXTRACT_ARGS_IND;
use bc_pack_signals_train::FUNCS_EXTRACT_ARGS as FUNCS_EXTRACT_ARGS_ST;
use bc_signals::train::mm::MM;
use bc_signals::train::ready_imports::*;
use bc_utils_lg::statics::prices::{SRC, SRC_TRANSPOSE};
use bc_utils_lg::structs::settings::{
    SETTINGS_IND, SETTINGS_INDS, SETTINGS_SIGNAL, SETTINGS_SIGNALS, SETTINGS_USED_SRC,
};
use bc_utils_lg::types::maps::MAP;
use pretty_assertions::assert_eq as assert_eq_pr;

use bc_constructor::indicators::*;
use bc_constructor::signals_train::*;

#[test]
fn signals_from_settings_without_bf_res_1() {
    let settings = SETTINGS_SIGNALS::from_iter([(
        "mm_1".to_string(),
        SETTINGS_SIGNAL { key: "mm".to_string(), ..Default::default() },
    )]);
    let funcs_extract_args = FUNCS_EXTRACT_ARGS_ST();
    let res = get_signals_from_settings_without_bf(&settings, &funcs_extract_args);
    let res_1 = res.get("mm_1").unwrap().as_ref();
    let rsi_test_1 = MM::default();
    let rsi_test_2 = (res_1 as &dyn Any).downcast_ref::<MM>().unwrap();
    assert_eq_pr!(&rsi_test_1, rsi_test_2);
}

#[test]
fn signals_train_res_1() {
    let settings_indicators = SETTINGS_INDS::from_iter([
        (
            "trend_ma_1".to_string(),
            SETTINGS_IND {
                key: "trend_ma".to_string(),
                used_src: vec![SETTINGS_USED_SRC { index: 1, sub_from_last_i: 0 }],
                ..Default::default()
            },
        ),
        (
            "repeat_1".to_string(),
            SETTINGS_IND {
                key: "repeat".to_string(),
                kwargs_f64: MAP::from_iter([("value".to_string(), 1.0)]),
                used_src: vec![SETTINGS_USED_SRC { index: 1, sub_from_last_i: 0 }],
                ..Default::default()
            },
        ),
    ]);
    let settings_signals = SETTINGS_SIGNALS::from_iter([(
        "mm_1".to_string(),
        SETTINGS_SIGNAL {
            key: "mm".to_string(),
            kwargs_usize: MAP::from_iter([("window".to_string(), 10)]),
            used_ind: vec!["trend_ma_1".to_string(), "repeat_1".to_string()],
            ..Default::default()
        },
    )]);
    let indicators = Indicators::new(
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_IND(),
        &SRC_TRANSPOSE,
    );
    let signals = SignalsTrain::new(
        &settings_signals,
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_ST(),
        &SRC_TRANSPOSE,
        &indicators.indicators_without_bf,
    );
    let indicators_gw = IndicatorsGateway::new(&indicators, &settings_indicators);
    let indications = indicators_gw.indications_series(&SRC_TRANSPOSE);
    let signals_gw = SignalsTrainGateway::new(
        &signals,
        &indicators,
        &settings_signals,
        &settings_indicators,
    );
    let res_1 = signals_gw.signals_series(&indications, &SRC_TRANSPOSE)["mm_1"];
    let res_2 = {
        let mut df = MM::default();
        df.set_window(10);
        df
    }
    .signal(
        &TREND_MA::default()
            .ind_vec(&SRC)
            .into_iter()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>(),
        &vec![],
    );
    assert_eq_pr!(res_1, res_2);
}

#[test]
fn signals_train_vec_res_1() {
    let settings_indicators = SETTINGS_INDS::from_iter([
        (
            "trend_ma_1".to_string(),
            SETTINGS_IND {
                key: "trend_ma".to_string(),
                used_src: vec![SETTINGS_USED_SRC { index: 1, sub_from_last_i: 0 }],
                ..Default::default()
            },
        ),
        (
            "repeat_1".to_string(),
            SETTINGS_IND {
                key: "repeat".to_string(),
                kwargs_f64: MAP::from_iter([("value".to_string(), 1.0)]),
                used_src: vec![SETTINGS_USED_SRC { index: 1, sub_from_last_i: 0 }],
                ..Default::default()
            },
        ),
    ]);
    let settings_signals = SETTINGS_SIGNALS::from_iter([(
        "mm_1".to_string(),
        SETTINGS_SIGNAL {
            key: "mm".to_string(),
            kwargs_usize: MAP::from_iter([("window".to_string(), 10)]),
            used_ind: vec!["trend_ma_1".to_string(), "repeat_1".to_string()],
            ..Default::default()
        },
    )]);
    let indicators = Indicators::new(
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_IND(),
        &SRC_TRANSPOSE,
    );
    let signals = SignalsTrain::new(
        &settings_signals,
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_ST(),
        &SRC_TRANSPOSE,
        &indicators.indicators_without_bf,
    );
    let signals_gw = SignalsTrainGateway::new(
        &signals,
        &indicators,
        &settings_signals,
        &settings_indicators,
    );
    let res_1 = &signals_gw.signals_vec(&SRC_TRANSPOSE)["mm_1"];
    let res_2 = &{
        let mut df = MM::default();
        df.set_window(10);
        df
    }
    .signals_vec(
        &TREND_MA::default()
            .ind_vec(&SRC)
            .into_iter()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>(),
        &vec![],
    );
    assert_eq_pr!(
        res_1.iter().filter(|v| !v.is_nan()).collect::<Vec<&f64>>(),
        res_2.iter().filter(|v| !v.is_nan()).collect::<Vec<&f64>>()
    );
}
