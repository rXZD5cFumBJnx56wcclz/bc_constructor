use bc_utils_lg::settings::SETTINGS;
use std::error::Error;

use crate::{buffer::Buffer, file_wr::FileWR, trade::trade_data_collector::TradeData};

pub fn backtest(s: &SETTINGS) -> Result<(), Box<dyn Error>> {
    let exch = 
    let file_wr = FileWR::new(&s.files_path);
    let mut trade_data = TradeData::default();
    let mut buffer = Buffer::new(file_wr.src_or())
    Ok(())
}