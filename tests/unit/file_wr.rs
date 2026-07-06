use std::{fs::remove_dir_all, path::Path, pin::Pin, sync::Mutex};

use bc_constructor::{
    file_wr::*,
    trade::{
        statistics::{StatCollector, StatData},
        trade_data::AfterTradeData,
    },
    visual::BT_SCRIPT,
};
use bc_utils_lg::structs::trade::TradeCell;

use crate::unit::prelude::*;

static LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
static F: LazyLock<FileWR> = LazyLock::new(|| FileWR::new(&S.files_dir));
static STAT_DATA_AFTER_DATA: LazyLock<fn() -> (StatData, Pin<Box<AfterTradeData<'static>>>)> =
    LazyLock::new(|| {
        || {
            let mut stat_collector = StatCollector::new("".to_string(), &S.trade);
            stat_collector.push(TradeCell::new(100., SRC_EL.clone(), SRC_EL1.clone()));
            stat_collector.push(TradeCell::new(100., SRC_EL.clone(), SRC_EL1.clone()));
            let stat_data = stat_collector.to_data();
            let stat_data_vec = stat_data.to_vec();
            let after_data = AfterTradeData::new(&S, &stat_data_vec[0], &FA_I());
            (stat_data, after_data)
        }
    });

#[test]
fn src_write_res_1() -> Result<(), Box<dyn Error>> {
    let _l = LOCK.lock()?;
    assert!(!S.files_dir.src.exists());
    F.src_write(&SRC)?;
    assert!(Path::new(&format!("{}/src.bin", S.files_dir.src.to_str().unwrap())).exists());
    remove_dir_all(&S.files_dir.src)?;
    assert!(!S.files_dir.src.exists());
    remove_dir_all(Path::new("test_dir"))?;
    Ok(())
}

#[test]
fn src_res_1() -> Result<(), Box<dyn Error>> {
    let _l = LOCK.lock()?;
    F.src_write(&SRC)?;
    assert!(Path::new(&format!("{}/src.bin", S.files_dir.src.to_str().unwrap())).exists());
    let _: Vec<Vec<f64>> = F.src()?;
    remove_dir_all(&S.files_dir.src)?;
    assert!(!S.files_dir.src.exists());
    remove_dir_all(Path::new("test_dir"))?;
    Ok(())
}

#[test]
fn script_write_res_1() -> Result<(), Box<dyn Error>> {
    let _l = LOCK.lock()?;
    let dir = get_backtest_dir(S.files_dir.script_backtest.to_str().unwrap(), "symbol", 1);
    let file_name = "script_backtest.plt";
    assert!(!S.files_dir.script_backtest.exists());
    F.script_write(&dir, file_name, &BT_SCRIPT("symbol"))?;
    assert!(Path::new(&format!("{dir}/{file_name}",)).exists());
    remove_dir_all(&S.files_dir.script_backtest)?;
    assert!(!S.files_dir.script_backtest.exists());
    remove_dir_all(Path::new("test_dir"))?;
    Ok(())
}

#[test]
fn script_res_1() -> Result<(), Box<dyn Error>> {
    let _l = LOCK.lock()?;
    let dir = get_backtest_dir(S.files_dir.script_backtest.to_str().unwrap(), "symbol", 1);
    let file_name = "script_backtest.plt";
    let path = format!("{dir}/{file_name}",);
    F.script_write(&dir, file_name, &BT_SCRIPT("symbol"))?;
    assert!(Path::new(&path).exists());
    let _: String = F.script(&path.into())?;
    remove_dir_all(&S.files_dir.script_backtest)?;
    assert!(!S.files_dir.script_backtest.exists());
    remove_dir_all(Path::new("test_dir"))?;
    Ok(())
}

#[test]
fn backtest_write_res_1() -> Result<(), Box<dyn Error>> {
    let _l = LOCK.lock()?;
    assert!(!S.files_dir.backtest.exists());
    let (stat_data, after_data) = STAT_DATA_AFTER_DATA();
    let stat_data_vec = stat_data.to_vec();
    F.backtest_write(
        &stat_data,
        &after_data.to_stat_columns(&stat_data_vec[0]),
        &after_data.to_stat_values(&stat_data_vec[0]),
        "symbol",
        1,
    )?;
    assert!(
        Path::new(&format!(
            "{}/1/symbol",
            S.files_dir.backtest.to_str().unwrap()
        ))
        .exists()
    );
    assert!(
        Path::new(&format!(
            "{}/1/symbol/script_backtest.plt",
            S.files_dir.backtest.to_str().unwrap()
        ))
        .exists()
    );
    assert!(
        Path::new(&format!(
            "{}/1/symbol/script_stat_columns.plt",
            S.files_dir.backtest.to_str().unwrap()
        ))
        .exists()
    );
    assert!(
        Path::new(&format!(
            "{}/1/symbol/script_stat_values.plt",
            S.files_dir.backtest.to_str().unwrap()
        ))
        .exists()
    );
    remove_dir_all(&S.files_dir.backtest)?;
    assert!(!S.files_dir.backtest.exists());
    remove_dir_all(Path::new("test_dir"))?;
    Ok(())
}

#[test]
fn backtest_res_1() -> Result<(), Box<dyn Error>> {
    let _l = LOCK.lock()?;
    assert!(!S.files_dir.backtest.exists());
    let (stat_data, after_data) = STAT_DATA_AFTER_DATA();
    let stat_data_vec = stat_data.to_vec();
    F.backtest_write(
        &stat_data,
        &after_data.to_stat_columns(&stat_data_vec[0]),
        &after_data.to_stat_values(&stat_data_vec[0]),
        "symbol",
        1,
    )?;
    let _: (
        Vec<MAP<String, Vec<f64>>>,
        MAP<String, Vec<f64>>,
        MAP<String, f64>,
    ) = F.backtest(&format!("{}/1/symbol", S.files_dir.backtest.to_str().unwrap()).into())?;
    remove_dir_all(&S.files_dir.backtest)?;
    assert!(!S.files_dir.backtest.exists());
    remove_dir_all(Path::new("test_dir"))?;
    Ok(())
}
