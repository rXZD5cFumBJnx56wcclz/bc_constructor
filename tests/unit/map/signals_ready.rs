use bc_signals::ready::pumpdump::PUMPDUMP;

use bc_constructor::map::signals_ready::*;
use bc_constructor::settings::{SETTINGS_SIGNAL, SETTINGS_SIGNALS};

use std::any::Any;

#[test]
fn signals_from_settings_without_bf_res_1() {
    let settings = SETTINGS_SIGNALS::from_iter([(
        "pumpdump_1".to_string(),
        SETTINGS_SIGNAL { key: "pumpdump".to_string(), ..Default::default() },
    )]);
    let funcs_extract_args = FUNCS_EXTRACT_ARGS();
    let res = get_signals_from_settings_without_bf(&settings, &funcs_extract_args);
    let res_1 = res.get("pumpdump_1").unwrap().as_ref();
    let rsi_test_1 = PUMPDUMP::default();
    let rsi_test_2 = (res_1 as &dyn Any).downcast_ref::<PUMPDUMP>().unwrap();
    assert_eq!(&rsi_test_1, rsi_test_2);
}
