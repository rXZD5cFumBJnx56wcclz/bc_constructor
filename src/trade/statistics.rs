use core::f64;
use std::cell::Ref;

use bc_utils_lg::settings::{SETTINGS, SETTINGS_TRADE};
use bc_utils_lg::types::maps::MAP;
use num_traits::Float;

use crate::trade::{
    structs::TradeCell,
    utils_cell::{price_is_real_time, qty_pnl},
};

pub struct StatCollector<'a> {
    pub symbol: String,
    pub cells: Vec<TradeCell>,
    s: &'a SETTINGS_TRADE,
}

impl<'a> StatCollector<'a> {
    pub fn new(
        symbol: String,
        s: &'a SETTINGS_TRADE,
    ) -> Self {
        Self { symbol, cells: Vec::new(), s }
    }
    pub fn push(
        &mut self,
        cell: TradeCell,
    ) {
        self.cells.push(cell);
    }
}

impl<'a> IntoIterator for &'a StatCollector<'a> {
    type Item = &'a TradeCell;
    type IntoIter = std::slice::Iter<'a, TradeCell>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.cells).into_iter()
    }
}

impl StatCollector<'_> {
    pub fn to_all(values: &[Vec<f64>]) -> Vec<f64> {
        let first = values.first().unwrap();
        (0..first.len())
            .map(|i| {
                if values.iter().all(|v| v[i].is_normal()) {
                    first[i]
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
    pub fn to_any(values: &[Vec<f64>]) -> Vec<f64> {
        let first = values.first().unwrap();
        (0..first.len())
            .map(|i| {
                if values.iter().any(|v| v[i].is_normal()) {
                    first[i]
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
    pub fn to_some<T>(
        &self,
        func: fn(&TradeCell) -> Ref<MAP<String, T>>,
    ) -> Vec<f64> {
        self.cells
            .iter()
            .map(|c| {
                if !func(c).is_empty() {
                    c.src[1]
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
    pub fn to_capital(&self) -> Vec<f64> {
        self.into_iter().map(|c| c.capital).collect()
    }
    pub fn to_pnl(&self) -> Vec<f64> {
        self.into_iter()
            .map(|c| {
                let positions = c.positions.borrow();
                if positions.is_empty() {
                    f64::NAN
                } else {
                    let position = positions.values().next().unwrap();
                    qty_pnl(
                        self.s.leverage,
                        position.qty,
                        position.avg_open_price,
                        price_is_real_time(self.s.work_in_real_time, &c.src),
                        &position.position_idx,
                    )
                }
            })
            .collect()
    }
    pub fn to_entry(&self) -> Vec<f64> {
        StatCollector::to_all(&[
            StatCollector::to_any(&[
                self.to_some(|c| c.market_orders.borrow()),
                self.into_iter()
                    .map(|c| {
                        if c.limit_orders
                            .borrow()
                            .values()
                            .any(|v| v.is_active == false)
                        {
                            c.src[1]
                        } else {
                            f64::NAN
                        }
                    })
                    .collect(),
            ]),
            self.to_some(|c| c.positions.borrow()),
        ])
    }
    pub fn to_exit(&self) -> Vec<f64> {
        self.into_iter()
            .map(|c| {
                if c.positions.borrow().values().next().unwrap().is_active == false {
                    c.src[0]
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
    pub fn to_market_orders(&self) -> Vec<f64> {
        self.to_some(|c| c.market_orders.borrow())
    }
    pub fn to_limit_orders(&self) -> Vec<f64> {
        self.to_some(|c| c.limit_orders.borrow())
    }
    pub fn to_entry_and_exit(&self) -> Vec<f64> {
        self.to_entry()
            .into_iter()
            .zip(self.to_exit().into_iter())
            .map(|(v1, v2)| {
                if v1.is_normal() && v2.is_normal() {
                    v1
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
    pub fn to_positions_orders(&self) -> Vec<f64> {
        StatCollector::to_all(&[self.to_some(|v| v.positions.borrow()), self.to_entry_and_exit()])
    }
    pub fn to_data(&self) -> Vec<MAP<String, Vec<f64>>> {
        let main_columns = [
            "time", "open", "high", "low", "close", "volume", "turnover", "index", "mark",
            "capital", "entry", "exit", "pnl", "qty",
        ];
        let mut res: Vec<MAP<String, Vec<f64>>> = Default::default();
        res.push(
            self.into_iter()
                .map(|c| &c.src)
                .zip(self.to_capital())
                .zip(self.to_entry())
                .zip(self.to_exit())
                .zip(StatCollector::to_all(&[self.to_exit(), self.to_pnl()]))
                .zip(StatCollector::to_all(&[
                    self.into_iter()
                        .map(|c| c.positions.borrow().values().next().unwrap().qty)
                        .collect(),
                    self.to_entry_and_exit(),
                ]))
                .enumerate()
                .map(|(i, (((((row, capital), entry), exit), pnl), qty))| {
                    let mut v = vec![];
                    v.push(i as f64);
                    v.extend_from_slice(&row[1..]);
                    v.extend_from_slice(&[capital, entry, exit, pnl, qty]);
                    v
                })
                .fold(MAP::default(), |mut map, row| {
                    for (i, key) in main_columns.iter().enumerate() {
                        map.entry(key.to_string())
                            .and_modify(|vec: &mut Vec<f64>| vec.push(row[i]));
                    }
                    map
                }),
        );
        res.push(self.to_positions_orders().into_iter().del_nan(1).fold(
            MAP::default(),
            |mut map, el| {
                map.entry("time".to_string())
                    .and_modify(|v: &mut Vec<f64>| v.push(el.0 as f64))
                    .or_insert(vec![el.0 as f64]);
                map.entry("position_orders".to_string())
                    .and_modify(|v| v.push(el.1))
                    .or_insert(vec![el.1]);
                map
            },
        ));
        res
    }
}

pub trait NumsExt<T> {
    fn del_nan(
        self,
        sep: usize,
    ) -> impl Iterator<Item = (usize, T)>;
}

impl<T: Float + Default, V: Iterator<Item = T>> NumsExt<T> for V {
    fn del_nan(
        self,
        sep: usize,
    ) -> impl Iterator<Item = (usize, T)> {
        self.enumerate()
            .scan(0usize, move |num, el| {
                if el.1.is_normal() {
                    Some((el.0, el.1))
                } else {
                    *num += 1;
                    if *num <= sep {
                        Some((el.0, el.1))
                    } else {
                        *num = 0;
                        Some((Default::default(), T::nan()))
                    }
                }
            })
            .filter(|v| v.1.is_normal())
    }
}
