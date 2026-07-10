use bc_symbol_filters::main_trait::SymbolFilter;
use bc_utils::other::{transpose, vec_len_sync_set};
use bc_utils_lg::{
    structs::settings::{SETTINGS_SYMBOL_FILTER, SETTINGS_SYMBOL_FILTERS},
    types::maps::{FUNCS_EXTRACT_ARGS_TYPE, MAP},
};

fn get_src(
    s: &SETTINGS_SYMBOL_FILTER,
    src: &[Vec<f64>],
    ind: &MAP<&str, Vec<f64>>,
    ind_columns: &MAP<&str, Vec<f64>>,
) -> Vec<Vec<f64>> {
    let mut res = vec![];
    let src_len = src.first().unwrap_or(&vec![]).len();
    let ind_len = ind.values().next().unwrap_or(&vec![]).len();
    let ind_columns_len = ind_columns.values().next().unwrap_or(&vec![]).len();

    for used_src in s.used_src.iter() {
        res.push(src[used_src.index][..src_len - used_src.sub_from_last_i].to_vec());
    }
    for used_ind in s.used_ind.iter() {
        res.push(ind[used_ind.key.as_str()][..ind_len - used_ind.sub_from_last_i].to_vec());
    }
    for used_ind_stat_columns in s.used_ind_stat_columns.iter() {
        res.push(
            ind_columns[used_ind_stat_columns.key.as_str()]
                [..ind_columns_len - used_ind_stat_columns.sub_from_last_i]
                .to_vec(),
        );
    }
    if !res.is_empty() {
        vec_len_sync_set(&mut res);
        return transpose(res);
    }
    Default::default()
}

fn get_values(
    s: &SETTINGS_SYMBOL_FILTER,
    ind_values: &MAP<&str, f64>,
) -> Vec<f64> {
    let mut res = vec![];
    for used_ind_stat_values in s.used_ind_stat_values.iter() {
        res.push(ind_values[used_ind_stat_values.as_str()]);
    }
    res
}

pub fn get_map_from_settings<'a>(
    s: &'a SETTINGS_SYMBOL_FILTERS,
    fa: &FUNCS_EXTRACT_ARGS_TYPE<SETTINGS_SYMBOL_FILTER, Box<dyn SymbolFilter>>,
) -> MAP<&'a str, Box<dyn SymbolFilter>> {
    s.iter()
        .map(|setting| (setting.key.as_str(), fa[setting.key.as_str()](setting)))
        .collect()
}

pub struct SymbolFiltersGateway<'a> {
    pub symbol_filters: &'a MAP<&'a str, Box<dyn SymbolFilter>>,
    pub s: &'a SETTINGS_SYMBOL_FILTERS,
}

impl<'a> SymbolFiltersGateway<'a> {
    pub fn new(
        symbol_filters: &'a MAP<&'a str, Box<dyn SymbolFilter>>,
        s: &'a SETTINGS_SYMBOL_FILTERS,
    ) -> Self {
        Self { symbol_filters, s }
    }
}

