use std::borrow::Borrow;
use std::error::Error;
use std::fs::{self, File, copy, create_dir_all, remove_dir_all};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use bc_utils::other::transpose;
use bc_utils_lg::structs::settings::SETTINGS_FILES_PATH;
use bc_utils_lg::types::maps::{MAP, MAP_LINK, MapTrait};
use bincode::config::standard;
use bincode::serde::{decode_from_slice, encode_to_vec};

use crate::visual::BT_SCRIPT;

fn get_backtest_dir(
    dir: &str,
    symbol: &str,
    time: u64,
) -> String {
    format!("{dir}/{time}/{symbol}",)
}

fn write_any_data_column<'a, T, M>(
    path: &str,
    file_path: &str,
    data: &'a [T],
) -> std::io::Result<()>
where
    T: Borrow<M>,
    M: MapTrait<'a, String, Vec<f64>>,
    M: 'a,
{
    create_dir_all(path)?;
    let mut buf = BufWriter::new(File::create_new(file_path)?);
    for el in data {
        writeln!(
            buf,
            "{}",
            el.borrow()
                .keys()
                .into_iter()
                .map(|v| v.as_str())
                .collect::<Vec<&str>>()
                .join(" ")
        )?;
        for i in 0..el.borrow().values().into_iter().next().unwrap().len() {
            writeln!(
                buf,
                "{}",
                el.borrow()
                    .values()
                    .into_iter()
                    .map(|v| v[i].to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )?;
        }
        writeln!(buf, "\n\n");
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

fn parse_data_columns<'a>(
    splitted: impl IntoIterator<Item = &'a str>
) -> Result<Vec<MAP<String, Vec<f64>>>, Box<dyn Error>> {
    splitted
        .into_iter()
        .map(|data| -> Result<MAP<String, Vec<f64>>, Box<dyn Error>> {
            transpose(
                data.split("\n")
                    .into_iter()
                    .map(|v| v.split(" ").collect())
                    .collect(),
            )
            .into_iter()
            .map(|v| -> Result<(String, Vec<f64>), Box<dyn Error>> {
                Ok((
                    v[0].to_string(),
                    v.into_iter()
                        .skip(1)
                        .map(|f| -> Result<f64, Box<dyn Error>> { Ok(f.parse::<f64>()?) })
                        .collect::<Result<Vec<f64>, Box<dyn Error>>>()?,
                ))
            })
            .collect::<Result<MAP<String, Vec<f64>>, Box<dyn Error>>>()
        })
        .collect::<Result<Vec<MAP<String, Vec<f64>>>, Box<dyn Error>>>()
}

fn parse_data_values<'a>(
    splitted: impl Iterator<Item = &'a str>
) -> Result<Vec<MAP<String, f64>>, Box<dyn Error>> {
    splitted
        .into_iter()
        .map(|v| {
            v.split("\n")
                .map(|v2| -> Result<(String, f64), Box<dyn Error>> {
                    let mut sp = v2.split(" ");
                    Ok((
                        sp.next().ok_or("err")?.to_string(),
                        sp.next().ok_or("err")?.parse::<f64>()?,
                    ))
                })
                .collect::<Result<MAP<String, f64>, Box<dyn Error>>>()
        })
        .collect()
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
        or: Vec<Vec<f64>>,
    ) -> Vec<Vec<f64>> {
        self.src().unwrap_or(or)
    }
    pub fn backtest_write(
        &self,
        data: &Vec<MAP_LINK<String, Vec<f64>>>,
        stat_columns: &MAP<String, Vec<f64>>,
        stat_values: &MAP<String, f64>,
        symbol: &str,
        time: u64,
    ) -> Result<(), Box<dyn Error>> {
        let dir = get_backtest_dir(self.s.backtest.to_str().unwrap(), symbol, time);
        create_dir_all(&dir)?;
        if !self.s.backtest.as_os_str().is_empty() {
            self.script_backtest_write(&dir, symbol)?;
            write_any_data_column(&dir, &format!("{dir}/data.dat"), data)?;
            write_any_data_column::<&MAP<_, _>, MAP<_, _>>(
                &dir,
                &format!("{dir}/stat_columns.dat"),
                &[stat_columns],
            )?;
            write_any_data_value(&dir, &format!("{dir}/stat_values.dat",), &stat_values)?;
        }
        Ok(())
    }
    pub fn backtest(
        &self,
        dir: &PathBuf,
    ) -> Result<
        (
            Vec<MAP<String, Vec<f64>>>,
            MAP<String, Vec<f64>>,
            MAP<String, f64>,
        ),
        Box<dyn Error>,
    > {
        Ok((
            parse_data_columns(
                fs::read_to_string(format!("{}/data.dat", dir.to_str().unwrap()))?.split("\n\n"),
            )?,
            parse_data_columns(
                [fs::read_to_string(format!("{}/stat_columns.dat", dir.to_str().unwrap()))?
                    .as_str()]
                .into_iter(),
            )?
            .remove(0),
            parse_data_values(
                [fs::read_to_string(format!("{}/stat_values.dat", dir.to_str().unwrap()))?
                    .as_str()]
                .into_iter(),
            )?
            .remove(0),
        ))
    }
    pub fn backtest_or(
        &self,
        dir: &PathBuf,
        or: (
            Vec<MAP<String, Vec<f64>>>,
            MAP<String, Vec<f64>>,
            MAP<String, f64>,
        ),
    ) -> (
        Vec<MAP<String, Vec<f64>>>,
        MAP<String, Vec<f64>>,
        MAP<String, f64>,
    ) {
        self.backtest(dir).unwrap_or(or)
    }
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
    // train_model_or
}

impl FileWR<'_> {
    pub fn backtests_del(&self) -> Result<(), Box<dyn Error>> {
        Ok(remove_dir_all(&self.s.backtest)?)
    }
}
