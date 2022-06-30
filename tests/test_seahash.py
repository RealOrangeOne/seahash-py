import seahash
import pkg_resources


def test_hash():
    assert seahash.hash(b"123") == 17880861427118377629


def test_hash_seeded():
    assert seahash.hash_seeded(b"123", 4, 5, 6, 7) == 6322058387033640399


def test_version():
    assert seahash.__version__ == pkg_resources.get_distribution("seahash").version
