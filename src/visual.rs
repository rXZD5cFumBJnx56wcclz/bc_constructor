use std::fs::{File, copy, create_dir_all};
use std::io::{BufWriter, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use bc_utils_lg::settings::SETTINGS;
use bc_utils_lg::types::maps::MAP;

use crate::trade::statistics::StatCollector;

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
    ) -> std::io::Result<()>;
}

impl FileModificator for VisualCollector<'_> {
    fn get_data_paths(&self) -> (String, String) {
        let path = format!(
            "{}/{}/{}",
            self.s.files_path.backtest.to_str().unwrap(),
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
        if self
            .s
            .files_path
            .script_backtest
            .to_str()
            .unwrap()
            .is_empty()
        {
            let mut file = File::create_new(format!("{path}/{}", "script_data.plt"))?;
            writeln!(
                file,
                // no work in real time
                r##"
                set datafile separator whitespace
                set datafile columnheaders
                set style fill solid
                set boxwidth 0.8
                set style textbox opaque fillcolor rgb "#EBEBEB" bordercolor rgb "#0F0F0F"
                plot \
                "data.dat" index 0 using "time":"open":"high":"low":"close" with candlesticks linecolor rgb "#7D2AD4" title "{}", \
                "data.dat" index 1 using "time":"positions_orders" with lines linewidth 2 dashtype (40,10) linecolor rgb "#C2820C" title "positions_orders", \
                "data.dat" index 0 using "time":"entry" with points pointtype 7 pointsize 3 linecolor rgb "#0F0F0F" notitle, \
                "data.dat" index 0 using "time":"exit" with points lw 8 pointtype 2 pointsize 2 linecolor rgb "#0F0F0F" notitle, \
                "data.dat" index 0 using "time":"entry" with points pointtype 7 pointsize 2.5 linecolor rgb "#FFFFFF" notitle, \
                "data.dat" index 0 using "time":"exit" with points lw 6 pointtype 2 pointsize 2 linecolor rgb "#FFFFFF" notitle, \
                "data.dat" index 0 using "time":"entry" with points pointtype 7 pointsize 2 linecolor rgb "#00C222" title "entry", \
                "data.dat" index 0 using "time":"exit" with points lw 3 pointtype 2 pointsize 2 linecolor rgb "#C20006" title "exit", \
                "data.dat" index 0 using "time":(column("pnl") != column("pnl") ? NaN : column("open")):"pnl" with labels boxed offset 0,1 title "pnl", \
                "data.dat" index 0 using "time":(column("qty") != column("qty") ? NaN : column("open")):"pnl" with labels boxed offset 0,2 title "qty"
                "##,
                self.stat_collector.symbol,
            )?;
        } else {
            copy(
                self.s.files_path.script_backtest.to_str().unwrap(),
                format!("{path}/script_data.plt"),
            )?;
        }
        Ok(())
    }
}
