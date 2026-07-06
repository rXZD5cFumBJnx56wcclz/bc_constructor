use std::borrow::Borrow;
use std::pin::Pin;

use bc_constructor::buffer::ToBuff;
use bc_constructor::trade::trade_data::TradeData;
use bc_indicators::ready_imports::BF_INDICATOR;
use bc_pack_indicators::FUNCS_EXTRACT_ARGS as FA_I;
use bc_pack_signals_ready::FUNCS_EXTRACT_ARGS as FA_R;
use bc_pack_signals_train::FUNCS_EXTRACT_ARGS as FA_T;
use bc_signals::def_impl::BF_SIGNALS;

use crate::unit::trade::prelude::*;

static TD: LazyLock<fn() -> Pin<Box<TradeData<'static>>>> =
    LazyLock::new(|| || TradeData::new(&SRC_TRANSPOSE, &S, "", &FA_I(), &FA_R(), &FA_T(), &FA_O()));

#[test]
fn update_res_1() {
    let td = TD();
    let res = TD();
    td.update(&SRC.to_buff(), None);
    let indications = res.indicators_gateway.indications_series(&SRC_TRANSPOSE);
    let orders = orders_create(
        &S.trade,
        res.cell.borrow().borrow(),
        res.symbol,
        &indications,
        &res.signals_ready_gateway
            .signals_series(&indications, &SRC_TRANSPOSE),
        &SRC,
    );
    res.as_ref().get_ref().cell.borrow_mut().step(
        SRC.last().unwrap(),
        &SRC[SRC.len() - 2],
        orders,
        &res.as_ref().get_ref().s.trade,
        &res.as_ref().get_ref().orders_collectors_gateway,
        None,
    );
    let td_ref = td.as_ref().get_ref();
    let res_ref = res.as_ref().get_ref();
    assert_eq_pr!(
        unsafe { &*td_ref.indicators_gateway.indicators }
            .indicators
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_INDICATOR>>(),
        unsafe { &*res_ref.indicators_gateway.indicators }
            .indicators
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_INDICATOR>>()
    );
    assert_eq_pr!(
        unsafe { &*td_ref.signals_ready_gateway.signals_ready }
            .signals_ready
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_SIGNALS>>(),
        unsafe { &*res_ref.signals_ready_gateway.signals_ready }
            .signals_ready
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_SIGNALS>>()
    );
    assert_eq_pr!(
        unsafe { &*td_ref.signals_train_gateway.signals_train }
            .signals_train
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_SIGNALS>>(),
        unsafe { &*res_ref.signals_train_gateway.signals_train }
            .signals_train
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_SIGNALS>>()
    );
}

#[test]
fn update_bf_res_1() {
    let td = TD();
    let res = TD();
    // fix fa
    td.update_bf(&SRC_TRANSPOSE, &FA_I(), &FA_R(), &FA_T());
    res.as_ref().gw_values.indicators.borrow_mut().update_bf(
        &SRC_TRANSPOSE,
        &(&*res.borrow().s).indications,
        &FA_I(),
    );
    res.gw_values.signals_ready.borrow_mut().update_bf(
        &SRC_TRANSPOSE,
        &res.s,
        &FA_R(),
        &res.gw_values.indicators.borrow().indicators_without_bf,
    );
    res.gw_values.signals_train.borrow_mut().update_bf(
        &SRC_TRANSPOSE,
        &res.s,
        &FA_T(),
        &res.gw_values.indicators.borrow().indicators_without_bf,
    );
    let td_ref = td.as_ref().get_ref();
    let res_ref = res.as_ref().get_ref();
    assert_eq_pr!(
        unsafe { &*td_ref.indicators_gateway.indicators }
            .indicators
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_INDICATOR>>(),
        unsafe { &*res_ref.indicators_gateway.indicators }
            .indicators
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_INDICATOR>>()
    );
    assert_eq_pr!(
        unsafe { &*td_ref.signals_ready_gateway.signals_ready }
            .signals_ready
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_SIGNALS>>(),
        unsafe { &*res_ref.signals_ready_gateway.signals_ready }
            .signals_ready
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_SIGNALS>>()
    );
    assert_eq_pr!(
        unsafe { &*td_ref.signals_train_gateway.signals_train }
            .signals_train
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_SIGNALS>>(),
        unsafe { &*res_ref.signals_train_gateway.signals_train }
            .signals_train
            .values()
            .map(|(v1, _)| v1)
            .collect::<Vec<&BF_SIGNALS>>()
    );
}
