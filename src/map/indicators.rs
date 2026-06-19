use std::sync::LazyLock;

use bc_indicators::indicators::ready_imports::*;
use bc_indicators::indicators::{
    avg::AVG, div::DIV, ema::EMA, minus::MINUS, mm_scaler::MM_SCALER, mult::MULT,
    osc_mult::OSC_MULT, percent::PERCENT, plus::PLUS, profit_factor::PROFIT_FACTOR, rem::REM,
    repeat::REPEAT, rma::RMA, rsi::RSI, sma::SMA, trend_ma::TREND_MA, wrap::WRAP,
};
use bc_utils_lg::types::maps::MAP;
use bc_utils_lg::types::structures::SRC_TRANSPOSE;

use crate::settings::{SETTINGS_IND, SETTINGS_INDS, SETTINGS_USED_SRC};

pub static INDICATORS_DEFAULT: LazyLock<fn() -> FxHashMap<&'static str, Box<dyn Indicator>>> =
    LazyLock::new(|| {
        || {
            FxHashMap::from_iter([
                ("avg", Box::new(AVG::default()) as Box<dyn Indicator>),
                ("div", Box::new(DIV::default())),
                ("ema", Box::new(EMA::default())),
                ("minus", Box::new(MINUS::default())),
                ("mm_scaler", Box::new(MM_SCALER::default())),
                ("mult", Box::new(MULT::default())),
                ("osc_mult", Box::new(OSC_MULT::default())),
                ("percent", Box::new(PERCENT::default())),
                ("plus", Box::new(PLUS::default())),
                ("profit_factor", Box::new(PROFIT_FACTOR::default())),
                ("rem", Box::new(REM::default())),
                ("rma", Box::new(RMA::default())),
                ("rsi", Box::new(RSI::default())),
                ("sma", Box::new(SMA::default())),
                ("trend_ma", Box::new(TREND_MA::default())),
                ("repeat", Box::new(REPEAT::default())),
                ("wrap", Box::new(WRAP::default())),
            ])
        }
    });

pub static FUNCS_EXTRACT_ARGS: LazyLock<
    fn() -> FxHashMap<&'static str, fn(&SETTINGS_IND) -> Box<dyn Indicator>>,
> = LazyLock::new(|| {
    || {
        FxHashMap::from_iter([
            (
                "rsi",
                (|v: &SETTINGS_IND| {
                    let mut df = RSI::default();
                    df.set_window(*v.kwargs_usize.get("window").unwrap_or(&df.window));
                    df.set_mult_window_accuracy(
                        *v.kwargs_usize
                            .get("mult_window_accuracy")
                            .unwrap_or(&df.mult_window_accuracy),
                    );
                    df.set_add_window_accuracy(
                        *v.kwargs_usize
                            .get("add_window_accuracy")
                            .unwrap_or(&df.add_window_accuracy),
                    );
                    Box::new(df) as Box<dyn Indicator>
                }) as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "rma",
                (|v: &SETTINGS_IND| {
                    let mut df = RMA::default();
                    df.set_window(*v.kwargs_usize.get("window").unwrap_or(&df.window));
                    df.set_mult_window_accuracy(
                        *v.kwargs_usize
                            .get("mult_window_accuracy")
                            .unwrap_or(&df.mult_window_accuracy),
                    );
                    df.set_add_window_accuracy(
                        *v.kwargs_usize
                            .get("add_window_accuracy")
                            .unwrap_or(&df.add_window_accuracy),
                    );
                    Box::new(df) as Box<dyn Indicator>
                }) as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "ema",
                (|v: &SETTINGS_IND| {
                    let mut df = EMA::default();
                    df.set_window(*v.kwargs_usize.get("window").unwrap_or(&df.window));
                    df.set_mult_window_accuracy(
                        *v.kwargs_usize
                            .get("mult_window_accuracy")
                            .unwrap_or(&df.mult_window_accuracy),
                    );
                    df.set_add_window_accuracy(
                        *v.kwargs_usize
                            .get("add_window_accuracy")
                            .unwrap_or(&df.add_window_accuracy),
                    );
                    Box::new(df) as Box<dyn Indicator>
                }) as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "sma",
                (|v: &SETTINGS_IND| {
                    let mut df = SMA::default();
                    df.set_window(*v.kwargs_usize.get("window").unwrap_or(&df.window));
                    Box::new(df) as Box<dyn Indicator>
                }) as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "avg",
                (|_: &SETTINGS_IND| Box::new(AVG::default()) as Box<dyn Indicator>)
                    as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "mm_scaler",
                (|v: &SETTINGS_IND| {
                    let mut df = MM_SCALER::default();
                    df.set_window(*v.kwargs_usize.get("window").unwrap_or(&df.window));
                    Box::new(df) as Box<dyn Indicator>
                }) as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "osc_mult",
                (|v: &SETTINGS_IND| {
                    let mut df = OSC_MULT::default();
                    df.set_th_short(
                        *v.kwargs_f64
                            .get("th_shset_th_short")
                            .unwrap_or(&df.th_short),
                    );
                    df.set_th_long(*v.kwargs_f64.get("th_set_th_long").unwrap_or(&df.th_long));
                    df.set_max_value(*v.kwargs_f64.get("max_value").unwrap_or(&df.max_value));
                    Box::new(df) as Box<dyn Indicator>
                }) as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "repeat",
                (|v: &SETTINGS_IND| {
                    let mut df = REPEAT::default();
                    df.set_value(*v.kwargs_f64.get("value").unwrap_or(&df.value));
                    Box::new(df) as Box<dyn Indicator>
                }) as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "percent",
                (|_: &SETTINGS_IND| Box::new(PERCENT::default()) as Box<dyn Indicator>)
                    as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "trend_ma",
                (|_: &SETTINGS_IND| Box::new(TREND_MA::default()) as Box<dyn Indicator>)
                    as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "minus",
                (|_: &SETTINGS_IND| Box::new(MINUS::default()) as Box<dyn Indicator>)
                    as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "plus",
                (|_: &SETTINGS_IND| Box::new(PLUS::default()) as Box<dyn Indicator>)
                    as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "mult",
                (|_: &SETTINGS_IND| Box::new(MULT::default()) as Box<dyn Indicator>)
                    as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "div",
                (|_: &SETTINGS_IND| Box::new(DIV::default()) as Box<dyn Indicator>)
                    as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "rem",
                (|_: &SETTINGS_IND| Box::new(REM::default()) as Box<dyn Indicator>)
                    as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
            (
                "wrap",
                (|_: &SETTINGS_IND| Box::new(WRAP::default()) as Box<dyn Indicator>)
                    as fn(&SETTINGS_IND) -> Box<dyn Indicator>,
            ),
        ])
    }
});

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
    in_: &SRC_TRANSPOSE,
    map_indicators: &MAP<&'a str, Box<dyn Indicator>>,
) -> MAP<&'a str, (RefCell<Vec<MAP<&'a str, Vec<f64>>>>, Box<dyn Indicator>)> {
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
