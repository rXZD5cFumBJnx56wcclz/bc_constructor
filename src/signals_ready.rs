use bc_indicators::indicators::ready_imports::Indicator;
use bc_signals::ready::ready_imports::*;
use bc_signals::ready::ready_trait::SignalReady;
use bc_utils_lg::{
    settings::SETTINGS,
    types::{
        maps::{FUNCS_EXTRACT_ARGS_TYPE, MAP},
        structures::SRC_TRANSPOSE,
    },
};

use crate::{
    buffer::Buffer,
    indicators::{Indicators, get_in_from_settings},
};
use bc_utils_lg::settings::{SETTINGS_INDS, SETTINGS_SIGNAL, SETTINGS_SIGNALS};

pub fn get_signals_arg_from_settings<'a>(
    used_signals: &Vec<String>,
    order_used_signals: &Vec<usize>,
    settings_signals: &SETTINGS_SIGNALS,
    settings_indicators: &SETTINGS_INDS,
    src: &SRC_TRANSPOSE,
    map_signals: &MAP<&'a str, Box<dyn SignalReady>>,
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
    funcs_extract_args: &MAP<&'a str, fn(&SETTINGS_SIGNAL) -> Box<dyn SignalReady>>,
) -> MAP<&'a str, Box<dyn SignalReady>> {
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
    funcs_extract_args: &MAP<&'a str, fn(&SETTINGS_SIGNAL) -> Box<dyn SignalReady>>,
    in_: &[Vec<f64>],
    map_signals: &MAP<&'a str, Box<dyn SignalReady>>,
    map_indicators: &MAP<&'a str, Box<dyn Indicator>>,
) -> MAP<&'a str, (BF_SIGNALS<'a>, Box<dyn SignalReady>)> {
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

#[derive(Default)]
pub struct SignalsReady<'a> {
    signals_ready_without_bf: MAP<&'a str, Box<dyn SignalReady>>,
    signals_ready: MAP<&'a str, (BF_SIGNALS<'a>, Box<dyn SignalReady>)>,
}

impl<'a> SignalsReady<'a> {
    pub fn new(
        s_signals_ready: &'a SETTINGS_SIGNALS,
        s_indicators: &'a SETTINGS_INDS,
        funcs_extract_args: &MAP<&'a str, fn(&SETTINGS_SIGNAL) -> Box<dyn SignalReady>>,
        in_: &[Vec<f64>],
        map_indicators: &MAP<&'a str, Box<dyn Indicator>>,
    ) -> Self {
        let signals_ready_without_bf =
            get_signals_from_settings_without_bf(s_signals_ready, funcs_extract_args);
        Self {
            signals_ready: get_signals_from_settings(
                s_signals_ready,
                s_indicators,
                funcs_extract_args,
                in_,
                &signals_ready_without_bf,
                map_indicators,
            ),
            signals_ready_without_bf: signals_ready_without_bf,
        }
    }
    pub fn update_bf<'b>(
        &mut self,
        in_: &[Vec<f64>],
        s: &'a SETTINGS,
        fa: &'b FUNCS_EXTRACT_ARGS_TYPE<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
        indicators_without_bf: &MAP<&'a str, Box<dyn Indicator>>,
    ) {
        self.signals_ready = get_signals_from_settings(
            &s.signals_ready,
            &s.indications,
            fa,
            in_,
            &self.signals_ready_without_bf,
            indicators_without_bf,
        );
    }
}

#[derive(Default)]
pub struct SignalsReadyGateway<'a> {
    pub signals_ready: *const SignalsReady<'a>,
    pub indicators: *const Indicators<'a>,
    pub settings_signals: *const SETTINGS_SIGNALS,
    pub settings_indicators: *const SETTINGS_INDS,
}

impl<'a> SignalsReadyGateway<'a> {
    pub fn new(
        signals_ready: *const SignalsReady<'a>,
        indicators: *const Indicators<'a>,
        settings_signals: *const SETTINGS_SIGNALS,
        settings_indicators: *const SETTINGS_INDS,
    ) -> Self {
        Self {
            signals_ready,
            indicators,
            settings_signals,
            settings_indicators,
        }
    }
    pub fn signals_series(
        &self,
        indications: &MAP<&'a str, f64>,
        buffer_in: &Buffer,
    ) -> MAP<&'a str, Signal> {
        unsafe { &*self.settings_signals }
            .iter()
            .fold(MAP::default(), |mut map, setting| {
                let key_uniq_str = setting.0.as_str();
                let mut src_arg = vec![];
                let mut signals_arg = vec![];
                for src_arg_el in &setting.1.used_src {
                    src_arg.push({
                        let sk = &buffer_in[src_arg_el.index];
                        sk[sk.len() - 1 - src_arg_el.sub_from_last_i]
                    });
                }
                for ind_arg_el in &setting.1.used_ind {
                    src_arg.push(indications[ind_arg_el.as_str()]);
                }
                for signals_arg_el in &setting.1.used_signals {
                    signals_arg.push(map[signals_arg_el.as_str()].clone());
                }
                if !setting.1.order_used_src.is_empty() {
                    src_arg = setting
                        .1
                        .order_used_src
                        .iter()
                        .map(|i| src_arg[*i])
                        .collect();
                }
                if !setting.1.order_used_signals.is_empty() {
                    src_arg = setting
                        .1
                        .order_used_signals
                        .iter()
                        .map(|i| src_arg[*i])
                        .collect();
                }
                let signal = unsafe { &(&(*self.signals_ready).signals_ready)[key_uniq_str] };
                map.insert(
                    key_uniq_str,
                    signal
                        .1
                        .signal_with_bf(&src_arg, &signals_arg, &signal.0, 0),
                );
                map
            })
    }
    pub fn signals_vec(
        &self,
        src: &SRC_TRANSPOSE,
    ) -> MAP<&'a str, Vec<Signal>> {
        unsafe { &*self.settings_signals }
            .iter()
            .map(|(k, setting)| {
                let key_uniq = k.as_str();
                let signal = unsafe { &(&(*self.signals_ready).signals_ready)[key_uniq] };
                (
                    key_uniq,
                    signal.1.signals_vec(
                        &get_in_from_settings(
                            &setting.used_ind,
                            &setting.used_src,
                            &setting.order_used_src,
                            unsafe { &*self.settings_indicators },
                            src,
                            unsafe { &(*self.indicators).indicators_without_bf },
                        ),
                        &get_signals_arg_from_settings(
                            &setting.used_signals,
                            &setting.order_used_signals,
                            unsafe { &*self.settings_signals },
                            unsafe { &*self.settings_indicators },
                            src,
                            unsafe { &(*self.signals_ready).signals_ready_without_bf },
                            unsafe { &(*self.indicators).indicators_without_bf },
                        ),
                    ),
                )
            })
            .collect()
    }
}
