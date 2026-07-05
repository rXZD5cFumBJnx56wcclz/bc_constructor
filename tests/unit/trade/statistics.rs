use bc_constructor::trade::statistics::StatCollector;
use bc_utils::nums::nz_coll;

use crate::unit::trade::prelude::*;

#[test]
fn to_all_res_1() {
    assert_eq_pr!(vec![1., 0.,], nz_coll::<Vec<f64>, _, _>(&StatCollector::to_all(&[vec![1., f64::NAN,], vec![1., 2.,]]), 0.))
}
