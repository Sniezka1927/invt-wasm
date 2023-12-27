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
#[decimal(24, u128)]
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
pub fn multiply(a: u64, b: u64) -> u64 {
    let tmp_a = D::new(a);
    let tmp_b = D::new(b);
    tmp_a.mul_up(tmp_b).v()
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
