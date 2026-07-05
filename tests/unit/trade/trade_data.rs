use std::pin::Pin;
use std::borrow::Borrow;

use bc_constructor::trade::trade_data::TradeData;
use bc_pack_indicators::FUNCS_EXTRACT_ARGS as FA_I;
use bc_pack_signals_ready::FUNCS_EXTRACT_ARGS as FA_R;
use bc_pack_signals_train::FUNCS_EXTRACT_ARGS as FA_T;

use crate::unit::trade::prelude::*;

static TD: LazyLock<fn() -> Pin<Box<TradeData<'static>>>> = LazyLock::new(|| || TradeData::new(&SRC_TRANSPOSE, &S, "", &FA_I(), &FA_R(), &FA_T(), &FA_O()));

#[test]
fn update_res_1() {
    let td = TD();
    let res = TD();
    td.update(&SRC_TRANSPOSE, None);
    let indications = res.indicators_gateway.indications_series(&SRC_TRANSPOSE);
        let orders = orders_create(
            &S.trade,
            res.cell.borrow().borrow(),
            res.symbol,
            &indications,
            &res
                .signals_ready_gateway
                .signals_series(&indications, &SRC_TRANSPOSE),
            &SRC_TRANSPOSE,
        );
        res.as_ref().get_ref().cell.borrow_mut().step(
            SRC_TRANSPOSE.last().unwrap(),
            &SRC_TRANSPOSE[SRC_TRANSPOSE.len() - 2],
            orders,
            &res.as_ref().get_ref().s.trade,
            &res.as_ref().get_ref().orders_collectors_gateway,
            stat_collector,
        );
    assert_eq_pr!(td, res)
}
