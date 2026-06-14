#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use bc_utils_lg::types::maps::{MAP, MAP_LINK};
use serde_json5::from_reader;

pub fn settings_from_json(dir: &str) -> Result<SETTINGS, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(dir)?);
    from_reader(&mut reader).map_err(|e| Box::new(e) as Box<dyn Error>)
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SETTINGS_USED_SRC {
    pub index: usize,
    pub sub_from_last_i: usize,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SETTINGS_IND {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_SRC>,
    pub used_ind: Vec<String>,
    pub order_used: Vec<usize>,
}
pub type SETTINGS_INDS = MAP_LINK<String, SETTINGS_IND>;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SETTINGS_SIGNAL {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_SRC>,
    pub used_ind: Vec<String>,
    pub used_signals: Vec<String>,
    pub order_used_src: Vec<usize>,
    pub order_used_signals: Vec<usize>,
}
pub type SETTINGS_SIGNALS = MAP_LINK<String, SETTINGS_SIGNAL>;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SETTINGS_EXCH {
    pub url: String,
    pub key: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SETTINGS_MSG {
    pub key: String,
    pub chat: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SETTINGS {
    pub exch: SETTINGS_EXCH,
    pub indications: SETTINGS_INDS,
    pub signals_train: SETTINGS_SIGNALS,
    pub signals_ready: SETTINGS_SIGNALS,
}
