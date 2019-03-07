import unittest
import ctypes
from typing import List, Callable

from othello.coordinate import Coordinate

""" Used to manage direction shifts and cropping rows or cols from a board """
from typing import NamedTuple


class Direction(NamedTuple):
    shift: int
    crop: int


""" Represent a bitboard of Othello pieces (a 64-bit integer). Need to use ctype u64 type to deal
with negative issues
"""

MAX_U64_VALUE = 2**64-1

            # Direction::E => (value >> 1) & 0x7F7F7F7F7F7F7F7Fu64,
            # Direction::SE => (value >> 9) & 0x007F7F7F7F7F7F7Fu64,
            # Direction::S => (value >> 8) & 0xFFFFFFFFFFFFFFFFu64,
            # Direction::SW => (value >> 7) & 0x00FEFEFEFEFEFEFEu64,
            # Direction::W => (value << 1) & 0xFEFEFEFEFEFEFEFEu64,
            # Direction::NW => (value << 9) & 0xFEFEFEFEFEFEFE00u64,
            # Direction::N => (value << 8) & 0xFFFFFFFFFFFFFFFFu64,
            # Direction::NE => (value << 7) & 0x7F7F7F7F7F7F7F00u64,


class Bitboard:
    # list of possible directions in terms of shifts. Use a list for cardinal directions starting from N and clockwise
    directions = {
        'N':  Direction(-8, 0xFFFFFFFFFFFFFFFF),
        'NE': Direction(-7, 0x7F7F7F7F7F7F7F00),
        'E':  Direction( 1, 0x7F7F7F7F7F7F7F7F),
        'SE': Direction( 9, 0x007F7F7F7F7F7F7F),
        'S':  Direction( 8, 0xFFFFFFFFFFFFFFFF),
        'SW': Direction( 7, 0x00FEFEFEFEFEFEFE),
        'W':  Direction(-1, 0xFEFEFEFEFEFEFEFE),
        'NW': Direction(-9, 0xFEFEFEFEFEFEFE00),
    }

    # maximum values of a bitboard
    max_u64_value = 2**64-1

    """ a bitboard is defined by a 64-bit integer. As Python doesn't support 64-bit only integers, there's a need for a specific class """
    def __init__(self, u64: int) -> None:
        if u64 > Bitboard.max_u64_value or u64 < 0:
            raise ValueError(
                f"Index {u64} must be positive and can't be greater than 63 !")

        self.bits = u64

    """ Create a bitboard from a list of algebric coordinates """
    @classmethod
    def from_algebric_list(cls, algs: List[str]) -> "Bitboard":
        bits: int = 0

        # for all algebric coordinates, get the linear index
        for alg in algs:
            bit: int = 2**(63-Coordinate.to_linear(Coordinate.from_algebric(alg)))

            bits |= bit

        return cls(bits)

    """ empty bitboard """
    @classmethod
    def zero(cls) -> "Bitboard":
        return cls(0)

    """ full bitboard """
    @classmethod
    def max_value(cls) -> "Bitboard":
        return cls(Bitboard.max_u64_value)        

    """ overloaded operators """

    """ equality """

    def __eq__(self, other) -> bool:
        return self.bits == other.bits

    """ unequality """

    def __ne__(self, other) -> bool:
        return self.bits != other.bits

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

    """ bitwise xor """

    def __xor__(self, other: "Bitboard") -> "Bitboard":
        a_xor_b: int = self.bits ^ other.bits
        return Bitboard(ctypes.c_uint64(a_xor_b).value)

    """ bitwise left shift """

    def __lshift__(self, n: int) -> "Bitboard":
        # manage negative shifts
        if n < 0:
            return self.__rshift__(-n)

        a_shifted_b: int = self.bits << n
        return Bitboard(ctypes.c_uint64(a_shifted_b).value)

    """ bitwise right shift """

    def __rshift__(self, n: int) -> "Bitboard":
        # manage negative shifts
        if n < 0:
            return self.__lshift__(-n)

        a_shifted_b: int = self.bits >> n
        return Bitboard(ctypes.c_uint64(a_shifted_b).value)

    """ iterate of bits of a bitboard """
    def __iter__(self):
        # build the list of bit values
        bits = [int(i) for i in str(self)]
        
        # just return the iterator on that list
        return iter(bits)

    """ this is one is not matrix multiplication. it's overloaded here to defined a specific right shift & crop """

    def __matmul__(self, dir: str) -> "Bitboard":
        # sanity check
        if dir not in Bitboard.directions.keys():
            raise ValueError(f"Direction {dir} is unknown !")        

        direction = Bitboard.directions[dir]
        return ((self >> direction.shift) & Bitboard(direction.crop))

    """ display """

    def __str__(self) -> str:
        return '{0:064b}'.format(self.bits)

    """ return a list of 8 bits corresponding to a row in a board """
    def row(self, row: int) -> List[int]:
        # sanity check
        if row < 0 or row > 7:
            raise ValueError(f"Row number {row} is wrong !")

        # convert to array of bits
        bits = list(self)
        return bits[8*row:8*(row+1)]

    """ return a list of 8 bits corresponding to a row in a board """
    def col(self, col: int) -> List[int]:
        # sanity check
        if col < 0 or col > 7:
            raise ValueError(f"Col number {col} is wrong !")

        # convert to array of bits
        bits = list(self)
        return [bit for index,bit in enumerate(bits) if index % 8 == col ]     

    """ return list of indexes which equals 0 or 1 depending on argument """

    def bit_list(self, zero_or_one: str) -> List[int]:
        # convert to string
        converted: str = "{0:064b}".format(self.bits)

        # filter to get (index,char) tuples matching predicate
        filtered: List[int] = [i for i, c in enumerate(
            converted) if c == zero_or_one]

        return filtered

    """ gives adjacent squares for all directions or just one """
    def adjacent(self, dir: str = "") -> "Bitboard":
        # sanity check
        if dir != "" and dir not in Bitboard.directions.keys():
            raise ValueError(f"Direction {dir} is unknown !")

        # get adjacent for all directions
        if dir == "":
            adjacent = Bitboard(0)

            for _, direction in Bitboard.directions.items():
                adjacent |= ((self >> direction.shift) & Bitboard(direction.crop))
        
        # or a sinle one
        else:
            direction = Bitboard.directions[dir]
            adjacent = ((self >> direction.shift) & Bitboard(direction.crop))

        return adjacent

    """ ascii representation of a bitboard """
    def to_ascii(self) -> str:
        # get list of char bits
        bits = str(self)

        # split into chunks
        return '\n'.join([ bits[i:i+8] for i in range(0, 64, 8) ])

    """ give the possible moves as a bitboard """
    def possible_moves(self, opponent_player: "Bitboard") -> "Bitboard":
        # current player is ourselves
        current_player = self

        # get empty squares
        empty_squares = ~(current_player|opponent_player)

        # no possible move yet
        possible_moves = Bitboard.zero()    
        
        # the algorithm is called dumb fill. It's the same for all directions. The |= line is just the union of moves each time one is found
        # the gist of such an algorithm is the following: for a direction, follow opponent pieces till an empty square is found. It's then a possible
        # move
        for dir in Bitboard.directions.keys():
            # (current_player @ dir) gives the adjacent neighbour in the direction dir
            # and then is intersected with opponent pieces. If 0, it means the adjacent square has not opponent
            candidates = opponent_player & (current_player @ dir)

            # otherwise continue which means we found a neighbour which is an opponent          
            while candidates != Bitboard.zero():
                # so now if the adjacent a the opponent is empty, it's a possible move !
                possible_moves |= empty_squares & (candidates @ dir)

                # kepp on progessing on adjacent squares
                candidates = opponent_player & (candidates @ dir)
                
        return possible_moves


