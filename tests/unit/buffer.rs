use bc_constructor::buffer::*;
use bc_utils::other::roll_slice1;

use crate::unit::prelude::*;

static BF: LazyLock<fn() -> Buffer> = LazyLock::new(|| || {Buffer::new(SRC.to_vec())});

#[test]
fn update_res_1() {
    let mut bf = BF();
    let mut res = BF();
    bf.update(SRC_EL.to_vec());
    roll_slice1(&mut res.0, -1);
    let l = res.0.len() - 1;
    res.0[l] = SRC_EL.to_vec();
    assert_eq_pr!(bf, res);
}

#[test]
fn update_extend_res_1() {
    let mut bf = BF();
    let mut res = BF();
    bf.update_extend(&SRC);
    roll_slice1(&mut res.0, -(SRC.len() as i32));
    for _ in 0..SRC.len() {
        res.0.pop();
    }
    res.0.extend_from_slice(&SRC);
    assert_eq_pr!(bf, res);
}