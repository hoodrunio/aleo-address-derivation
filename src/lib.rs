//! Aleo address derivation from signature
//! 
//! This library provides functionality to derive an Aleo address from a given signature.

use wasm_bindgen::prelude::*;
use snarkvm_console_account::{Address, Signature};
use snarkvm_console_network::Testnet3 as Network;
use std::str::FromStr;

#[wasm_bindgen]
pub fn signature_to_address(signature_str: &str) -> Result<String, JsValue> {
    let signature = Signature::<Network>::from_str(signature_str)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let compute_key = signature.compute_key();

    let address = Address::try_from(&compute_key)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(address.to_string())
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}