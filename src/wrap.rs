use crate::prelude::*;

pub struct WRAP;

impl OrderFilter for WRAP {
    fn init_bf(&self) {}
    fn filter<'a>(
        &self,
        orders: &[Option<&'a OrderWrap>],
        _src: &[f64],
        _signals: &[Signal],
        _state: &TradeState,
    ) -> Option<&'a OrderWrap> {
        orders[0]
    }
}
