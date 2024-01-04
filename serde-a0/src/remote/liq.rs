use core::convert::{TryFrom, TryInto};
use decimal::*;

#[decimal(6)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Liquidity {
    pub v: u128,
}
