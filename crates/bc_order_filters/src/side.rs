use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct SIDE {
    pub side: String,
}

impl OrderFilter for SIDE {
    fn bf<'a>(
        &self,
        _: &[Option<&(Order, bool, Option<Trigger>)>],
        _: &[f64],
        _: &[Signal],
        _: &TradeState,
    ) -> BF_ORDER_FILTER<'a> {
        Default::default()
    }
    fn filter<'a>(
        &self,
        _: &RefCell<MAP<&str, Vec<f64>>>,
        orders: &[Option<&'a (Order, bool, Option<Trigger>)>],
        _: &[f64],
        _: &[Signal],
        _: &TradeState,
    ) -> Option<&'a (Order, bool, Option<Trigger>)> {
        let order_wrap = orders[0];
        if let Some(order) = order_wrap {
            if &order.0.side == &self.side {
                return Some(order);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[test]
    fn filter_res_1() {
        assert_eq_pr!(
            SIDE { side: "buy".to_string() }.filter(
                &Default::default(),
                &[Some(&(
                    Order { side: "buy".to_string(), ..Default::default() },
                    Default::default(),
                    Default::default(),
                ))],
                &[],
                &[],
                &Default::default()
            ),
            Some(&(
                Order { side: "buy".to_string(), ..Default::default() },
                Default::default(),
                Default::default(),
            ))
        )
    }

    #[test]
    fn filter_res_2() {
        assert_eq_pr!(
            SIDE { side: "buy".to_string() }.filter(
                &Default::default(),
                &[Default::default()],
                &[],
                &[],
                &Default::default()
            ),
            None
        )
    }
}
