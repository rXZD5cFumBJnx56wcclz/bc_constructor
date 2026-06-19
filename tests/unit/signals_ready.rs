use bc_indicators::indicators::ready_imports::*;
use bc_indicators::indicators::{repeat::REPEAT, trend_ma::TREND_MA};
use bc_signals::ready::ready_imports::*;
use bc_signals::ready::{change_signal::CHANGE_SIGNAL, convert::CONVERT, invert::INVERT};
use bc_utils_lg::statics::prices::{SRC_NOMAP, SRC_TRANSPOSE};
use bc_utils_lg::types::maps::MAP;

use bc_constructor::indicators::*;
use bc_constructor::map::indicators::{
    FUNCS_EXTRACT_ARGS as FUNCS_EXTRACT_ARGS_IND, get_indicators_from_settings,
    get_indicators_from_settings_without_bf,
};
use bc_constructor::map::signals_ready::{
    FUNCS_EXTRACT_ARGS as FUNCS_EXTRACT_ARGS_SR, get_signals_from_settings,
    get_signals_from_settings_without_bf,
};
use bc_constructor::settings::{
    SETTINGS_IND, SETTINGS_INDS, SETTINGS_SIGNAL, SETTINGS_SIGNALS, SETTINGS_USED_SRC,
};
use bc_constructor::signals_ready::*;

#[test]
fn signals_ready_res_1() {
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
    let settings_signals = SETTINGS_SIGNALS::from_iter([
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
    ]);
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
    let signals_gw = SignalsReadyGateway::new(
        &signals_bf,
        &ind_bf,
        &signals_without_bf,
        &ind_without_bf,
        &settings_signals,
        &settings_indicators,
    );
    let res_1 = signals_gw.get_signals_from_settings(&indications, &SRC_TRANSPOSE)["invert_1"];
    let res_2 = INVERT::default().signal(
        &vec![],
        &vec![vec![
            CHANGE_SIGNAL::default().signal(
                &vec![],
                &CONVERT::default()
                    .signals_vec(
                        &TREND_MA::default()
                            .ind_vec(&SRC_NOMAP)
                            .into_iter()
                            .zip(REPEAT::new(1.0).ind_vec(&SRC_NOMAP))
                            .map(|(v1, v2)| vec![v1, v2])
                            .collect::<Vec<Vec<f64>>>(),
                        &vec![],
                    )
                    .into_iter()
                    .map(|s| vec![s])
                    .collect::<Vec<Vec<Signal>>>(),
            ),
        ]],
    );
    assert_eq!(res_1, res_2);
}

#[test]
fn signals_ready_vec_res_1() {
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
    let settings_signals = SETTINGS_SIGNALS::from_iter([
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
    ]);
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
    let signals_gw = SignalsReadyGateway::new(
        &signals_bf,
        &ind_bf,
        &signals_without_bf,
        &ind_without_bf,
        &settings_signals,
        &settings_indicators,
    );
    let res_1 = &signals_gw.get_signals_vec_from_settings(&SRC_TRANSPOSE)["invert_1"];
    let res_2 = &INVERT::default().signals_vec(
        &vec![],
        &CHANGE_SIGNAL::default()
            .signals_vec(
                &vec![],
                &CONVERT::default()
                    .signals_vec(
                        &TREND_MA::default()
                            .ind_vec(&SRC_NOMAP)
                            .into_iter()
                            .zip(REPEAT::new(1.0).ind_vec(&SRC_NOMAP))
                            .map(|(v1, v2)| vec![v1, v2])
                            .collect::<Vec<Vec<f64>>>(),
                        &vec![],
                    )
                    .into_iter()
                    .map(|s| vec![s])
                    .collect::<Vec<Vec<Signal>>>(),
            )
            .into_iter()
            .map(|s| vec![s])
            .collect::<Vec<Vec<Signal>>>(),
    );
    assert_eq!(
        res_1
            .iter()
            .filter(|s| !s.signal.is_nan())
            .collect::<Vec<_>>(),
        res_2
            .iter()
            .filter(|s| !s.signal.is_nan())
            .collect::<Vec<_>>()
    );
}
