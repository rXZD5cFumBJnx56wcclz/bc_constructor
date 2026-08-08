use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct CountParams {
    pub max_count: usize,
}

impl CountParams {
    pub fn new(max_count: usize) -> Self {
        Self { max_count }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CountBf {
    count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct COUNT {
    pub params: CountParams,
    bf: RefCell<CountBf>,
}

impl COUNT {
    pub fn new(max_count: usize) -> Self {
        Self {
            params: CountParams::new(max_count),
            ..Default::default()
        }
    }
}

impl OrderFilter for COUNT {
    fn init_bf(&self) {
        *self.bf.borrow_mut() = Default::default();
    }
    fn filter<'a>(
        &self,
        orders: &[Option<&'a (Order, bool, Option<Trigger>)>],
        _src: &[f64],
        _signals: &[Signal],
        state: &TradeState,
    ) -> Option<&'a (Order, bool, Option<Trigger>)> {
        if state.positions.borrow().is_empty() {
            *self.bf.borrow_mut() = Default::default();
        }
        let count_res = orders.len() + self.bf.borrow().count;
        if count_res <= self.params.max_count {
            self.bf.borrow_mut().count += 1;
            *orders.get(0).unwrap()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;
    use crate::prelude_tests::prelude::*;

    static BIND: LazyLock<fn() -> COUNT> = LazyLock::new(|| || COUNT::new(2));

    #[test]
    fn filter_res_1() {
        let count = BIND();
        assert_eq_pr!(
            count.filter(
                &[Some(&(Default::default()))],
                &[],
                &[],
                &TradeState {
                    positions: RefCell::new(MAP::from_iter([(1, Default::default())])),
                    ..Default::default()
                }
            ),
            Some(&(Default::default()))
        );
        assert_eq_pr!(
            count.filter(
                &[Some(&(Default::default()))],
                &[],
                &[],
                &TradeState {
                    positions: RefCell::new(MAP::from_iter([(1, Default::default())])),
                    ..Default::default()
                }
            ),
            Some(&(Default::default()))
        );
        assert_eq_pr!(
            count.filter(
                &[Some(&(Default::default()))],
                &[],
                &[],
                &TradeState {
                    positions: RefCell::new(MAP::from_iter([(1, Default::default())])),
                    ..Default::default()
                }
            ),
            None
        );
    }

    #[test]
    fn filter_res_2() {
        let count = BIND();
        assert_eq_pr!(
            count.filter(
                &[Some(&(Default::default()))],
                &[],
                &[],
                &Default::default(),
            ),
            Some(&(Default::default()))
        );
        assert_eq_pr!(
            count.filter(
                &[Some(&(Default::default()))],
                &[],
                &[],
                &Default::default(),
            ),
            Some(&(Default::default()))
        );
    }

    #[test]
    fn filter_res_3() {
        let count = COUNT::new(1);
        assert_eq_pr!(
            count.filter(
                &[Some(&(Default::default()))],
                &[],
                &[],
                &Default::default(),
            ),
            Some(&(Default::default()))
        );
        dbg!(count.bf.borrow());
        assert_eq_pr!(
            count.filter(
                &[Some(&(Default::default()))],
                &[],
                &[],
                &TradeState {
                    positions: RefCell::new(MAP::from_iter([(1, Default::default())])),
                    ..Default::default()
                }
            ),
            None
        );
    }
}
