use pyo3::prelude::*;
use snarkvm_console_account::{Address, Signature};
use snarkvm_console_network::Testnet3 as Network;
use std::str::FromStr;

#[pyfunction]
fn signature_to_address(signature_str: &str) -> PyResult<String> {
    // İmzayı ayrıştır
    let signature = Signature::<Network>::from_str(signature_str)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Geçersiz imza: {}", e)))?;

    // İmzadan ComputeKey'i elde et
    let compute_key = signature.compute_key();

    // ComputeKey'den Address'i elde et
    let address = Address::try_from(&compute_key)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Adres türetilemedi: {}", e)))?;

    // Address'i string'e çevir
    Ok(address.to_string())
}

#[pymodule]
fn aleo_address_derivation(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(signature_to_address, m)?)?;
    Ok(())
}