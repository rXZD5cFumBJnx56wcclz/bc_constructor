use std::hint::black_box;

use bc_constructor::signals_ready::SignalsReadyGateway;
use bc_utils_lg::statics::prices::SRC_TRANSPOSE;
use bc_utils_lg::types::maps::MAP;
use criterion::{Criterion, criterion_group, criterion_main};

use bc_constructor::indicators::IndicatorsGateway;
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

fn get_signals_from_settings_1(c: &mut Criterion) {
    let s = SETTINGS_SIGNALS::from_iter([(
        "pumpdump_1".to_string(),
        SETTINGS_SIGNAL {
            key: "pumpdump".to_string(),
            used_src: vec![SETTINGS_USED_SRC { index: 0, sub_from_last_i: 0 }],
            ..Default::default()
        },
    )]);
    let sr_without_bf = get_signals_from_settings_without_bf(&s, &FUNCS_EXTRACT_ARGS_SR());
    let bind = Default::default();
    let bind2 = Default::default();
    let bind3 = Default::default();
    let bind4 = Default::default();
    let bind5 = Default::default();
    let bind6 = Default::default();
    let sr_bf = get_signals_from_settings(
        &s,
        &bind,
        &FUNCS_EXTRACT_ARGS_SR(),
        &SRC_TRANSPOSE,
        &sr_without_bf,
        &bind5,
    );
    let sr_gw = SignalsReadyGateway::new(&sr_bf, &bind3, &sr_without_bf, &bind4, &s, &bind6);
    c.bench_function("get_signals_from_settings_1", |b| {
        b.iter(|| sr_gw.get_signals_from_settings(black_box(&bind2), black_box(&SRC_TRANSPOSE)))
    });
}

fn get_signals_from_settings_2(c: &mut Criterion) {
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
                key: "change".to_string(),
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
    c.bench_function("get_signals_from_settings_2", |b| {
        b.iter(|| {
            signals_gw.get_signals_from_settings(black_box(&indications), black_box(&SRC_TRANSPOSE))
        })
    });
}

criterion_group!(
    benches,
    get_signals_from_settings_1,
    get_signals_from_settings_2
);
criterion_main!(benches);
