use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct COUNT {
    pub max_count: f64,
}

impl OrderFilter for COUNT {
    fn bf<'a>(
        &self,
        _: &[Option<&(Order, bool, Option<Trigger>)>],
        _: &[f64],
        _: &[Signal],
        _: &TradeState,
    ) -> BF_ORDER_FILTER<'a> {
        BF_ORDER_FILTER::from_iter([("count", vec![0.])])
    }
    fn filter<'a>(
        &self,
        bf: &RefCell<MAP<&str, Vec<f64>>>,
        orders: &[Option<&'a (Order, bool, Option<Trigger>)>],
        src: &[f64],
        signals: &[Signal],
        state: &TradeState,
    ) -> Option<&'a (Order, bool, Option<Trigger>)> {
        if state.positions.borrow().is_empty() {
            *bf.borrow_mut() = self.bf(orders, src, signals, state).into_inner();
        }
        let count_res = orders.len() as f64 + bf.borrow()["count"][0];
        if count_res <= self.max_count {
            bf.borrow_mut()
                .entry("count")
                .and_modify(|v| *v.get_mut(0).unwrap() = count_res);
            *orders.get(0).unwrap()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[test]
    fn bf_res_1() {
        assert_eq_pr!(
            COUNT { max_count: 2. }.bf(&[], &[], &[], &Default::default()),
            BF_ORDER_FILTER::from_iter([("count", vec![0.])])
        )
    }

    #[test]
    fn filter_res_1() {
        let bf = BF_ORDER_FILTER::from_iter([("count", vec![0.])]);
        let count = COUNT { max_count: 2. };
        assert_eq_pr!(
            count.filter(
                &bf,
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
                &bf,
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
                &bf,
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
        let bf = BF_ORDER_FILTER::from_iter([("count", vec![0.])]);
        let count = COUNT { max_count: 1. };
        assert_eq_pr!(
            count.filter(
                &bf,
                &[Some(&(Default::default()))],
                &[],
                &[],
                &Default::default(),
            ),
            Some(&(Default::default()))
        );
        assert_eq_pr!(
            count.filter(
                &bf,
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
        let bf = BF_ORDER_FILTER::from_iter([("count", vec![0.])]);
        let count = COUNT { max_count: 1. };
        assert_eq_pr!(
            count.filter(
                &bf,
                &[Some(&(Default::default()))],
                &[],
                &[],
                &Default::default(),
            ),
            Some(&(Default::default()))
        );
        assert_eq_pr!(
            count.filter(
                &bf,
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
