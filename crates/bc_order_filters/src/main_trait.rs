#![allow(non_camel_case_types)]

use std::{any::Any, cell::RefCell};

use bc_utils_lg::{
    structs::{
        signals::Signal,
        trade::{Order, TradeState, Trigger},
    },
    types::maps::MAP,
};

pub trait OrderFilter: Any {
    fn bf<'a>(
        &self,
        orders: &[Option<&(Order, bool, Option<Trigger>)>],
        src: &[f64],
        signals: &[Signal],
        state: &TradeState,
    ) -> RefCell<MAP<&'a str, Vec<f64>>>;
    fn filter<'a>(
        &self,
        bf: &RefCell<MAP<&str, Vec<f64>>>,
        orders: &[Option<&'a (Order, bool, Option<Trigger>)>],
        src: &[f64],
        signals: &[Signal],
        state: &TradeState,
    ) -> Option<&'a (Order, bool, Option<Trigger>)>;
}

pub type BF_ORDER_FILTER<'a> = RefCell<MAP<&'a str, Vec<f64>>>;

pub trait BfOrderFilterExt {
    fn from_iter<'a>(iter: impl IntoIterator<Item = (&'a str, Vec<f64>)>) -> BF_ORDER_FILTER<'a>;
}

impl BfOrderFilterExt for BF_ORDER_FILTER<'_> {
    fn from_iter<'a>(iter: impl IntoIterator<Item = (&'a str, Vec<f64>)>) -> BF_ORDER_FILTER<'a> {
        RefCell::new(MAP::from_iter(iter))
    }
}