""" my unit tests """


class Test(unittest.TestCase):
    def test_init(self):
        value = Bitboard(1234)
        self.assertEqual(value.bits, 1234)

        with self.assertRaises(ValueError):
            Bitboard(-1)
            Bitboard(2**65)

    def test_from_algebric_list(self):
        value = Bitboard.from_algebric_list(["A1", "F8", "G8"])
        self.assertEqual(value.bits, 2**63+6)

        with self.assertRaises(ValueError):
            Bitboard.from_algebric_list(["A1", "F8", "Z8"])

    def test_eq(self):
        self.assertEqual(Bitboard(0), ~Bitboard.max_value())

    def test_neq(self):
        self.assertNotEqual(Bitboard(0), Bitboard.max_value())

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

        value = Bitboard(2**63)
        self.assertEqual((value << -1).bits, 2**62)        

    def test_rshift(self):
        value = Bitboard(MAX_U64_VALUE)
        self.assertEqual((value >> 64).bits, 0)

        value = Bitboard(1)
        self.assertEqual((value >> 1).bits, 0)

        value = Bitboard(1)
        self.assertEqual((value >> -1).bits, 2)

    def test_iter(self):
        self.assertEqual(list(Bitboard(0)), [0]*64)
        self.assertEqual(list(Bitboard.max_value()), [1]*64)

    def test_row(self):
        value = Bitboard(0)
        self.assertEqual(len(value.row(0)), 8)
        self.assertEqual(value.row(1), [0]*8)

        # detailed test
        value = Bitboard(0b00000000_11111111_11111111_11111111_11111111_11111111_11111111_00000000)
        self.assertEqual(value.row(0), [0]*8)        
        self.assertEqual(value.row(1), [1]*8)        
        self.assertEqual(value.row(2), [1]*8)        
        self.assertEqual(value.row(3), [1]*8)        
        self.assertEqual(value.row(4), [1]*8)        
        self.assertEqual(value.row(5), [1]*8)        
        self.assertEqual(value.row(6), [1]*8)        
        self.assertEqual(value.row(7), [0]*8)        

        with self.assertRaises(ValueError):
            value.row(-1)
            value.row(9)

    def test_col(self):
        value = Bitboard(0)
        self.assertEqual(len(value.col(0)), 8)
        self.assertEqual(value.col(1), [0]*8)

        # detailed test
        value = Bitboard(0b01111111_01111111_01111111_01111111_01111111_01111111_01111111_00000000)
        self.assertEqual(value.col(0), [0]*8)        
        self.assertEqual(value.col(1), [1]*7+[0])        
        self.assertEqual(value.col(2), [1]*7+[0])        
        self.assertEqual(value.col(3), [1]*7+[0])        
        self.assertEqual(value.col(4), [1]*7+[0])        
        self.assertEqual(value.col(5), [1]*7+[0])        
        self.assertEqual(value.col(6), [1]*7+[0])        
        self.assertEqual(value.col(7), [1]*7+[0])         

        with self.assertRaises(ValueError):
            value.col(-1)
            value.col(9)            

    def test_ascii(self):
        value = Bitboard(0b00000000_11111111_11111111_11111111_11111111_11111111_11111111_00000000)
        self.assertEqual(value.to_ascii(), "00000000\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n00000000")

    def test_special_shift(self):
        value = Bitboard.max_value()

        # test directions and then cols/rows
        shifted = value @ 'N'
        for i in range(0,7):
            self.assertEqual(shifted.row(i), [1]*8)
        self.assertEqual(shifted.row(7), [0]*8)   

        shifted = value @ 'NE'
        for i in range(0,7):
            self.assertEqual(shifted.row(i), [0]+[1]*7)
        self.assertEqual(shifted.row(7), [0]*8) 

        self.assertEqual(shifted.col(0), [0]*8)            
        for i in range(1,8):
            self.assertEqual(shifted.col(i), [1]*7+[0])
        
        shifted = value @ 'E'
        for i in range(0,8):
            self.assertEqual(shifted.row(i), [0]+[1]*7)

        self.assertEqual(shifted.col(0), [0]*8)            
        for i in range(1,8):
            self.assertEqual(shifted.col(i), [1]*8)
        
        shifted = value @ 'SE'
        self.assertEqual(shifted.row(0), [0]*8) 
        for i in range(1,8):
            self.assertEqual(shifted.row(i), [0]+[1]*7)
          
        self.assertEqual(shifted.col(0), [0]*8)          
        for i in range(1,8):
            self.assertEqual(shifted.col(i), [0]+[1]*7)   

        shifted = value @ 'S'
        self.assertEqual(shifted.row(0), [0]*8) 
        for i in range(1,8):
            self.assertEqual(shifted.row(i), [1]*8)
          
        for i in range(0,8):
            self.assertEqual(shifted.col(i), [0]+[1]*7) 

        shifted = value @ 'SW'
        self.assertEqual(shifted.row(0), [0]*8) 
        for i in range(1,8):
            self.assertEqual(shifted.row(i), [1]*7+[0])
          
        for i in range(0,7):
            self.assertEqual(shifted.col(i), [0]+[1]*7) 
        self.assertEqual(shifted.col(7), [0]*8)  

        shifted = value @ 'W'
        for i in range(0,8):
            self.assertEqual(shifted.row(i), [1]*7+[0])
          
        for i in range(0,7):
            self.assertEqual(shifted.col(i), [1]*8) 
        self.assertEqual(shifted.col(7), [0]*8)                                   

        shifted = value @ 'NW'
        for i in range(0,7):
            self.assertEqual(shifted.row(i), [1]*7+[0])
        self.assertEqual(shifted.row(7), [0]*8)
          
        for i in range(0,7):
            self.assertEqual(shifted.col(i), [1]*7+[0]) 
        self.assertEqual(shifted.col(7), [0]*8)         



if __name__ == '__main__':
    unittest.main()