impl SymbolFiltersGateway<'_> {
    pub fn symbol_filters(
        &self,
        src: &[Vec<f64>],
        ind: &MAP<&str, Vec<f64>>,
        ind_columns: &MAP<&str, Vec<f64>>,
        ind_values: &MAP<&str, f64>,
        fa: &FUNCS_EXTRACT_ARGS_TYPE<SETTINGS_SYMBOL_FILTER, Box<dyn SymbolFilter>>,
    ) -> bool {
        self.s
            .iter()
            .map(|setting| {
                let symbol_filter = fa[setting.key.as_str()](setting);
                symbol_filter.symbol_filter(
                    &get_src(setting, src, ind, ind_columns),
                    &get_values(setting, ind_values),
                )
            })
            .all(|v| v)
    }

    pub fn symbols_filters(
        &self,
        src: &MAP<String, Vec<Vec<f64>>>,
        ind: &MAP<String, MAP<&str, Vec<f64>>>,
        ind_columns: &MAP<String, MAP<&str, Vec<f64>>>,
        ind_values: &MAP<String, MAP<&str, f64>>,
        fa: &FUNCS_EXTRACT_ARGS_TYPE<SETTINGS_SYMBOL_FILTER, Box<dyn SymbolFilter>>,
    ) -> Vec<bool> {
        src.iter()
            .map(|(symbol, src_value)| {
                self.symbol_filters(
                    src_value,
                    &ind[symbol],
                    &ind_columns[symbol],
                    &ind_values[symbol],
                    fa,
                )
            })
            .collect()
    }
    pub fn symbol_filters_added(
        &self,
        src: &[Vec<f64>],
        ind: &MAP<&str, Vec<f64>>,
        ind_columns: &MAP<&str, Vec<f64>>,
        ind_values: &MAP<&str, f64>,
        fa: &FUNCS_EXTRACT_ARGS_TYPE<SETTINGS_SYMBOL_FILTER, Box<dyn SymbolFilter>>,
        symbol: &str,
    ) -> Option<String> {
        if self.symbol_filters(src, ind, ind_columns, ind_values, fa) {
            Some(symbol.to_string())
        } else {
            None
        }
    }
    pub fn symbols_filters_added(
        &self,
        src: &MAP<String, Vec<Vec<f64>>>,
        ind: &MAP<String, MAP<&str, Vec<f64>>>,
        ind_columns: &MAP<String, MAP<&str, Vec<f64>>>,
        ind_values: &MAP<String, MAP<&str, f64>>,
        fa: &FUNCS_EXTRACT_ARGS_TYPE<SETTINGS_SYMBOL_FILTER, Box<dyn SymbolFilter>>,
    ) -> Vec<String> {
        src.iter()
            .filter_map(|(k, v)| {
                self.symbol_filters_added(v, &ind[k], &ind_columns[k], &ind_values[k], fa, k)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::LazyLock;

    use bc_pack_symbol_filters::FUNCS_EXTRACT_ARGS as FA;
    use bc_utils_lg::statics::prices::SRC;
    use pretty_assertions::assert_eq as assert_eq_pr;

    static S: LazyLock<SETTINGS_SYMBOL_FILTERS> = LazyLock::new(|| {
        SETTINGS_SYMBOL_FILTERS::from_iter([SETTINGS_SYMBOL_FILTER {
            key: "ordering".to_string(),
            kwargs_f64: MAP::from_iter([("value".to_string(), 1.)]),
            used_ind_stat_values: vec!["value".to_string()],
            ..Default::default()
        }])
    });

    #[test]
    fn symbol_filters_res_1() {
        let bind = get_map_from_settings(&S, &FA());
        let gw = SymbolFiltersGateway::new(&bind, &S);
        assert_eq_pr!(
            gw.symbol_filters(
                &SRC,
                &Default::default(),
                &Default::default(),
                &MAP::from_iter([("value", 0.9)]),
                &FA()
            ),
            true
        );
    }

    #[test]
    fn symbols_filters_res_1() {
        let bind = get_map_from_settings(&S, &FA());
        let gw = SymbolFiltersGateway::new(&bind, &S);
        assert_eq_pr!(
            vec![true, true],
            gw.symbols_filters(
                &MAP::from_iter([
                    ("1".to_string(), Default::default()),
                    ("2".to_string(), Default::default()),
                ]),
                &MAP::from_iter([
                    ("1".to_string(), Default::default()),
                    ("2".to_string(), Default::default()),
                ]),
                &MAP::from_iter([
                    ("1".to_string(), Default::default()),
                    ("2".to_string(), Default::default()),
                ]),
                &MAP::from_iter([
                    ("1".to_string(), MAP::from_iter([("value", 0.9)])),
                    ("2".to_string(), MAP::from_iter([("value", 0.9)]))
                ]),
                &FA()
            )
        )
    }

    #[test]
    fn symbol_filters_added_res_1() {
        let bind = get_map_from_settings(&S, &FA());
        let gw = SymbolFiltersGateway::new(&bind, &S);
        assert_eq_pr!(
            gw.symbol_filters_added(
                &SRC,
                &Default::default(),
                &Default::default(),
                &MAP::from_iter([("value", 0.9)]),
                &FA(),
                "1",
            ),
            Some("1".to_string())
        );
    }

    #[test]
    fn symbols_filters_added_res_1() {
        let bind = get_map_from_settings(&S, &FA());
        let gw = SymbolFiltersGateway::new(&bind, &S);
        assert_eq_pr!(
            vec!["1".to_string(), "2".to_string()],
            gw.symbols_filters_added(
                &MAP::from_iter([
                    ("1".to_string(), Default::default()),
                    ("2".to_string(), Default::default()),
                ]),
                &MAP::from_iter([
                    ("1".to_string(), Default::default()),
                    ("2".to_string(), Default::default()),
                ]),
                &MAP::from_iter([
                    ("1".to_string(), Default::default()),
                    ("2".to_string(), Default::default()),
                ]),
                &MAP::from_iter([
                    ("1".to_string(), MAP::from_iter([("value", 0.9)])),
                    ("2".to_string(), MAP::from_iter([("value", 0.9)]))
                ]),
                &FA()
            )
        )
    }
}
