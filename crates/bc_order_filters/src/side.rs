use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct SIDE {
    pub side: String,
}

impl SIDE {
    pub fn new(side: String) -> Self {
        Self { side }
    }
}

impl OrderFilter for SIDE {
    fn init_bf(&self) {}
    fn filter<'a>(
        &self,
        orders: &[Option<&'a (Order, bool, Option<Trigger>)>],
        _src: &[f64],
        _signals: &[Signal],
        _state: &TradeState,
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
            SIDE {
                side: "buy".to_string()
            }
            .filter(
                &[Some(&(
                    Order {
                        side: "buy".to_string(),
                        ..Default::default()
                    },
                    Default::default(),
                    Default::default(),
                ))],
                &[],
                &[],
                &Default::default()
            ),
            Some(&(
                Order {
                    side: "buy".to_string(),
                    ..Default::default()
                },
                Default::default(),
                Default::default(),
            ))
        )
    }

    #[test]
    fn filter_res_2() {
        assert_eq_pr!(
            SIDE {
                side: "buy".to_string()
            }
            .filter(&[Default::default()], &[], &[], &Default::default()),
            None
        )
    }
}
