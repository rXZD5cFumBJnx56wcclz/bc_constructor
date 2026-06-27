#![allow(non_camel_case_types)]

use std::sync::LazyLock;

use bc_signals::train::mm::MM;
use bc_signals::train::ready_imports::*;
use bc_utils_lg::types::maps::MAP;

use crate::settings::SETTINGS_SIGNAL;

pub static SIGNALS_DEFAULT: LazyLock<fn() -> MAP<&'static str, Box<dyn SignalTrain>>> =
    LazyLock::new(|| || MAP::from_iter([("mm", Box::new(MM::default()) as Box<dyn SignalTrain>)]));

pub type FUNCS_EXTRACT_ARGS_TYPE = MAP<&'static str, fn(&SETTINGS_SIGNAL) -> Box<dyn SignalTrain>>;

pub static FUNCS_EXTRACT_ARGS: LazyLock<fn() -> FUNCS_EXTRACT_ARGS_TYPE> = LazyLock::new(|| {
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
                Box::new(df) as Box<dyn SignalTrain>
            }) as fn(&SETTINGS_SIGNAL) -> Box<dyn SignalTrain>,
        )])
    }
});
