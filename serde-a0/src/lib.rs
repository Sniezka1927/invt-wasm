#![no_std]
// Decimal requirements
use decimal::*;
extern crate alloc;
use crate::alloc::string::ToString;
use core::convert::{TryFrom, TryInto};
use js_sys::BigInt;
// use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use traceable_result::*;
use tsify::JsValueSerdeExt;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[decimal(1)]
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct D {
    v: u128,
}

#[decimal(0)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Test {
    #[tsify(type = "BigInt")]
    pub v: u64,
}

#[decimal(0)]
#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Serialize,
    Deserialize,
    Tsify,
    scale::Encode,
    scale::Decode,
)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenAmount {
    #[tsify(type = "BN")]
    pub v: u128,
}

#[decimal(6)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Liquidity {
    #[tsify(type = "BN")]
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

#[decimal(24)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SqrtPrice {
    #[tsify(type = "number")]
    pub v: u128,
}

#[wasm_bindgen]
pub fn get_custom_struct(a: SqrtPrice) {}

#[wasm_bindgen]
pub fn get_delta_y(
    // js_sqrt_price_a: JsValue,
    // js_sqrt_price_b: JsValue,
    // js_liquidity: JsValue,
    // js_rounding_up: JsValue,
    sqrt_price_a: SqrtPrice,
    sqrt_price_b: SqrtPrice,
    liquidity: Liquidity,
    rounding_up: bool,
) -> Result<JsValue, JsValue> {
    // let sqrt_price_a: SqrtPrice = serde_wasm_bindgen::from_value(js_sqrt_price_a)?;
    // let sqrt_price_b: SqrtPrice = serde_wasm_bindgen::from_value(js_sqrt_price_b)?;
    // let liquidity: Liquidity = serde_wasm_bindgen::from_value(js_liquidity)?;
    // let rounding_up: bool = serde_wasm_bindgen::from_value(js_rounding_up)?;

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
