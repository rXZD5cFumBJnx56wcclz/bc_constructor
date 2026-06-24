use std::slice::Iter;
use std::iter::Zip;

use bc_utils_lg::types::maps::MAP;

use crate::trade::{
    structs::{Position, TradeCell},
    utils_cell::qty_pnl,
};

pub struct StatCollector {
    pub cells: Vec<TradeCell>,
    pub src: Vec<Vec<f64>>,
}
impl StatCollector {
    pub fn push(
        &mut self,
        cell: TradeCell,
        src: Vec<f64>,
    ) {
        self.cells.push(cell);
        self.src.push(src);
    }
}

impl<'a> IntoIterator for &'a StatCollector {
    type Item = (&'a TradeCell, &'a Vec<f64>);
    type IntoIter = Zip<Iter<'a, TradeCell>, Iter<'a, Vec<f64>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter().zip(self.src.iter())
    }
}

pub trait Modificator {
    fn to_capital(&self) -> Vec<f64>;
    fn to_orders(
        &self,
        func: fn(&TradeCell) -> &MAP<String, Vec<f64>>,
    ) -> Vec<usize>;
    fn to_pnl(
        &self,
        func: fn(&MAP<String, Position>) -> &Position,
    ) -> Vec<f64>;
}

impl Modificator for StatCollector {
    fn to_capital(&self) -> Vec<f64> {
        self.cells.iter().map(|c| c.capital).collect()
    }
    fn to_orders(
        &self,
        func: fn(&TradeCell) -> &MAP<String, Vec<f64>>,
    ) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if !func(c).is_empty() {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }
    fn to_pnl(
        &self,
        func: fn(&MAP<String, Position>) -> &Position,
    ) -> Vec<f64> {
        self.into_iter()
            .map(|(c, src)| {
                let bind = c.positions.borrow();
                let p = func(&bind);
                qty_pnl(p.leverage, p.qty, p.avg_open_price, src[4], &p.position_idx)
            })
            .collect()
    }
}
