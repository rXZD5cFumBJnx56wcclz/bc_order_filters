#![allow(non_camel_case_types)]

use crate::prelude::*;

pub trait OrderFilter: Any {
    fn init_bf(&self);
    fn filter<'a>(
        &self,
        orders: &[Option<&'a OrderWrap>],
        src: &[f64],
        signals: &[Signal],
        state: &TradeState,
    ) -> Option<&'a OrderWrap>;
}
