use std::sync::LazyLock;

use bc_indicators::indicators::ready_imports::Indicator;
use bc_signals::ready::ready_imports::*;
use bc_signals::ready::{
    change::CHANGE, convert::CONVERT, filter::FILTER, invert::INVERT, osc_mult::OSC_MULT,
    pumpdump::PUMPDUMP,
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
                (
                    "change",
                    Box::new(CHANGE::default()) as Box<dyn SignalsReady>,
                ),
                (
                    "convert",
                    Box::new(CONVERT::default()) as Box<dyn SignalsReady>,
                ),
                (
                    "filter",
                    Box::new(FILTER::default()) as Box<dyn SignalsReady>,
                ),
                (
                    "invert",
                    Box::new(INVERT::default()) as Box<dyn SignalsReady>,
                ),
                (
                    "osc_mult",
                    Box::new(OSC_MULT::default()) as Box<dyn SignalsReady>,
                ),
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
                "osc_mult",
                (|setting: &SETTINGS_SIGNAL| {
                    let mut df = OSC_MULT::default();
                    df.set_th_short(*setting.kwargs_f64.get("th_short").unwrap_or(&df.th_short));
                    df.set_th_long(*setting.kwargs_f64.get("th_long").unwrap_or(&df.th_long));
                    df.set_max_value(*setting.kwargs_f64.get("max_value").unwrap_or(&df.max_value));
                    Box::new(df) as Box<dyn SignalsReady>
                }) as fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsReady>,
            ),
            (
                "change",
                (|_: &SETTINGS_SIGNAL| Box::new(CHANGE::new()) as Box<dyn SignalsReady>)
                    as fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsReady>,
            ),
            (
                "convert",
                (|_: &SETTINGS_SIGNAL| Box::new(CONVERT::new()) as Box<dyn SignalsReady>)
                    as fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsReady>,
            ),
            (
                "invert",
                (|_: &SETTINGS_SIGNAL| Box::new(INVERT::new()) as Box<dyn SignalsReady>)
                    as fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsReady>,
            ),
            (
                "filter",
                (|_: &SETTINGS_SIGNAL| Box::new(FILTER::new()) as Box<dyn SignalsReady>)
                    as fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsReady>,
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
