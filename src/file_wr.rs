use std::error::Error;
use std::fs::{self, File, copy, create_dir_all};
use std::io::{BufWriter, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use bc_utils_lg::settings::SETTINGS_FILES_PATH;
use bc_utils_lg::types::maps::MAP;
use bincode::config::standard;
use bincode::serde::{decode_from_slice, encode_to_vec};

use crate::visual::BT_SCRIPT;

fn get_backtest_dir(
    dir: &str,
    symbol: &str,
    time: u64
) -> String {
    format!(
        "{dir}/{time}/{symbol}",
        
    )
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

pub struct FileWR<'a> {
    s: &'a SETTINGS_FILES_PATH,
}

impl<'a> FileWR<'a> {
    pub fn new(s: &'a SETTINGS_FILES_PATH) -> Self {
        Self { s }
    }
}

impl FileWR<'_> {
    pub fn src_write(
        &self,
        src: &Vec<Vec<f64>>,
    ) -> Result<(), Box<dyn Error>> {
        if !self.s.src_data.as_os_str().is_empty() {
            create_dir_all(&self.s.src_data)?;
            fs::write(
                &format!("{}/src.bin", self.s.src_data.to_str().unwrap()),
                encode_to_vec(src, standard())?,
            )?;
        }
        Ok(())
    }
    pub fn src(&self) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
        Ok(decode_from_slice(&fs::read(&self.s.src_data)?, standard())?.0)
    }
    pub fn src_or(
        &self,
        src: Vec<Vec<f64>>,
    ) -> Vec<Vec<f64>> {
        self.src().unwrap_or(src)
    }
    pub fn backtest_write(
        &self,
        data: &Vec<MAP<String, Vec<f64>>>,
        symbol: &str,
        time: u64,
    ) -> Result<(), Box<dyn Error>> {
        let dir = get_backtest_dir(self.s.backtest.to_str().unwrap(), symbol, time);
        create_dir_all(&dir)?;
        if !self.s.backtest.as_os_str().is_empty() {
            self.script_backtest_write(&dir, symbol)?;
            write_any_data_column(&dir, &format!("{dir}/data.dat"), &data[0])?;
        }
        Ok(())
    }
    // pub fn backtest(&self, path: &PathBuf) -> Result<()>
    pub fn script_backtest_write(
        &self,
        dir: &str,
        symbol: &str,
    ) -> Result<(), Box<dyn Error>> {
        if self.s.script_backtest.as_os_str().is_empty() {
            let mut file = File::create_new(format!("{}/{}", dir, "script_data.plt"))?;
            writeln!(file, "{}", BT_SCRIPT(symbol,))?;
        } else {
            copy(
                self.s.script_backtest.to_str().unwrap(),
                format!("{dir}/script_data.plt"),
            )?;
        }
        Ok(())
    }
    pub fn script_backtest(&self) -> Result<String, Box<dyn Error>> {
        Ok(fs::read_to_string(&self.s.script_backtest)?)
    }
    pub fn script_backtest_or(
        &self,
        script_backtest: String,
    ) -> String {
        self.script_backtest().unwrap_or(script_backtest)
    }
    // train_model_write
    // train_model
}
