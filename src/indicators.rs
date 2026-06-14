use bc_indicators::indicators::ready_imports::{BF_INDICATOR, Indicator};
use bc_utils_lg::types::{maps::MAP, structures::SRC_TRANSPOSE};

use crate::map::indicators::get_in_from_settings;
use crate::settings::SETTINGS_INDS;

pub struct IndicatorsGateway<'a> {
    pub indicators: &'a MAP<&'a str, (BF_INDICATOR<'a>, Box<dyn Indicator>)>,
    pub indicators_without_bf: &'a MAP<&'a str, Box<dyn Indicator>>,
    pub settings: &'a SETTINGS_INDS,
}

impl<'a> IndicatorsGateway<'a> {
    pub fn new(
        indicators: &'a MAP<&'a str, (BF_INDICATOR<'a>, Box<dyn Indicator>)>,
        indicators_without_bf: &'a MAP<&'a str, Box<dyn Indicator>>,
        settings: &'a SETTINGS_INDS,
    ) -> Self {
        Self { indicators, indicators_without_bf, settings }
    }
    pub fn get_indications_from_settings(
        &self,
        buffer_in: &SRC_TRANSPOSE,
    ) -> MAP<&'a str, f64> {
        self.settings
            .iter()
            .fold(MAP::default(), |mut map, setting| {
                let key_uniq_str = setting.0.as_str();
                let mut src_arg = vec![];
                for us_el in &setting.1.used_src {
                    src_arg.push({
                        let sk = &buffer_in[us_el.index];
                        sk[sk.len() - 1 - us_el.sub_from_last_i]
                    });
                }
                for ui_el in &setting.1.used_ind {
                    src_arg.push(map[ui_el.as_str()]);
                }
                if setting.1.order_used.len() != 0 {
                    src_arg = setting.1.order_used.iter().map(|i| src_arg[*i]).collect();
                }
                let indicator = &self.indicators[key_uniq_str];
                map.insert(
                    key_uniq_str,
                    indicator.1.ind_with_bf(src_arg.as_slice(), &indicator.0, 0),
                );
                map
            })
    }
    pub fn get_indications_vec_from_settings(
        &self,
        src: &SRC_TRANSPOSE,
    ) -> MAP<&'a str, Vec<f64>> {
        self.settings
            .iter()
            .map(|(k, setting)| {
                let key_uniq = k.as_str();
                let indicator = &self.indicators[key_uniq];
                (
                    key_uniq,
                    indicator.1.ind_vec(&get_in_from_settings(
                        &setting.used_ind,
                        &setting.used_src,
                        &setting.order_used,
                        &self.settings,
                        src,
                        &self.indicators_without_bf,
                    )),
                )
            })
            .collect()
    }
}
