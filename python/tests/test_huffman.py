import pytest
from hypothesis import assume, given, settings
from hypothesis import strategies as st

from codewars.huffman import Freqs, decode, encode, frequencies


@given(st.text())
@settings(max_examples=500)
def test_decode_inverts_encode(s: str) -> None:
    freqs = frequencies(s)
    bits = encode(freqs, s)
    assume(bits is not None)
    assert bits is not None
    assert decode(freqs, bits) == s


@pytest.fixture(name="text")
def example_text() -> str:
    return "aaaabcc"


@pytest.fixture(name="freqs")
def example_freqs(text: str) -> Freqs:
    return frequencies(text)


def test_example_freqs(freqs: Freqs) -> None:
    assert sorted(freqs) == [("a", 4), ("b", 1), ("c", 2)]


def test_example_encode_len(freqs: Freqs, text: str) -> None:
    encoded = encode(freqs, text)
    assert encoded is not None
    assert len(encoded) == 10


@pytest.mark.parametrize(
    "fs,s,expected",
    [
        ([("a", 1), ("b", 1)], "a", 1),
        ([("a", 1), ("b", 1)], "b", 1),
        ([("a", 1), ("b", 1), ("c", 2)], "a", 2),
        ([("a", 1), ("b", 1), ("c", 2)], "b", 2),
        ([("a", 1), ("b", 1), ("c", 2)], "c", 1),
    ],
)
def test_encode_len(fs: Freqs, s: str, expected: int) -> None:
    encoded = encode(fs, s)
    assert encoded is not None
    assert len(encoded) == expected


def test_empty_imput(freqs: Freqs) -> None:
    assert encode(freqs, "") == ""
    assert decode(freqs, "") == ""


@pytest.mark.parametrize("fs", [[], [("a", 1)]])
def test_insufficient_information(fs: Freqs) -> None:
    assert encode(fs, "") is None
    assert decode(fs, "") is None
