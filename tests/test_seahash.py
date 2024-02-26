from importlib.metadata import version

import sys

import pytest
import seahash


def test_hash():
    assert seahash.hash(b"123") == 17880861427118377629


def test_hash_seeded():
    assert seahash.hash_seeded(b"123", 4, 5, 6, 7) == 6322058387033640399


def test_version():
    assert seahash.__version__ == version("seahash")


def test_hashlib_compatible():
    s = seahash.SeaHash()
    s.update(b"123")
    assert s.intdigest() == seahash.hash(b"123")
    assert s.digest() == b"\x9d\n`)\xbb\x94%\xf8"
    assert s.hexdigest() == "f82594bb29600a9d"
    s.update(b"456")
    assert s.intdigest() == seahash.hash(b"123456")
    assert s.digest() == b"0|l\x17P\xeb+R"
    assert s.intdigest() == int.from_bytes(s.digest(), sys.byteorder)


@pytest.mark.skipif(
    sys.version_info < (3, 11),
    reason="requires hashlib.file_digest which is in python3.11 or higher",
)
def test_hashlib_file_digest_compatible(tmp_path):
    import hashlib

    p = tmp_path / "hello.txt"
    CONTENT = b"Hash me!"
    p.write_bytes(CONTENT)
    with p.open("rb") as f:
        s = hashlib.file_digest(f, lambda: seahash.SeaHash())
    assert s.intdigest() == seahash.hash(CONTENT)
    assert s.digest() == b"\x9d\n`)\xbb\x94%\xf8"
    assert s.hexdigest() == "f82594bb29600a9d"


def test_initial_state():
    assert seahash.SeaHash(b"123").intdigest() == seahash.hash(b"123")


def test_digest_size():
    s = seahash.SeaHash(b"123")
    assert len(s.digest()) == s.digest_size
