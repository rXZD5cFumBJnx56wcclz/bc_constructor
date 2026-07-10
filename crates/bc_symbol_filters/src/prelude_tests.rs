#[cfg(test)]
pub mod prelude {
    #![allow(unused_imports)]

    pub use pretty_assertions::assert_eq as assert_eq_pr;

    pub use crate::main_trait::*;
}
