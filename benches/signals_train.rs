use std::hint::black_box;

use bc_constructor::signals_train::SignalsTrainGateway;
use bc_utils_lg::statics::prices::SRC_TRANSPOSE;
use bc_utils_lg::types::maps::MAP;
use criterion::{Criterion, criterion_group, criterion_main};

use bc_constructor::map::signals_train::{
    FUNCS_EXTRACT_ARGS as FUNCS_EXTRACT_ARGS_SR, get_signals_from_settings,
    get_signals_from_settings_without_bf,
};
use bc_constructor::settings::{SETTINGS_SIGNAL, SETTINGS_SIGNALS, SETTINGS_USED_SRC};

fn get_signals_train_from_settings_1(c: &mut Criterion) {
    let settings_signals = SETTINGS_SIGNALS::from_iter([(
        "mm_1".to_string(),
        SETTINGS_SIGNAL {
            key: "mm".to_string(),
            kwargs_usize: MAP::from_iter([("window".to_string(), 10)]),
            used_src: vec![SETTINGS_USED_SRC { index: 0, sub_from_last_i: 0 }],
            ..Default::default()
        },
    )]);
    let sr_without_bf =
        get_signals_from_settings_without_bf(&settings_signals, &FUNCS_EXTRACT_ARGS_SR());
    let bind = Default::default();
    let bind2 = Default::default();
    let bind3 = Default::default();
    let bind4 = Default::default();
    let bind5 = Default::default();
    let bind6 = Default::default();
    let sr_bf = get_signals_from_settings(
        &settings_signals,
        &bind,
        &FUNCS_EXTRACT_ARGS_SR(),
        &SRC_TRANSPOSE,
        &sr_without_bf,
        &bind5,
    );
    let sr_gw = SignalsTrainGateway::new(
        &sr_bf,
        &bind3,
        &sr_without_bf,
        &bind4,
        &settings_signals,
        &bind6,
    );
    c.bench_function("get_signals_train_from_settings_1", |b| {
        b.iter(|| sr_gw.get_signals_from_settings(black_box(&bind2), black_box(&SRC_TRANSPOSE)))
    });
}

criterion_group!(benches, get_signals_train_from_settings_1,);
criterion_main!(benches);
