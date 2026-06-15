use bc_indicators::indicators::ready_imports::*;
use bc_indicators::indicators::trend_ma::TREND_MA;
use bc_signals::train::mm::MM;
use bc_signals::train::ready_imports::*;
use bc_utils_lg::statics::prices::{SRC_NOMAP, SRC_TRANSPOSE};
use bc_utils_lg::types::maps::MAP;

use bc_constructor::indicators::*;
use bc_constructor::map::indicators::{
    FUNCS_EXTRACT_ARGS as FUNCS_EXTRACT_ARGS_IND, get_indicators_from_settings,
    get_indicators_from_settings_without_bf,
};
use bc_constructor::map::signals_train::{
    FUNCS_EXTRACT_ARGS as FUNCS_EXTRACT_ARGS_SR, get_signals_from_settings,
    get_signals_from_settings_without_bf,
};
use bc_constructor::settings::{
    SETTINGS_IND, SETTINGS_INDS, SETTINGS_SIGNAL, SETTINGS_SIGNALS, SETTINGS_USED_SRC,
};
use bc_constructor::signals_train::*;

#[test]
fn signals_train_res_1() {
    let settings_indicators = SETTINGS_INDS::from_iter([
        (
            "trend_ma_1".to_string(),
            SETTINGS_IND {
                key: "trend_ma".to_string(),
                used_src: vec![SETTINGS_USED_SRC { index: 0, sub_from_last_i: 0 }],
                ..Default::default()
            },
        ),
        (
            "repeat_1".to_string(),
            SETTINGS_IND {
                key: "repeat".to_string(),
                kwargs_f64: MAP::from_iter([("value".to_string(), 1.0)]),
                used_src: vec![SETTINGS_USED_SRC { index: 0, sub_from_last_i: 0 }],
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
    let ind_without_bf =
        get_indicators_from_settings_without_bf(&settings_indicators, &FUNCS_EXTRACT_ARGS_IND());
    let ind_bf = get_indicators_from_settings(
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_IND(),
        &SRC_TRANSPOSE,
        &ind_without_bf,
    );
    let signals_without_bf =
        get_signals_from_settings_without_bf(&settings_signals, &FUNCS_EXTRACT_ARGS_SR());
    let signals_bf = get_signals_from_settings(
        &settings_signals,
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_SR(),
        &SRC_TRANSPOSE,
        &signals_without_bf,
        &ind_without_bf,
    );
    let indicators_gw = IndicatorsGateway::new(&ind_bf, &ind_without_bf, &settings_indicators);
    let indications = indicators_gw.get_indications_from_settings(&SRC_TRANSPOSE);
    let signals_gw = SignalsTrainGateway::new(
        &signals_bf,
        &ind_bf,
        &signals_without_bf,
        &ind_without_bf,
        &settings_signals,
        &settings_indicators,
    );
    let res_1 = signals_gw.get_signals_from_settings(&indications, &SRC_TRANSPOSE)["mm_1"];
    let res_2 = {
        let mut df = MM::default();
        df.set_window(10);
        df
    }
    .signal(
        &TREND_MA::default()
            .ind_vec(&SRC_NOMAP)
            .into_iter()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>(),
        &vec![],
    );
    assert_eq!(res_1, res_2);
}

#[test]
fn signals_train_vec_res_1() {
    let settings_indicators = SETTINGS_INDS::from_iter([
        (
            "trend_ma_1".to_string(),
            SETTINGS_IND {
                key: "trend_ma".to_string(),
                used_src: vec![SETTINGS_USED_SRC { index: 0, sub_from_last_i: 0 }],
                ..Default::default()
            },
        ),
        (
            "repeat_1".to_string(),
            SETTINGS_IND {
                key: "repeat".to_string(),
                kwargs_f64: MAP::from_iter([("value".to_string(), 1.0)]),
                used_src: vec![SETTINGS_USED_SRC { index: 0, sub_from_last_i: 0 }],
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
    let ind_without_bf =
        get_indicators_from_settings_without_bf(&settings_indicators, &FUNCS_EXTRACT_ARGS_IND());
    let ind_bf = get_indicators_from_settings(
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_IND(),
        &SRC_TRANSPOSE,
        &ind_without_bf,
    );
    let signals_without_bf =
        get_signals_from_settings_without_bf(&settings_signals, &FUNCS_EXTRACT_ARGS_SR());
    let signals_bf = get_signals_from_settings(
        &settings_signals,
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_SR(),
        &SRC_TRANSPOSE,
        &signals_without_bf,
        &ind_without_bf,
    );
    let signals_gw = SignalsTrainGateway::new(
        &signals_bf,
        &ind_bf,
        &signals_without_bf,
        &ind_without_bf,
        &settings_signals,
        &settings_indicators,
    );
    let res_1 = &signals_gw.get_signals_vec_from_settings(&SRC_TRANSPOSE)["mm_1"];
    let res_2 = &{
        let mut df = MM::default();
        df.set_window(10);
        df
    }
    .signals_vec(
        &TREND_MA::default()
            .ind_vec(&SRC_NOMAP)
            .into_iter()
            .map(|v| vec![v])
            .collect::<Vec<Vec<f64>>>(),
        &vec![],
    );
    assert_eq!(
        res_1.iter().filter(|v| !v.is_nan()).collect::<Vec<&f64>>(),
        res_2.iter().filter(|v| !v.is_nan()).collect::<Vec<&f64>>()
    );
}
