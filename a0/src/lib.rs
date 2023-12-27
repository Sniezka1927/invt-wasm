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
#[decimal(24)]
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub struct D {
    v: u64,
}

#[wasm_bindgen]
impl D {
    #[wasm_bindgen(constructor)]
    pub fn new(v: u64) -> D {
        D { v }
    }

    #[wasm_bindgen(getter)]
    pub fn v(&self) -> u64 {
        self.v
    }

    #[wasm_bindgen(setter)]
    pub fn set_v(&mut self, new_v: u64) {
        self.v = new_v;
    }
}

#[wasm_bindgen]
pub fn add(a: D, b: D) -> D {
    a.checked_add(b).unwrap()
}

#[wasm_bindgen]
pub fn multiply(a: D, b: D) -> D {
    a.mul_up(b)
}

#[wasm_bindgen]
pub fn big_mul(a: D, b: D) -> D {
    a.big_mul(b)
}
#[wasm_bindgen]
pub fn mul_up(a: D, b: D) -> D {
    a.mul_up(b)
}
#[wasm_bindgen]
pub fn from_integer(a: u64) -> D {
    D::from_integer(a)
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
