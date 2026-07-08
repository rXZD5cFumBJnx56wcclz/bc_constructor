use std::error::Error;

use bc_exch_api_funcs::bybit::{exch_struct::BYBIT, market::src::Src};
use bc_file_wr::file_wr::FileWR;
use bc_indicators::main_trait::Indicator;
use bc_indicators_gw::gw::get_w_max;
use bc_orders_collectors::main_trait::OrderCollector;
use bc_signals::ready::main_trait::SignalReady;
use bc_signals::train::main_trait::SignalTrain;
use bc_utils_lg::{
    structs::settings::{SETTINGS, SETTINGS_IND, SETTINGS_ORDER_COLLECTOR, SETTINGS_SIGNAL},
    types::maps::FUNCS_EXTRACT_ARGS_TYPE as FA,
};

use crate::{cli::AdditionArgsFlags, cli_processing::check_addition_args_flags};

pub fn init_data<'a>(
    s: &'a SETTINGS,
    fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
    _fa_signals_ready: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
    _fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
    _fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
    addition_args_flags: &Option<AdditionArgsFlags>,
) -> Result<(FileWR<'a>, Box<impl Src>, Vec<String>, usize), Box<dyn Error>> {
    let file_wr = FileWR::new(&s.files_dir);
    check_addition_args_flags(addition_args_flags, &file_wr)?;
    // fix symbols
    Ok((
        file_wr,
        Box::new(BYBIT::new(s)),
        Default::default(),
        get_w_max(&s.indications, fa_indicators),
    ))
}
