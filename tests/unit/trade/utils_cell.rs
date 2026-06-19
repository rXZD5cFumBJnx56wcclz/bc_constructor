use std::sync::LazyLock;

use bc_constructor::settings::SETTINGS_ORDER_PLACE;
use bc_signals::ready::ready_trait::Signal;

use bc_constructor::settings::SETTINGS_STRATEGY;
use bc_constructor::trade::utils_cell::*;

static S: LazyLock<SETTINGS_STRATEGY> = LazyLock::new(|| SETTINGS_STRATEGY {
    signal_hold: 0.,
    signal_short: -1.,
    signal_long: 1.,
    commission_market: 0.00055,
    commission_limit: 0.0002,
    leverage: 10.,
    capital: 100.,
    percent_of_capital: 0.01,
    order_place_settings: SETTINGS_ORDER_PLACE {
        stoploss: vec![((1., 0.), (0.5, 0.))],
        ..Default::default()
    },
    ..Default::default()
});
static SIGNAL: LazyLock<Signal> = LazyLock::new(|| Signal::default());

#[test]
fn qty_and_commision_1() {
    let qty = S.capital * S.percent_of_capital * S.leverage;
    assert_eq!(
        (qty, qty * S.commission_market),
        qty_and_commission(&*S, &*SIGNAL, "market", 0., 0.),
    );
}
