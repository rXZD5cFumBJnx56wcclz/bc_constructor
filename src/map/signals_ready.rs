use std::sync::LazyLock;

use bc_indicators::indicators::ready_imports::Indicator;
use bc_signals::ready::ready_imports::*;
use bc_signals::ready::{
    change_signal::CHANGE_SIGNAL, change_src::CHANGE_SRC, convert::CONVERT, copy::COPY,
    filter::FILTER, invert::INVERT, pumpdump::PUMPDUMP, repeat::REPEAT,
    set_probability::SET_PROBABILITY,
};
use bc_utils_lg::types::maps::MAP;
use bc_utils_lg::types::structures::SRC_TRANSPOSE;

use crate::map::indicators::get_in_from_settings;
use crate::settings::{SETTINGS_INDS, SETTINGS_SIGNAL, SETTINGS_SIGNALS};

pub static SIGNALS_DEFAULT: LazyLock<fn() -> MAP<&'static str, Box<dyn SignalsReady>>> =
    LazyLock::new(|| {
        || {
            MAP::from_iter([
                (
                    "pumpdump",
                    Box::new(PUMPDUMP::default()) as Box<dyn SignalsReady>,
                ),
                ("change_signal", Box::new(CHANGE_SIGNAL::default())),
                ("convert", Box::new(CONVERT::default())),
                ("filter", Box::new(FILTER::default())),
                ("invert", Box::new(INVERT::default())),
                ("set_probability", Box::new(SET_PROBABILITY::default())),
                ("change_src", Box::new(CHANGE_SRC::default())),
                ("copy", Box::new(COPY::default())),
                ("repeat", Box::new(REPEAT::default())),
            ])
        }
    });

pub static FUNCS_EXTRACT_ARGS: LazyLock<
    fn() -> MAP<&'static str, fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsReady>>,
> = LazyLock::new(|| {
    || {
        MAP::from_iter([
            (
                "pumpdump",
                (|setting: &SETTINGS_SIGNAL| {
                    let mut df = PUMPDUMP::default();
                    df.set_th_min(*setting.kwargs_f64.get("th_min").unwrap_or(&df.th_min));
                    df.set_th_max(*setting.kwargs_f64.get("th_max").unwrap_or(&df.th_max));
                    df.set_limit(*setting.kwargs_f64.get("limit").unwrap_or(&df.limit));
                    df.set_index_min(
                        *setting
                            .kwargs_usize
                            .get("index_min")
                            .unwrap_or(&df.index_min),
                    );
                    df.set_index_max(
                        *setting
                            .kwargs_usize
                            .get("index_max")
                            .unwrap_or(&df.index_max),
                    );
                    df.set_index_normal(
                        *setting
                            .kwargs_usize
                            .get("index_normal")
                            .unwrap_or(&df.index_normal),
                    );
                    Box::new(df) as Box<dyn SignalsReady>
                }) as fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsReady>,
            ),
            (
                "set_probability",
                (|_: &SETTINGS_SIGNAL| Box::new(SET_PROBABILITY::new()) as Box<dyn SignalsReady>),
            ),
            (
                "change_signal",
                (|_: &SETTINGS_SIGNAL| Box::new(CHANGE_SIGNAL::new()) as Box<dyn SignalsReady>),
            ),
            (
                "change_src",
                (|setting: &SETTINGS_SIGNAL| {
                    let mut df = CHANGE_SRC::default();
                    df.set_signal_short(
                        *setting
                            .kwargs_f64
                            .get("signal_short")
                            .unwrap_or(&df.signal_short),
                    );
                    df.set_signal_long(
                        *setting
                            .kwargs_f64
                            .get("signal_long")
                            .unwrap_or(&df.signal_long),
                    );
                    df.set_signal_hold(
                        *setting
                            .kwargs_f64
                            .get("signal_hold")
                            .unwrap_or(&df.signal_hold),
                    );
                    Box::new(df) as Box<dyn SignalsReady>
                }),
            ),
            (
                "convert",
                (|_: &SETTINGS_SIGNAL| Box::new(CONVERT::new()) as Box<dyn SignalsReady>),
            ),
            (
                "invert",
                (|setting: &SETTINGS_SIGNAL| {
                    let mut df = INVERT::default();
                    df.set_signal_short(
                        *setting
                            .kwargs_f64
                            .get("signal_short")
                            .unwrap_or(&df.signal_short),
                    );
                    df.set_signal_long(
                        *setting
                            .kwargs_f64
                            .get("signal_long")
                            .unwrap_or(&df.signal_long),
                    );
                    df.set_signal_hold(
                        *setting
                            .kwargs_f64
                            .get("signal_hold")
                            .unwrap_or(&df.signal_hold),
                    );
                    Box::new(df) as Box<dyn SignalsReady>
                }),
            ),
            (
                "filter",
                (|_: &SETTINGS_SIGNAL| Box::new(FILTER::new()) as Box<dyn SignalsReady>),
            ),
            (
                "copy",
                (|_: &SETTINGS_SIGNAL| Box::new(COPY::new()) as Box<dyn SignalsReady>),
            ),
            (
                "repeat",
                (|setting: &SETTINGS_SIGNAL| {
                    let mut df = REPEAT::default();
                    df.set_value_signal(
                        *setting
                            .kwargs_f64
                            .get("value_signal")
                            .unwrap_or(&df.value_signal),
                    );
                    df.set_value_probability(
                        *setting
                            .kwargs_f64
                            .get("value_probability")
                            .unwrap_or(&df.value_probability),
                    );
                    Box::new(df) as Box<dyn SignalsReady>
                }),
            ),
        ])
    }
});

