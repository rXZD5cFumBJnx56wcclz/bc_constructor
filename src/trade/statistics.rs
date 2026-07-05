use std::cell::Ref;
use std::ops::Deref;

use bc_utils::other::transpose;
use bc_utils_lg::structs::trade::{Position, TradeCell};
use bc_utils_lg::types::maps::MAP;
use bc_utils_lg::{structs::settings::SETTINGS_TRADE, types::maps::MAP_LINK};
use num_traits::Float;

use crate::trade::structs::IsActive;
use crate::trade::utils_cell::{price_is_real_time, qty_pnl};

#[derive(Debug)]
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
        (0..values.first().unwrap().len())
            .map(|i| {
                let bind = values.iter().map(|v| v[i]).find(|v| v.is_normal());
                if let Some(el) = bind {
                    el
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
    pub fn to_some<T>(
        &self,
        func: fn(&TradeCell) -> Ref<MAP<String, T>>,
        include_inactive: bool,
    ) -> Vec<f64>
    where
        T: IsActive,
    {
        let f = |v: Ref<MAP<String, T>>| {
            if include_inactive {
                v.values().any(|v| v.is_active())
            } else {
                !v.is_empty()
            }
        };
        self.cells
            .iter()
            .map(|c| {
                if f(func(c)) {
                    // stat used open prices
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
                self.to_some(|c| c.market_orders.borrow(), true),
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
            self.to_some(|c| c.positions.borrow(), true),
        ])
    }
    pub fn to_exit(&self) -> Vec<f64> {
        self.into_iter()
            .map(|c| {
                if c.positions.borrow().values().any(|v| !v.is_active) {
                    c.src[0]
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
    pub fn to_market_orders(&self) -> Vec<f64> {
        self.to_some(|c| c.market_orders.borrow(), true)
    }
    pub fn to_limit_orders(&self) -> Vec<f64> {
        self.to_some(|c| c.limit_orders.borrow(), true)
    }
    pub fn to_entry_and_exit(&self) -> Vec<f64> {
        self.to_entry()
            .into_iter()
            .zip(self.to_exit().into_iter())
            .map(|(v1, v2)| {
                let bind = [v1, v2].into_iter().find(|v| v.is_normal());
                if let Some(v) = bind {
                    v
                } else {
                    f64::NAN
                }
            })
            .collect()
    }
    pub fn to_positions_entry_exit(&self) -> Vec<f64> {
        StatCollector::to_all(&[
            self.to_some(|v| v.positions.borrow(), false),
            self.to_entry_and_exit(),
        ])
    }
    pub fn to_value_positions(
        &self,
        func: fn(&Position) -> f64,
    ) -> Vec<f64> {
        StatCollector::to_all(&[
            self.into_iter()
                .map(|c| {
                    if !c.positions.borrow().is_empty() {
                        func(&c.positions.borrow().values().next().unwrap())
                    } else {
                        f64::NAN
                    }
                })
                .collect(),
            self.to_entry_and_exit(),
        ])
    }
    pub fn to_data(&self) -> StatData {
        StatData(vec![
            MAP_LINK::from_iter([
                (
                    "time".to_string(),
                    (0..self.cells.len())
                        .map(|v| v as f64)
                        .collect::<Vec<f64>>(),
                ),
                (
                    "open".to_string(),
                    self.into_iter().map(|v| v.src[1]).collect(),
                ),
                (
                    "high".to_string(),
                    self.into_iter().map(|v| v.src[2]).collect(),
                ),
                (
                    "low".to_string(),
                    self.into_iter().map(|v| v.src[3]).collect(),
                ),
                (
                    "close".to_string(),
                    self.into_iter().map(|v| v.src[4]).collect(),
                ),
                (
                    "volume".to_string(),
                    self.into_iter().map(|v| v.src[5]).collect(),
                ),
                (
                    "turnover".to_string(),
                    self.into_iter().map(|v| v.src[6]).collect(),
                ),
                ("capital".to_string(), self.to_capital()),
                ("entry".to_string(), self.to_entry()),
                ("exit".to_string(), self.to_exit()),
                (
                    "pnl".to_string(),
                    StatCollector::to_all(&[self.to_pnl(), self.to_exit()]),
                ),
                ("qty".to_string(), self.to_value_positions(|v| v.qty)),
            ]),
            {
                let mut bind = transpose(
                    self.to_positions_entry_exit()
                        .into_iter()
                        .del_nan(1)
                        .map(|(time, pos)| vec![time as f64, pos])
                        .collect::<Vec<Vec<f64>>>(),
                );
                MAP_LINK::from_iter([
                    ("time".to_string(), bind.remove(0)),
                    ("positions_entry_exit".to_string(), bind.remove(0)),
                ])
            },
        ])
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct StatData(pub Vec<MAP_LINK<String, Vec<f64>>>);

impl StatData {
    pub fn to_vec(&self) -> Vec<Vec<Vec<f64>>> {
        self.0
            .iter()
            .map(|v| v.iter().map(|v| v.1.clone()).collect::<Vec<Vec<f64>>>())
            .collect::<Vec<Vec<Vec<f64>>>>()
    }
}

impl Deref for StatData {
    type Target = Vec<MAP_LINK<String, Vec<f64>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
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
