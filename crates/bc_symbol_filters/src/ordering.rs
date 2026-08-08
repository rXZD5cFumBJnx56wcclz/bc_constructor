use crate::prelude::*;

pub struct ORDERING {
    pub value: f64,
    pub type_: String,
}

impl ORDERING {
    pub fn new(value: f64, type_: String) -> Self {
        Self { value, type_ }
    }
}

impl Default for ORDERING {
    fn default() -> Self {
        Self {
            value: Default::default(),
            type_: "less".to_string(),
        }
    }
}

impl SymbolFilter for ORDERING {
    fn symbol_filter(&self, _: &[Vec<f64>], ind_values: &[f64]) -> bool {
        let bind = self.type_.as_str();
        let ind_value = ind_values[0];
        match bind {
            "less" => ind_value < self.value,
            "equal" => ind_value == self.value,
            "greater" => ind_value < self.value,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[test]
    fn symbol_filter_res_1() {
        assert_eq_pr!(
            ORDERING::new(1., "less".to_string()).symbol_filter(&[], &[0.9]),
            true
        )
    }
}
