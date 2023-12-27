#![no_std]
// Decimal requirements
use decimal::*;
extern crate alloc;
use crate::alloc::string::ToString;
use core::convert::{TryFrom, TryInto};
// Traceable result
use traceable_result::*;

use serde::{Deserialize, Serialize};
use tsify::JsValueSerdeExt;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[decimal(1)]
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct D {
    v: u128,
}

// #[wasm_bindgen]
#[decimal(0)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenAmount {
    pub v: u128,
}

#[decimal(24)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SqrtPrice {
    pub v: u128,
}

#[decimal(6)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Liquidity {
    pub v: u128,
}

// Decimal type
#[wasm_bindgen]
pub fn send_example_to_js() -> Result<JsValue, JsValue> {
    let decimal = D::new(1267650600228229401496703205376u128);
    Ok(serde_wasm_bindgen::to_value(&decimal)?)
}

#[wasm_bindgen]
pub fn receive_example_from_js(val: JsValue) -> Result<JsValue, JsValue> {
    let received_value: D = serde_wasm_bindgen::from_value(val)?;
    let example: D = D::new(1267650600228229401496703205376u128);
    let result = example.checked_div(received_value).unwrap();
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

#[wasm_bindgen]
pub fn get_custom_struct(a: JsValue) {
    // let received_value: TokenAmount = serde_wasm_bindgen::from_value(a).unwrap();
    let _: TokenAmount = a.into_serde().unwrap();
    // let received_struct: SqrtPrice = a.into_serde().expect("Failed to deserialize");
}

#[wasm_bindgen]
pub fn get_delta_y(
    js_sqrt_price_a: JsValue,
    js_sqrt_price_b: JsValue,
    js_liquidity: JsValue,
    js_rounding_up: JsValue,
) -> Result<JsValue, JsValue> {
    let sqrt_price_a: SqrtPrice = js_sqrt_price_a.into_serde().unwrap();
    let sqrt_price_b: SqrtPrice = js_sqrt_price_b.into_serde().unwrap();
    let liquidity: Liquidity = js_liquidity.into_serde().unwrap();
    let rounding_up: bool = js_rounding_up.into_serde().unwrap();

    let delta: SqrtPrice = if sqrt_price_a > sqrt_price_b {
        sqrt_price_a - sqrt_price_b
    } else {
        sqrt_price_b - sqrt_price_a
    };

    let delta_y = match rounding_up {
        true => delta
            .big_mul_to_value_up(liquidity)
            .checked_add(SqrtPrice::almost_one())
            .unwrap()
            .checked_div(SqrtPrice::one())
            .unwrap(),
        false => delta
            .big_mul_to_value(liquidity)
            .checked_div(SqrtPrice::one())
            .unwrap(),
    };

    let result: TokenAmount = TokenAmount::new(delta_y.try_into().unwrap());

    Ok(serde_wasm_bindgen::to_value(&result)?)
    // Ok(serde_wasm_bindgen::to_value(&sqrt_price_a)?)
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
