use std::fs::{File, copy, create_dir_all};
use std::io::{BufWriter, Write};
use std::ops::Deref;
use std::time::{SystemTime, UNIX_EPOCH};

use bc_utils_lg::types::maps::MAP;

use crate::trade::statistics::Modificator;
use crate::{
    settings::{SETTINGS, SETTINGS_FILES_PATH},
    trade::statistics::StatCollector,
};

pub struct VisualCollector<'a> {
    s: &'a SETTINGS,
    stat_collector: &'a StatCollector<'a>,
}

impl<'a> VisualCollector<'a> {
    pub fn new(
        s: &'a SETTINGS,
        stat_collector: &'a StatCollector<'a>,
    ) -> Self {
        Self { s, stat_collector }
    }
}

pub trait FileModificator {
    fn get_data_paths(&self) -> (String, String);
    fn write_any_data_column(
        path: &str,
        file_path: &str,
        data: &MAP<String, Vec<f64>>,
    ) -> std::io::Result<()>;
    fn write_any_data_value(
        path: &str,
        file_path: &str,
        data: &MAP<String, f64>,
    ) -> std::io::Result<()>;
    fn write_script(
        &self,
        path: &str,
    ) -> std::io::Result<()>; // -> std::io::Result<()>;
}

impl FileModificator for VisualCollector<'_> {
    fn get_data_paths(&self) -> (String, String) {
        let path = format!(
            "{}/{}/{}",
            self.s.files_path.backtest,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            self.stat_collector.symbol,
        );
        (format!("{}/data.dat", &path), path)
    }
    fn write_any_data_column(
        path: &str,
        file_path: &str,
        data: &MAP<String, Vec<f64>>,
    ) -> std::io::Result<()> {
        create_dir_all(path)?;
        let mut buf = BufWriter::new(File::create_new(file_path)?);
        writeln!(
            buf,
            "{}",
            data.keys()
                .map(|v| v.as_str())
                .collect::<Vec<&str>>()
                .join(" ")
        )?;
        for i in 0..data.values().into_iter().next().unwrap().len() {
            writeln!(
                buf,
                "{}",
                data.values()
                    .into_iter()
                    .map(|v| v[i].to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )?;
        }
        Ok(())
    }
    fn write_any_data_value(
        path: &str,
        file_path: &str,
        data: &MAP<String, f64>,
    ) -> std::io::Result<()> {
        create_dir_all(path)?;
        let mut buf = BufWriter::new(File::create_new(file_path)?);
        for (k, v) in data {
            writeln!(buf, "{k} {v}",)?;
        }
        Ok(())
    }
    fn write_script(
        &self,
        path: &str,
    ) -> std::io::Result<()> {
        create_dir_all(path)?;
        if self.s.files_path.script_backtest.is_empty() {
            let mut file = File::create_new(format!("{path}/{}", "script_data.plt"))?;
            writeln!(
                file,
                r##"
                set datafile separator whitespace
                set datafile columnheaders
                set boxwidth 0.8
                set style fill solid

                plot "data.dat" \
                using "time":"open":"high":"low":"close" \
                with candlesticks linecolor rgb "#7D2AD4" title "BTC", \
                "data.dat" using "time":"exit_entry" with lines dashtype (40,10) linecolor rgb "#C2820C" title "exit_entry", \
                "data.dat" using "time":"entry" with points pointtype 7 pointsize 3 linecolor rgb "#02A624" title "entry", \
                "data.dat" using "time":"exit" with points pointtype 2 pointsize 3 linecolor rgb "#A60202" title "exit", \
                "##
            )?;
        } else {
            copy(
                self.s.files_path.script_backtest.as_str(),
                format!("{path}/script_data.plt"),
            )?;
        }
        Ok(())
    }
}
