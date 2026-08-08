use std::collections::hash_map::Entry;

use bc_statistics::stat_collector::StatCollector;
use bc_utils_lg::{
    structs::settings::{SETTINGS_STAT_DATA_COLL, SETTINGS_STAT_DATA_INDEXING_DATA},
    types::maps::MAP,
};

pub struct StatisticsGateway<'a> {
    pub statistics_funcs: *const MAP<&'a str, fn(&StatCollector) -> Vec<f64>>,
    s: &'a SETTINGS_STAT_DATA_COLL,
}

impl<'a> StatisticsGateway<'a> {
    pub fn new(
        statistics_funcs: *const MAP<&'a str, fn(&StatCollector) -> Vec<f64>>,
        s: &'a SETTINGS_STAT_DATA_COLL,
    ) -> Self {
        Self {
            statistics_funcs,
            s,
        }
    }
}

fn indexing_data(
    s: &SETTINGS_STAT_DATA_INDEXING_DATA,
    statistics_vec: Vec<f64>,
    map: &MAP<&str, MAP<&str, Vec<f64>>>,
) -> Vec<f64> {
    map[s.key_map_index.as_str()][s.key_index.as_str()]
        .iter()
        .map(|i| {
            if !i.is_nan() {
                statistics_vec[*i as usize]
            } else {
                f64::NAN
            }
        })
        .collect()
}

impl<'a> StatisticsGateway<'a> {
    pub fn data(&self, stat_collector: &StatCollector) -> MAP<&str, MAP<&str, Vec<f64>>> {
        self.s
            .0
            .iter()
            .fold(MAP::default(), |mut init, (k, setting)| {
                let statistics =
                    unsafe { &*self.statistics_funcs }[setting.key.as_str()](stat_collector);
                let res = if let Some(setting) = setting.indexing_data.as_ref() {
                    indexing_data(setting, statistics, &init)
                } else {
                    statistics
                };
                match init.entry(&setting.map_group) {
                    Entry::Occupied(mut e) => {
                        e.get_mut().insert(k.as_str(), res);
                    }
                    Entry::Vacant(e) => {
                        e.insert(MAP::from_iter([(setting.map_group.as_str(), res)]));
                    }
                };
                init
            })
    }
}
