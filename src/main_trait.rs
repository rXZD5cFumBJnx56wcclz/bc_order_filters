#![allow(non_camel_case_types)]

use std::any::Any;

use bc_utils_lg::structs::{
    signals::Signal,
    trade::{OrderWrap, TradeState},
};

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
