use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct PRIORITY {
    pub priority_in_position: usize,
    pub priority_out_of_position: usize,
}

impl PRIORITY {
    pub fn new(priority_in_position: usize, priority_out_of_position: usize) -> Self {
        Self {
            priority_in_position,
            priority_out_of_position,
        }
    }
}

impl OrderFilter for PRIORITY {
    fn init_bf(&self) {}
    fn filter<'a>(
        &self,
        orders: &[Option<&'a (Order, bool, Option<Trigger>)>],
        _src: &[f64],
        _signals: &[Signal],
        state: &TradeState,
    ) -> Option<&'a (Order, bool, Option<Trigger>)> {
        if orders
            .iter()
            .scan(0usize, |init, v| {
                if v.is_some() && v.unwrap().0.side.as_str() != "hold" {
                    *init += 1;
                }
                Some(*init)
            })
            .any(|v| v > 1)
        {
            if state.positions.borrow().is_empty() {
                orders[self.priority_out_of_position]
            } else {
                orders[self.priority_in_position]
            }
        } else {
            *orders
                .iter()
                .find(|v| {
                    if let Some(order) = v {
                        if order.0.side.as_str() != "hold" {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .unwrap_or(&orders[0])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[test]
    fn filter_res_1() {
        assert_eq_pr!(
            PRIORITY {
                priority_in_position: 1,
                priority_out_of_position: 1,
            }
            .filter(
                &[
                    Some(&(
                        Order {
                            side: "buy".to_string(),
                            qty: 1.,
                            ..Default::default()
                        },
                        Default::default(),
                        Default::default()
                    )),
                    Some(&(
                        Order {
                            side: "buy".to_string(),
                            qty: 2.,
                            ..Default::default()
                        },
                        Default::default(),
                        Default::default()
                    )),
                ],
                &[],
                &[],
                &Default::default()
            ),
            Some(&(
                Order {
                    side: "buy".to_string(),
                    qty: 2.,
                    ..Default::default()
                },
                Default::default(),
                Default::default()
            ))
        );
    }

    #[test]
    fn filter_res_2() {
        assert_eq_pr!(
            PRIORITY {
                priority_in_position: 1,
                priority_out_of_position: 1,
            }
            .filter(
                &[
                    Some(&(
                        Order {
                            side: "hold".to_string(),
                            qty: 1.,
                            ..Default::default()
                        },
                        Default::default(),
                        Default::default()
                    )),
                    Some(&(
                        Order {
                            side: "hold".to_string(),
                            qty: 2.,
                            ..Default::default()
                        },
                        Default::default(),
                        Default::default()
                    )),
                ],
                &[],
                &[],
                &Default::default()
            ),
            Some(&(
                Order {
                    side: "hold".to_string(),
                    qty: 1.,
                    ..Default::default()
                },
                Default::default(),
                Default::default()
            ))
        );
    }

    #[test]
    fn filter_res_3() {
        assert_eq_pr!(
            PRIORITY {
                priority_in_position: 1,
                priority_out_of_position: 0,
            }
            .filter(
                &[
                    Some(&(
                        Order {
                            side: "buy".to_string(),
                            qty: 1.,
                            ..Default::default()
                        },
                        Default::default(),
                        Default::default()
                    )),
                    Some(&(
                        Order {
                            side: "buy".to_string(),
                            qty: 2.,
                            ..Default::default()
                        },
                        Default::default(),
                        Default::default()
                    )),
                ],
                &[],
                &[],
                &Default::default()
            ),
            Some(&(
                Order {
                    side: "buy".to_string(),
                    qty: 1.,
                    ..Default::default()
                },
                Default::default(),
                Default::default()
            ))
        );
    }
}
