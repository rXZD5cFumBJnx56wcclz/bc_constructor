use std::any::Any;

use bc_indicators::indicators::ready_imports::*;
use bc_indicators::indicators::{repeat::REPEAT, trend_ma::TREND_MA};
use bc_pack_indicators::FUNCS_EXTRACT_ARGS as FUNCS_EXTRACT_ARGS_IND;
use bc_pack_signals_ready::FUNCS_EXTRACT_ARGS as FUNCS_EXTRACT_ARGS_SR;
use bc_signals::ready::ready_imports::*;
use bc_signals::ready::{
    change_signal::CHANGE_SIGNAL, convert::CONVERT, invert::INVERT, th::TH,
};
use bc_utils_lg::settings::{
    SETTINGS_IND, SETTINGS_INDS, SETTINGS_SIGNAL, SETTINGS_SIGNALS, SETTINGS_USED_SRC,
};
use bc_utils_lg::statics::prices::{SRC_NOMAP, SRC_TRANSPOSE};
use bc_utils_lg::types::maps::MAP;

use bc_constructor::indicators::*;
use bc_constructor::signals_ready::*;

#[test]
fn signals_from_settings_without_bf_res_1() {
    let settings = SETTINGS_SIGNALS::from_iter([(
        "th_1".to_string(),
        SETTINGS_SIGNAL { key: "th".to_string(), ..Default::default() },
    )]);
    let funcs_extract_args = FUNCS_EXTRACT_ARGS_SR();
    let res = get_signals_from_settings_without_bf(&settings, &funcs_extract_args);
    let res_1 = res.get("th_1").unwrap().as_ref();
    let rsi_test_1 = TH::default();
    let rsi_test_2 = (res_1 as &dyn Any).downcast_ref::<TH>().unwrap();
    assert_eq!(&rsi_test_1, rsi_test_2);
}

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
    let indicators = Indicators::new(
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_IND(),
        &SRC_TRANSPOSE,
    );
    let signals = SignalsReady::new(
        &settings_signals,
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_SR(),
        &SRC_TRANSPOSE,
        &indicators.indicators_without_bf,
    );
    let indicators_gw = IndicatorsGateway::new(&indicators, &settings_indicators);
    let indications = indicators_gw.indications_series(&SRC_TRANSPOSE);
    let signals_gw = SignalsReadyGateway::new(
        &signals,
        &indicators,
        &settings_signals,
        &settings_indicators,
    );
    let res_1 = signals_gw.signals_series(&indications, &SRC_TRANSPOSE)["invert_1"];
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
    let indicators = Indicators::new(
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_IND(),
        &SRC_TRANSPOSE,
    );
    let signals = SignalsReady::new(
        &settings_signals,
        &settings_indicators,
        &FUNCS_EXTRACT_ARGS_SR(),
        &SRC_TRANSPOSE,
        &indicators.indicators_without_bf,
    );
    let signals_gw = SignalsReadyGateway::new(
        &signals,
        &indicators,
        &settings_signals,
        &settings_indicators,
    );
    let res_1 = &signals_gw.signals_vec(&SRC_TRANSPOSE)["invert_1"];
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
