use pyo3::prelude::*;
use pyo3::types::PyBytes;

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

#[pyclass]
struct SeaHash {
    inner: Vec<u8>,
}

#[pymethods]
impl SeaHash {
    #[pyo3(name = "digest_size")]
    #[classattr]
    const DIGEST_SIZE: u8 = 64;

    #[pyo3(name = "block_size")]
    #[classattr]
    const BLOCK_SIZE: u8 = 64;

    #[new]
    #[args(string = "Vec::new()")]
    fn new(string: Vec<u8>) -> Self {
        SeaHash { inner: string }
    }

    pub fn update(&mut self, obj: &[u8]) {
        self.inner.extend_from_slice(obj)
    }

    pub fn digest(&self, py: Python) -> PyObject {
        PyBytes::new(py, &self.intdigest().to_ne_bytes()).into()
    }

    pub fn intdigest(&self) -> u64 {
        seahash::hash(&self.inner)
    }

    pub fn hexdigest(&self) -> String {
        format!("{:x}", self.intdigest())
    }

    pub fn copy(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn seahash(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(hash, m)?)?;
    m.add_function(wrap_pyfunction!(hash_seeded, m)?)?;
    m.add_class::<SeaHash>()?;
    Ok(())
}