pub fn get_signals_arg_from_settings<'a>(
    used_signals: &Vec<String>,
    order_used_signals: &Vec<usize>,
    settings_signals: &SETTINGS_SIGNALS,
    settings_indicators: &SETTINGS_INDS,
    src: &SRC_TRANSPOSE,
    map_signals: &MAP<&'a str, Box<dyn SignalsReady>>,
    map_indicators: &MAP<&'a str, Box<dyn Indicator>>,
) -> Vec<Vec<Signal>> {
    let mut res = vec![];
    for used_signal in used_signals {
        res.push(map_signals[used_signal.as_str()].signals_vec(
            &get_in_from_settings(
                &settings_signals[used_signal].used_ind,
                &settings_signals[used_signal].used_src,
                &settings_signals[used_signal].order_used_src,
                settings_indicators,
                src,
                map_indicators,
            ),
            &get_signals_arg_from_settings(
                &settings_signals[used_signal].used_signals,
                &settings_signals[used_signal].order_used_signals,
                settings_signals,
                settings_indicators,
                src,
                map_signals,
                map_indicators,
            ),
        ));
    }
    if !order_used_signals.is_empty() {
        res = order_used_signals.iter().map(|i| res[*i].clone()).collect();
    }
    if !res.is_empty() {
        let min_len = res
            .iter()
            .map(|v| v.len())
            .min()
            .expect("this is nan or wtf");
        res = res
            .into_iter()
            .map(|v| v[v.len() - min_len..].to_vec())
            .collect::<Vec<Vec<Signal>>>();
        return (0..min_len)
            .map(|i| res.iter().map(|v1| v1[i].clone()).collect::<Vec<Signal>>())
            .collect::<Vec<Vec<Signal>>>();
    }
    Default::default()
}

pub fn get_signals_from_settings_without_bf<'a>(
    settings: &'a SETTINGS_SIGNALS,
    funcs_extract_args: &MAP<&'a str, fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsReady>>,
) -> MAP<&'a str, Box<dyn SignalsReady>> {
    settings
        .iter()
        .map(|(signal_name, settings_signal)| {
            let signal = funcs_extract_args[settings_signal.key.as_str()](settings_signal);
            (signal_name.as_str(), signal)
        })
        .collect()
}

pub fn get_signals_from_settings<'a>(
    settings_signals: &'a SETTINGS_SIGNALS,
    settings_indicators: &'a SETTINGS_INDS,
    funcs_extract_args: &MAP<&'a str, fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsReady>>,
    in_: &SRC_TRANSPOSE,
    map_signals: &MAP<&'a str, Box<dyn SignalsReady>>,
    map_indicators: &MAP<&'a str, Box<dyn Indicator>>,
) -> MAP<&'a str, (BF_SIGNALS<'a>, Box<dyn SignalsReady>)> {
    settings_signals
        .iter()
        .map(|(signal_name, settings_signal)| {
            let signal = funcs_extract_args[settings_signal.key.as_str()](settings_signal);
            let src = &in_
                .into_iter()
                .map(|v| v[..v.len() - 1].to_vec())
                .collect::<Vec<Vec<f64>>>();
            (
                signal_name.as_str(),
                (
                    signal.bf(
                        &get_in_from_settings(
                            &settings_signal.used_ind,
                            &settings_signal.used_src,
                            &settings_signal.order_used_src,
                            settings_indicators,
                            src,
                            map_indicators,
                        ),
                        &get_signals_arg_from_settings(
                            &settings_signal.used_signals,
                            &settings_signal.order_used_signals,
                            settings_signals,
                            settings_indicators,
                            src,
                            map_signals,
                            map_indicators,
                        ),
                    ),
                    signal,
                ),
            )
        })
        .collect()
}
