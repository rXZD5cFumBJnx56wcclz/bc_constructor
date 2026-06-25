use core::f64;
use std::slice::Iter;
use std::{cell::Ref, iter::Zip};

use bc_utils_lg::types::maps::MAP;

use crate::{
    settings::SETTINGS_STRATEGY,
    trade::{
        structs::{Order, Position, TradeCell},
        utils_cell::{price_is_real_time, qty_pnl},
    },
};

pub struct StatCollector<'a> {
    pub symbol: String,
    pub cells: Vec<TradeCell>,
    pub src: Vec<Vec<f64>>,
    s: &'a SETTINGS_STRATEGY,
}

impl<'a> StatCollector<'a> {
    pub fn new(
        symbol: String,
        s: &'a SETTINGS_STRATEGY,
    ) -> Self {
        Self { symbol, cells: Vec::new(), src: Vec::new(), s }
    }
    pub fn push(
        &mut self,
        cell: TradeCell,
        src: Vec<f64>,
    ) {
        self.cells.push(cell);
        self.src.push(src);
    }
}

impl<'a> IntoIterator for &'a StatCollector<'a> {
    type Item = (&'a TradeCell, &'a Vec<f64>);
    type IntoIter = Zip<Iter<'a, TradeCell>, Iter<'a, Vec<f64>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter().zip(self.src.iter())
    }
}

pub trait Modificator {
    fn to_capital(&self) -> Vec<f64>;
    fn to_some<T>(
        &self,
        func: fn(&TradeCell) -> Ref<MAP<String, T>>,
    ) -> Vec<f64>;
    fn to_pnl(
        &self,
        func: fn(&MAP<String, Position>) -> &Position,
    ) -> Vec<f64>;
    fn to_entry(&self) -> Vec<f64>;
    fn to_exit(&self) -> Vec<f64>;
    fn to_entry_and_exit(&self) -> Vec<f64>;
    fn to_data(&self) -> MAP<String, Vec<f64>>;
}

impl Modificator for StatCollector<'_> {
    fn to_capital(&self) -> Vec<f64> {
        self.cells.iter().map(|c| c.capital).collect()
    }
    fn to_some<T>(
        &self,
        func: fn(&TradeCell) -> Ref<MAP<String, T>>,
    ) -> Vec<f64> {
        self.into_iter()
            .map(|(c, src)| {
                if !func(c).is_empty() {
                    src[1]
                } else {
                    f64::NAN
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
    fn to_entry(&self) -> Vec<f64> {
        self.into_iter()
            .map(|(c, src)| {
                if !c.positions.borrow().is_empty()
                    && (!c.market_orders.borrow().is_empty()
                        || !c
                            .limit_orders
                            .borrow()
                            .values()
                            .into_iter()
                            .all(|v| v.is_active == false))
                {
                    src[1]
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
    fn to_exit(&self) -> Vec<f64> {
        self.into_iter()
            .map(|(c, src)| {
                if !c
                    .positions
                    .borrow()
                    .values()
                    .into_iter()
                    .next()
                    .unwrap()
                    .is_active
                {
                    src[1]
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
    fn to_entry_and_exit(&self) -> Vec<f64> {
        self.into_iter()
            .map(|(c, src)| {
                if !c
                    .positions
                    .borrow()
                    .values()
                    .into_iter()
                    .next()
                    .unwrap()
                    .is_active
                    || (!c.positions.borrow().is_empty()
                        && (!c.market_orders.borrow().is_empty()
                            || !c
                                .limit_orders
                                .borrow()
                                .values()
                                .into_iter()
                                .all(|v| v.is_active == false)))
                {
                    src[1]
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
    fn to_data(&self) -> MAP<String, Vec<f64>> {
        let mut res = MAP::default();
        for row in self
            .src
            .iter()
            .zip(self.to_capital())
            .zip(self.to_entry())
            .zip(self.to_exit())
            .zip(self.to_entry_and_exit())
            .enumerate()
            .map(|(i, ((((row, capital), entry), exit), entry_and_exit))| {
                let mut res = vec![];
                res.push(i as f64);
                res.extend_from_slice(&row[1..]);
                res.extend_from_slice(&[capital, entry, exit, entry_and_exit]);
                res
            })
        {
            for (i, key) in [
                "time", "open", "high", "low", "close", "volume", "turnover", "index", "mark",
                "capital", "entry", "exit", "entry_and_exit"
            ]
            .iter()
            .enumerate()
            {
                let el = row[i];
                res.entry(key.to_string())
                    .and_modify(|v: &mut Vec<f64>| v.push(el))
                    .or_insert(vec![el]);
            }
        }
        res
    }
}
