use pyo3::prelude::*;
mod inner {
    use pyo3::prelude::*;
    use pyo3::types::PyBytes;

    /// Hash some bytes
    #[pyfunction]
    pub(crate) fn hash(py: Python, buf: &[u8]) -> u64 {
        py.allow_threads(|| seahash::hash(buf))
    }

    /// Hash some bytes according to a chosen seed.
    #[pyfunction]
    pub(crate) fn hash_seeded(py: Python, buf: &[u8], a: u64, b: u64, c: u64, d: u64) -> u64 {
        py.allow_threads(|| seahash::hash_seeded(buf, a, b, c, d))
    }

    #[pyclass]
    pub(crate) struct SeaHash {
        inner: Vec<u8>,
    }

    #[pymethods]
    impl SeaHash {
        #[pyo3(name = "digest_size")]
        #[classattr]
        const DIGEST_SIZE: u8 = 8;

        #[pyo3(name = "block_size")]
        #[classattr]
        const BLOCK_SIZE: u8 = 8;

        #[new]
        #[pyo3(signature = (string = Vec::new()))]
        fn new(string: Vec<u8>) -> Self {
            SeaHash { inner: string }
        }

        pub fn update(&mut self, obj: &[u8]) {
            self.inner.extend_from_slice(obj)
        }

        pub fn digest(&self, py: Python) -> PyObject {
            PyBytes::new(py, &self.intdigest(py).to_ne_bytes()).into()
        }

        pub fn intdigest(&self, py: Python) -> u64 {
            hash(py, &self.inner)
        }

        pub fn hexdigest(&self, py: Python) -> String {
            format!("{:x}", self.intdigest(py))
        }

        pub fn copy(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
use inner::{hash, hash_seeded, SeaHash};
/// A Python module implemented in Rust.
#[pymodule]
fn seahash(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(hash, m)?)?;
    m.add_function(wrap_pyfunction!(hash_seeded, m)?)?;
    m.add_class::<SeaHash>()?;
    Ok(())
}
