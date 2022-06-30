use pyo3::prelude::*;

/// Hash some bytes
#[pyfunction]
fn hash(buf: &[u8]) -> PyResult<u64> {
    Ok(seahash::hash(buf))
}

/// Hash some bytes according to a chosen seed.
#[pyfunction]
fn hash_seeded(buf: &[u8], a: u64, b: u64, c: u64, d: u64) -> PyResult<u64> {
    Ok(seahash::hash_seeded(buf, a, b, c, d))
}

/// A Python module implemented in Rust.
#[pymodule]
fn seahash(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(hash, m)?)?;
    m.add_function(wrap_pyfunction!(hash_seeded, m)?)?;
    Ok(())
}
