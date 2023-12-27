#![no_std]
// Decimal requirements
use decimal::*;
extern crate alloc;
use crate::alloc::string::ToString;
use core::convert::{TryFrom, TryInto};
// Traceable result
use traceable_result::*;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use tsify::JsValueSerdeExt;
use tsify::Tsify;
use uint::construct_uint;
use wasm_bindgen::prelude::*;

use odra::types::{U128, U256, U512};
use odra::OdraType;
construct_uint! {
    #[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
    pub struct U256T(4);
}

construct_uint! {
    #[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
    pub struct U384T(6);
}
// // Decimal - required a0 decimal directory
// #[wasm_bindgen]
// #[decimal(0)]
// #[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
// pub struct D {
//     v: u128,
// }

// CSPR
#[wasm_bindgen]
#[decimal(1, U256T)]
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct D {
    v: U128,
}

// #[wasm_bindgen]
#[decimal(24, U384T)]
#[derive(
    OdraType, Default, Debug, Copy, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Tsify,
)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SqrtPrice {
    pub v: U128,
}

// #[wasm_bindgen]
#[decimal(5, U512)]
#[derive(
    OdraType, Default, Debug, Copy, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Tsify,
)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Liquidity {
    pub v: U256,
}

// #[wasm_bindgen]
#[decimal(0, U512)]
#[derive(
    OdraType, Default, Debug, Copy, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Tsify,
)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenAmount {
    pub v: U256,
}

// Decimal type
#[wasm_bindgen]
pub fn send_example_to_js() -> Result<JsValue, JsValue> {
    let decimal = D::new(U128::from(1267650600228229401496703205376u128));
    Ok(serde_wasm_bindgen::to_value(&decimal)?)
}

#[wasm_bindgen]
pub fn receive_example_from_js(val: JsValue) -> Result<JsValue, JsValue> {
    let received_value: D = serde_wasm_bindgen::from_value(val)?;
    let example: D = D::new(U128::from(1267650600228229401496703205376u128));
    let result = example.checked_div(received_value).unwrap();
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

#[wasm_bindgen]
pub fn get_custom_struct(a: SqrtPrice) {
    // let received_struct: SqrtPrice = a.into_serde().expect("Failed to deserialize");
    // let _ = received_struct.checked_add(SqrtPrice::one());
    // panic!("Received struct is: {:?}", a);
    // if received_struct > SqrtPrice::one() {
    //     let _ = received_struct.checked_div(SqrtPrice::one());
    // } else {
    //     let _ = SqrtPrice::one().checked_div(received_struct);
    // }
}

#[wasm_bindgen]
pub fn get_delta_y(
    js_sqrt_price_a: JsValue,
    js_sqrt_price_b: JsValue,
    js_liquidity: JsValue,
    js_rounding_up: JsValue,
) -> Result<JsValue, JsValue> {
    // let sqrt_price_a: SqrtPrice = serde_wasm_bindgen::from_value(js_sqrt_price_a)?;
    let sqrt_price_a: SqrtPrice = js_sqrt_price_a.into_serde().unwrap();
    let sqrt_price_b: SqrtPrice = serde_wasm_bindgen::from_value(js_sqrt_price_b)?;
    let liquidity: Liquidity = serde_wasm_bindgen::from_value(js_liquidity)?;
    let rounding_up: bool = serde_wasm_bindgen::from_value(js_rounding_up)?;

    let delta: SqrtPrice = if sqrt_price_a > sqrt_price_b {
        sqrt_price_a - sqrt_price_b
    } else {
        sqrt_price_b - sqrt_price_a
    };

    let delta_y = match rounding_up {
        true => delta
            .big_mul_to_value_up(liquidity)
            .checked_add(SqrtPrice::almost_one().cast())
            .unwrap()
            .checked_div(SqrtPrice::one().cast())
            .unwrap(),
        false => delta
            .big_mul_to_value(liquidity)
            .checked_div(SqrtPrice::one().cast())
            .unwrap(),
    };

    let result: TokenAmount = TokenAmount::new(TokenAmount::checked_from_value(delta_y).unwrap());

    Ok(serde_wasm_bindgen::to_value(&result)?)
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
