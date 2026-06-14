use bc_utils_lg::statics::prices::SRC_TRANSPOSE;
use bc_utils_lg::types::maps::MAP;
use criterion::{Criterion, criterion_group, criterion_main};

use bc_constructor::indicators::IndicatorsGateway;
use bc_constructor::map::indicators::{
    FUNCS_EXTRACT_ARGS, get_indicators_from_settings, get_indicators_from_settings_without_bf,
};
use bc_constructor::settings::{SETTINGS_IND, SETTINGS_INDS, SETTINGS_USED_SRC};

fn get_indications_from_settings_1(c: &mut Criterion) {
    let s = SETTINGS_INDS::from_iter([(
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
    let ind_without_bf = get_indicators_from_settings_without_bf(&s, &FUNCS_EXTRACT_ARGS());
    let ind_bf =
        get_indicators_from_settings(&s, &FUNCS_EXTRACT_ARGS(), &SRC_TRANSPOSE, &ind_without_bf);
    let indicators_gw = IndicatorsGateway::new(&ind_bf, &ind_without_bf, &s);
    c.bench_function("get_indications_from_settings_1", |b| {
        b.iter(|| indicators_gw.get_indications_from_settings(&SRC_TRANSPOSE))
    });
}

fn get_indications_from_settings_2(c: &mut Criterion) {
    let s = SETTINGS_INDS::from_iter([
        (
            "avg_1".to_string(),
            SETTINGS_IND {
                key: "avg".to_string(),
                kwargs_usize: MAP::from_iter([]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![
                    SETTINGS_USED_SRC { index: 0, sub_from_last_i: 0 },
                    SETTINGS_USED_SRC { index: 1, sub_from_last_i: 1 },
                    SETTINGS_USED_SRC { index: 2, sub_from_last_i: 1 },
                    SETTINGS_USED_SRC { index: 3, sub_from_last_i: 1 },
                ],
                used_ind: vec![],
                order_used: vec![],
            },
        ),
        (
            "ema_1".to_string(),
            SETTINGS_IND {
                key: "ema".to_string(),
                kwargs_usize: MAP::from_iter([("window".to_string(), 4)]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![],
                used_ind: vec!["avg_1".to_string()],
                order_used: vec![],
            },
        ),
        (
            "trend_ma_1".to_string(),
            SETTINGS_IND {
                key: "trend_ma".to_string(),
                kwargs_usize: MAP::from_iter([]),
                kwargs_f64: MAP::default(),
                kwargs_string: MAP::default(),
                used_src: vec![],
                used_ind: vec!["ema_1".to_string()],
                order_used: vec![],
            },
        ),
    ]);
    let ind_without_bf = get_indicators_from_settings_without_bf(&s, &FUNCS_EXTRACT_ARGS());
    let ind_bf =
        get_indicators_from_settings(&s, &FUNCS_EXTRACT_ARGS(), &SRC_TRANSPOSE, &ind_without_bf);
    let indicators_gw = IndicatorsGateway::new(&ind_bf, &ind_without_bf, &s);
    c.bench_function("get_indications_from_settings_2", |b| {
        b.iter(|| indicators_gw.get_indications_from_settings(&SRC_TRANSPOSE))
    });
}

criterion_group!(
    benches,
    get_indications_from_settings_1,
    get_indications_from_settings_2
);
criterion_main!(benches);
