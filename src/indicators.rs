use bc_indicators::indicators::ready_imports::{BF_INDICATOR, Indicator};
use bc_utils_lg::types::{maps::MAP, structures::SRC_TRANSPOSE};
use rustc_hash::FxHashMap;

use crate::settings::{SETTINGS_IND, SETTINGS_INDS, SETTINGS_USED_SRC};

pub fn get_in_from_settings<'a>(
    used_ind: &Vec<String>,
    used_src: &Vec<SETTINGS_USED_SRC>,
    order_used: &Vec<usize>,
    settings: &SETTINGS_INDS,
    src: &SRC_TRANSPOSE,
    map_indicators: &MAP<&'a str, Box<dyn Indicator>>,
) -> Vec<Vec<f64>> {
    let mut res = vec![];
    for used_src_el in used_src {
        res.push({
            let sk = &src[used_src_el.index];
            sk[..sk.len() - used_src_el.sub_from_last_i].to_vec()
        });
    }
    for used_ind_el in used_ind {
        res.push(map_indicators[used_ind_el.as_str()].ind_vec(
            // recursive func
            &get_in_from_settings(
                &settings[used_ind_el].used_ind,
                &settings[used_ind_el].used_src,
                &settings[used_ind_el].order_used,
                settings,
                src,
                map_indicators,
            ),
        ));
    }
    if !order_used.is_empty() {
        res = order_used.iter().map(|i| res[*i].clone()).collect();
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
            .map(|v| res.iter().map(|v1| v1[v]).collect::<Vec<f64>>())
            .collect::<Vec<Vec<f64>>>();
    }
    Default::default()
}

pub fn get_indicators_from_settings_without_bf<'a>(
    settings: &'a SETTINGS_INDS,
    funcs_extract_args: &FxHashMap<&'a str, fn(&SETTINGS_IND) -> Box<dyn Indicator>>,
) -> MAP<&'a str, Box<dyn Indicator>> {
    settings
        .iter()
        .map(|(indicator_name, settings_indicator)| {
            let indicator = funcs_extract_args[settings_indicator.key.as_str()](settings_indicator);
            (indicator_name.as_str(), indicator)
        })
        .collect()
}

pub fn get_indicators_from_settings<'a>(
    settings: &'a SETTINGS_INDS,
    funcs_extract_args: &FxHashMap<&'a str, fn(&SETTINGS_IND) -> Box<dyn Indicator>>,
    in_: &[Vec<f64>],
    map_indicators: &MAP<&'a str, Box<dyn Indicator>>,
) -> MAP<&'a str, (BF_INDICATOR<'a>, Box<dyn Indicator>)> {
    settings
        .iter()
        .map(|(indicator_name, settings_indicator)| {
            let indicator = funcs_extract_args[settings_indicator.key.as_str()](settings_indicator);
            (
                indicator_name.as_str(),
                (
                    indicator.bf(&get_in_from_settings(
                        &settings_indicator.used_ind,
                        &settings_indicator.used_src,
                        &settings_indicator.order_used,
                        settings,
                        &in_.into_iter()
                            .map(|v| v[..v.len() - 1].to_vec())
                            .collect::<Vec<Vec<f64>>>(),
                        map_indicators,
                    )),
                    indicator,
                ),
            )
        })
        .collect()
}

#[derive(Default)]
pub struct Indicators<'a> {
    pub indicators_without_bf: MAP<&'a str, Box<dyn Indicator>>,
    pub indicators: MAP<&'a str, (BF_INDICATOR<'a>, Box<dyn Indicator>)>,
}

impl<'a> Indicators<'a> {
    pub fn new(
        settings: &'a SETTINGS_INDS,
        funcs_extract_args: &FxHashMap<&'a str, fn(&SETTINGS_IND) -> Box<dyn Indicator>>,
        in_: &[Vec<f64>],
    ) -> Self {
        let ind_without_bf = get_indicators_from_settings_without_bf(settings, funcs_extract_args);
        Self {
            indicators: get_indicators_from_settings(
                settings,
                funcs_extract_args,
                in_,
                &ind_without_bf,
            ),
            indicators_without_bf: ind_without_bf,
        }
    }
}

pub struct IndicatorsGateway<'a> {
    pub indicators: &'a Indicators<'a>,
    pub settings: &'a SETTINGS_INDS,
}

impl<'a> IndicatorsGateway<'a> {
    pub fn new(
        indicators: &'a Indicators<'a>,
        settings: &'a SETTINGS_INDS,
    ) -> Self {
        Self { indicators, settings }
    }
    pub fn indications_series(
        &self,
        buffer_in: &[Vec<f64>],
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
                let indicator = &self.indicators.indicators[key_uniq_str];
                map.insert(
                    key_uniq_str,
                    indicator.1.ind_with_bf(src_arg.as_slice(), &indicator.0, 0),
                );
                map
            })
    }
    pub fn indications_vec(
        &self,
        src: &SRC_TRANSPOSE,
    ) -> MAP<&'a str, Vec<f64>> {
        self.settings
            .iter()
            .map(|(k, setting)| {
                let key_uniq = k.as_str();
                let indicator = &self.indicators.indicators[key_uniq];
                (
                    key_uniq,
                    indicator.1.ind_vec(&get_in_from_settings(
                        &setting.used_ind,
                        &setting.used_src,
                        &setting.order_used,
                        &self.settings,
                        src,
                        &self.indicators.indicators_without_bf,
                    )),
                )
            })
            .collect()
    }
}
