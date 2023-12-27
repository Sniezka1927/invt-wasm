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

use odra::{
    types::{U128, U256},
    OdraType,
};

construct_uint! {
    #[wasm_bindgen]
    #[derive(BorshSerialize, BorshDeserialize)]
    pub struct U448T(7);
}

// OK
#[wasm_bindgen]
pub struct Primitive {
    v: u64,
}

#[wasm_bindgen]
impl Primitive {
    #[wasm_bindgen(constructor)]
    pub fn new(v: u64) -> Primitive {
        Primitive { v }
    }

    #[wasm_bindgen(getter)]
    pub fn v(&self) -> u64 {
        self.v
    }
}

#[wasm_bindgen]
pub struct UINT {
    v: U448T,
}

// !OK
// https://github.com/rustwasm/wasm-bindgen/issues/111
#[wasm_bindgen]
impl UINT {
    #[wasm_bindgen(constructor)]
    pub fn new(v: u64) -> UINT {
        UINT::new(v)
    }

    #[wasm_bindgen(getter)]
    pub fn v(&self) -> U448T {
        self.v
    }
}

// OK
#[wasm_bindgen]
#[derive(OdraType, Default, Debug)]
pub struct B {
    v: U128,
}
// OK
#[derive(Default, Debug)]
#[wasm_bindgen]
pub struct C {
    v: U448T,
}

#[wasm_bindgen]
#[decimal(24, U448T)]
#[derive(OdraType, Default, Debug, PartialEq, Copy)]
pub struct D {
    v: U128,
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
