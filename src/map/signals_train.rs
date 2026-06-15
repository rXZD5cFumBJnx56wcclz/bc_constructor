use std::sync::LazyLock;

use bc_indicators::indicators::ready_imports::Indicator;
use bc_signals::train::mm::MM;
use bc_signals::train::ready_imports::*;
use bc_utils_lg::types::maps::MAP;
use bc_utils_lg::types::structures::SRC_TRANSPOSE;

use crate::map::indicators::get_in_from_settings;
use crate::settings::{SETTINGS_INDS, SETTINGS_SIGNAL, SETTINGS_SIGNALS};

pub static SIGNALS_DEFAULT: LazyLock<fn() -> MAP<&'static str, Box<dyn SignalsTrain>>> =
    LazyLock::new(|| || MAP::from_iter([("mm", Box::new(MM::default()) as Box<dyn SignalsTrain>)]));

pub static FUNCS_EXTRACT_ARGS: LazyLock<
    fn() -> MAP<&'static str, fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsTrain>>,
> = LazyLock::new(|| {
    || {
        MAP::from_iter([(
            "mm",
            (|setting: &SETTINGS_SIGNAL| {
                let mut df = MM::default();
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
                df.set_min_distance(
                    *setting
                        .kwargs_usize
                        .get("min_distance")
                        .unwrap_or(&df.min_distance),
                );
                df.set_window(*setting.kwargs_usize.get("window").unwrap_or(&df.window));
                df.set_tp_th(*setting.kwargs_f64.get("tp_th").unwrap_or(&df.tp_th));
                df.set_tp_limit(*setting.kwargs_f64.get("tp_limit").unwrap_or(&df.tp_limit));
                df.set_signal_hold(
                    *setting
                        .kwargs_f64
                        .get("signal_hold")
                        .unwrap_or(&df.signal_hold),
                );
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
                Box::new(df) as Box<dyn SignalsTrain>
            }) as fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsTrain>,
        )])
    }
});

pub fn get_signals_arg_from_settings<'a>(
    used_signals: &Vec<String>,
    order_used_signals: &Vec<usize>,
    settings_signals: &SETTINGS_SIGNALS,
    settings_indicators: &SETTINGS_INDS,
    src: &SRC_TRANSPOSE,
    map_signals: &MAP<&'a str, Box<dyn SignalsTrain>>,
    map_indicators: &MAP<&'a str, Box<dyn Indicator>>,
) -> Vec<Vec<f64>> {
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
            .collect::<Vec<Vec<f64>>>();
        return (0..min_len)
            .map(|i| res.iter().map(|v1| v1[i].clone()).collect::<Vec<f64>>())
            .collect::<Vec<Vec<f64>>>();
    }
    Default::default()
}

pub fn get_signals_from_settings_without_bf<'a>(
    settings: &'a SETTINGS_SIGNALS,
    funcs_extract_args: &MAP<&'a str, fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsTrain>>,
) -> MAP<&'a str, Box<dyn SignalsTrain>> {
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
    funcs_extract_args: &MAP<&'a str, fn(&SETTINGS_SIGNAL) -> Box<dyn SignalsTrain>>,
    in_: &SRC_TRANSPOSE,
    map_signals: &MAP<&'a str, Box<dyn SignalsTrain>>,
    map_indicators: &MAP<&'a str, Box<dyn Indicator>>,
) -> MAP<&'a str, (BF_SIGNALS<'a>, Box<dyn SignalsTrain>)> {
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
