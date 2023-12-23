#![no_std]
use wasm_bindgen::prelude::*;
// Decimal requirements
use decimal::*;
extern crate alloc;
use crate::alloc::string::ToString;
use core::convert::{TryFrom, TryInto};
// Traceable result
use traceable_result::*;

use borsh::{BorshDeserialize, BorshSerialize};
use uint::construct_uint;

construct_uint! {
    #[derive(BorshSerialize, BorshDeserialize)]
    pub struct U448T(7);
}

// OK
#[wasm_bindgen]
pub struct Primitive {
    v: u64,
}

// OK
#[wasm_bindgen]
pub struct A {
    v: u128,
}

// OK
#[wasm_bindgen]
#[decimal(24)]
#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct D {
    v: u128,
}

pub fn traceable_result() -> TrackableResult<D> {
    let a = D::default();
    Ok(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        let a = D::default();
        let b = D::default();
        assert_eq!(a, b);
    }

    #[test]
    fn test_traceable_result() {
        let a = traceable_result();
        assert!(a.is_ok());
    }
}
