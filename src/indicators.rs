use bc_indicators::ready_imports::{BF_INDICATOR, Indicator};
use bc_utils_lg::types::{
    maps::{FUNCS_EXTRACT_ARGS_TYPE, MAP},
    structures::SRC_TRANSPOSE,
};
use rustc_hash::FxHashMap;

use bc_utils_lg::structs::settings::{SETTINGS_IND, SETTINGS_INDS, SETTINGS_USED_SRC};

pub fn get_w_max(
    s: &SETTINGS_INDS,
    funcs_extract_args: &FxHashMap<&str, fn(&SETTINGS_IND) -> Box<dyn Indicator>>,
) -> usize {
    get_indicators_from_settings_without_bf(s, funcs_extract_args)
        .values()
        .map(|v| v.w())
        .max()
        .unwrap()
}

pub fn get_in_from_settings<'a>(
    used_ind: &Vec<String>,
    used_src: &Vec<SETTINGS_USED_SRC>,
    procedure_used: &Vec<usize>,
    settings: &SETTINGS_INDS,
    src: &[Vec<f64>],
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
                &settings[used_ind_el].procedure_used,
                settings,
                src,
                map_indicators,
            ),
        ));
    }
    if !procedure_used.is_empty() {
        res = procedure_used.iter().map(|i| res[*i].clone()).collect();
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
                        &settings_indicator.procedure_used,
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
    pub fn update_bf(
        &mut self,
        in_: &[Vec<f64>],
        s: &'a SETTINGS_INDS,
        fa: &FUNCS_EXTRACT_ARGS_TYPE<SETTINGS_IND, Box<dyn Indicator>>,
    ) {
        self.indicators = get_indicators_from_settings(s, fa, in_, &self.indicators_without_bf);
    }
}

#[derive(Default)]
pub struct IndicatorsGateway<'a> {
    pub indicators: *const Indicators<'a>,
    pub settings: *const SETTINGS_INDS,
}

impl<'a> IndicatorsGateway<'a> {
    pub fn new(
        indicators: *const Indicators<'a>,
        settings: &'a SETTINGS_INDS,
    ) -> Self {
        Self { indicators, settings }
    }
    pub fn indications_series(
        &self,
        buffer_in: &[Vec<f64>],
    ) -> MAP<&'a str, f64> {
        unsafe { &*self.settings }
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
                if setting.1.procedure_used.len() != 0 {
                    src_arg = setting
                        .1
                        .procedure_used
                        .iter()
                        .map(|i| src_arg[*i])
                        .collect();
                }
                let indicator = unsafe { &(&(*self.indicators).indicators)[key_uniq_str] };
                map.insert(
                    key_uniq_str,
                    indicator.1.ind_with_bf(src_arg.as_slice(), &indicator.0, 0),
                );
                map
            })
    }
    pub fn indications_vec(
        &self,
        src: &[Vec<f64>],
    ) -> MAP<&'a str, Vec<f64>> {
        unsafe { &*self.settings }
            .iter()
            .map(|(k, setting)| {
                let key_uniq = k.as_str();
                let indicator = unsafe { &(&(*self.indicators).indicators)[key_uniq] };
                (
                    key_uniq,
                    indicator.1.ind_vec(&get_in_from_settings(
                        &setting.used_ind,
                        &setting.used_src,
                        &setting.procedure_used,
                        unsafe { &*self.settings },
                        src,
                        unsafe { &(*self.indicators).indicators_without_bf },
                    )),
                )
            })
            .collect()
    }
}
