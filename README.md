# SeaHash

[![CI](https://github.com/RealOrangeOne/seahash-py/actions/workflows/ci.yml/badge.svg)](https://github.com/RealOrangeOne/seahash-py/actions/workflows/ci.yml)
![PyPI](https://img.shields.io/pypi/v/seahash.svg)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/seahash.svg)
![PyPI - Wheel](https://img.shields.io/pypi/wheel/seahash.svg)
![PyPI - Status](https://img.shields.io/pypi/status/seahash.svg)
![PyPI - License](https://img.shields.io/pypi/l/seahash.svg)


Python bindings to [`seahash`](https://docs.rs/seahash/) - A blazingly fast, portable hash function with proven statistical guarantees.

## Installation

```
pip install seahash
```

Compiling from source will require a Rust toolchain.

## Usage

Currently, only the 2 primitive methods are available:

```python
import seahash

# Plain hash
seahash.hash(b"123")

# Hash with custom seeds
seahash.hash_seeded(b"123", 4, 5, 6, 7)
```

Both methods return an `int`.
