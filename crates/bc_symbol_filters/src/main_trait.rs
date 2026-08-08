use std::any::Any;

pub trait SymbolFilter: Any {
    fn symbol_filter(&self, src: &[Vec<f64>], ind_values: &[f64]) -> bool;
}
