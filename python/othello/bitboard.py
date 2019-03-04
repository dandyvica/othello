import unittest
import ctypes
from typing import List, Callable

from othello.coordinate import Coordinate

""" Represent a bitboard of Othello pieces (a 64-bit integer). Need to use ctype u64 type to deal
with negative issues
"""

MAX_U64_VALUE = 2**64-1


class Bitboard:

    def __init__(self, u64: int) -> None:
        if u64 > MAX_U64_VALUE or u64 < 0:
            raise ValueError(f"Index {u64} can't be greater than 63 !")

        self.bits = u64

    """ Create a bitboard from a list of algebric coordinates """
    @classmethod
    def from_algebric_list(cls, algs: List[str]) -> "Bitboard":
        bits: int = 0

        # for all algebric coordinates, get the linear index
        for alg in algs:
            bit: int = Coordinate.to_bitboard(63-Coordinate.from_algebric(alg))

            bits |= bit

        return cls(bits)

    """ invert operator a.k.a. bitwise not """

    def __invert__(self) -> "Bitboard":
        return Bitboard(ctypes.c_uint64(~self.bits).value)

    """ bitwise and """

    def __and__(self, other: "Bitboard") -> "Bitboard":
        a_and_b: int = self.bits & other.bits
        return Bitboard(ctypes.c_uint64(a_and_b).value)

    """ bitwise or """

    def __or__(self, other: "Bitboard") -> "Bitboard":
        a_or_b: int = self.bits | other.bits
        return Bitboard(ctypes.c_uint64(a_or_b).value)

    """ bitwise left shift """

    def __lshift__(self, other: int) -> "Bitboard":
        a_shifted_b: int = self.bits << other
        return Bitboard(ctypes.c_uint64(a_shifted_b).value)

    """ bitwise right shift """

    def __rshift__(self, other: int) -> "Bitboard":
        a_shifted_b: int = self.bits >> other
        return Bitboard(ctypes.c_uint64(a_shifted_b).value)

    """ return list of indexes which equals 0 or 1 depending on argument """

    def bit_list(self, zero_or_one: str) -> List[int]:
        # convert to string
        converted: str = "{0:064b}".format(self.bits)

        # filter to get (index,char) tuples matching predicate
        filtered: List[int] = [i for i, c in enumerate(
            converted) if c == zero_or_one]

        return filtered


""" my unit tests """


class Test(unittest.TestCase):
    def test_invert(self):
        max_value = Bitboard(MAX_U64_VALUE)
        self.assertEqual((~max_value).bits, 0)

        min_value = Bitboard(0)
        self.assertEqual((~min_value).bits, MAX_U64_VALUE)

    def test_and(self):
        max_value = Bitboard(MAX_U64_VALUE)
        min_value = Bitboard(0)
        self.assertEqual((min_value & max_value).bits, 0)

    def test_or(self):
        max_value = Bitboard(MAX_U64_VALUE)
        min_value = Bitboard(0)
        self.assertEqual((min_value | max_value).bits, MAX_U64_VALUE)

    def test_lshift(self):
        value = Bitboard(MAX_U64_VALUE)
        self.assertEqual((value << 64).bits, 0)

        value = Bitboard(2**63)
        self.assertEqual((value << 1).bits, 0)

    def test_rshift(self):
        value = Bitboard(MAX_U64_VALUE)
        self.assertEqual((value >> 64).bits, 0)

        value = Bitboard(1)
        self.assertEqual((value >> 1).bits, 0)


if __name__ == '__main__':
    unittest.main()
