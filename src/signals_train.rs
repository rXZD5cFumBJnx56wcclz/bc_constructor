use bc_indicators::indicators::ready_imports::{BF_INDICATOR, Indicator};
use bc_signals::train::ready_imports::*;
use bc_utils_lg::types::{maps::MAP, structures::SRC_TRANSPOSE};

use crate::map::indicators::get_in_from_settings;
use crate::map::signals_train::get_signals_arg_from_settings;
use crate::settings::{SETTINGS_INDS, SETTINGS_SIGNALS};

pub struct SignalsTrainGateway<'a> {
    pub signals_ready: &'a MAP<&'a str, (BF_SIGNALS<'a>, Box<dyn SignalsTrain>)>,
    pub indicators: &'a MAP<&'a str, (BF_INDICATOR<'a>, Box<dyn Indicator>)>,
    pub signals_ready_without_bf: &'a MAP<&'a str, Box<dyn SignalsTrain>>,
    pub indicators_without_bf: &'a MAP<&'a str, Box<dyn Indicator>>,
    pub settings_signals: &'a SETTINGS_SIGNALS,
    pub settings_indicators: &'a SETTINGS_INDS,
}

impl<'a> SignalsTrainGateway<'a> {
    pub fn new(
        signals_ready: &'a MAP<&'a str, (BF_SIGNALS<'a>, Box<dyn SignalsTrain>)>,
        indicators: &'a MAP<&'a str, (BF_INDICATOR<'a>, Box<dyn Indicator>)>,
        signals_ready_without_bf: &'a MAP<&'a str, Box<dyn SignalsTrain>>,
        indicators_without_bf: &'a MAP<&'a str, Box<dyn Indicator>>,
        settings_signals: &'a SETTINGS_SIGNALS,
        settings_indicators: &'a SETTINGS_INDS,
    ) -> Self {
        Self {
            signals_ready,
            indicators,
            signals_ready_without_bf,
            indicators_without_bf,
            settings_signals,
            settings_indicators,
        }
    }
    pub fn get_signals_from_settings(
        &self,
        indications: &MAP<&'a str, f64>,
        buffer_in: &SRC_TRANSPOSE,
    ) -> MAP<&'a str, f64> {
        self.settings_signals
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
                let signal = &self.signals_ready[key_uniq_str];
                map.insert(
                    key_uniq_str,
                    signal
                        .1
                        .signal_with_bf(&src_arg, &signals_arg, &signal.0, 0),
                );
                map
            })
    }
    pub fn get_signals_vec_from_settings(
        &self,
        src: &SRC_TRANSPOSE,
    ) -> MAP<&'a str, Vec<f64>> {
        self.settings_signals
            .iter()
            .map(|(k, setting)| {
                let key_uniq = k.as_str();
                let signal = &self.signals_ready[key_uniq];
                (
                    key_uniq,
                    signal.1.signals_vec(
                        &get_in_from_settings(
                            &setting.used_ind,
                            &setting.used_src,
                            &setting.order_used_src,
                            self.settings_indicators,
                            src,
                            self.indicators_without_bf,
                        ),
                        &get_signals_arg_from_settings(
                            &setting.used_signals,
                            &setting.order_used_signals,
                            self.settings_signals,
                            self.settings_indicators,
                            src,
                            self.signals_ready_without_bf,
                            self.indicators_without_bf,
                        ),
                    ),
                )
            })
            .collect()
    }
}
