""" useful transformation static methods from (x,y) bitboard coordinates to linear index """


class Coordinate:
    """ Convert from linear list index to (x,y) coordinates """
    @staticmethod
    def from_linear(x: int) -> (int, int):
        return (x % 8, x // 8)

    """ Convert from (x,y) coordinates to linear """
    @staticmethod
    def to_linear(xy: (int, int)) -> int:
        return xy[0] + xy[1]*8       

    """ Convert from linear list index to (x,y) bitboard coordinates (A1 is square 63) """
    @staticmethod
    def to_bitboard(index: int) -> (int, int):
        # safeguard
        if index >= 64:
            raise ValueError(f"Index {index} can't be greater than 63 !")
        return Coordinate.from_linear(63 - index)

    """ Convert from an algebric coordinate (e.g.: A1 is (0,0) and H8 is (7,7)) """
    @staticmethod    
    def from_algebric(alg: str) -> (int, int):
        # get first and second coordinates
        x, y = alg

        # valid letters
        letters: list = list("ABCDEFGH")

        # sanity checks
        if x not in letters or not y.isdigit() or int(y) > 9 or int(y) == 0:
            raise ValueError(f"{alg} is not a valid algebric coordinate!")

        return (int(letters.index(x)), int(y)-1)

""" my unit tests """
import unittest
class Test(unittest.TestCase):
    def test_from_linear(self):
                self.assertEqual(Coordinate.from_linear(0), (0,0))
                self.assertEqual(Coordinate.from_linear(63), (7,7))

    def test_to_linear(self):
                self.assertEqual(Coordinate.to_linear((0,0)), 0)
                self.assertEqual(Coordinate.to_linear((7,7)), 63)

    def test_to_bitboard(self):
                self.assertEqual(Coordinate.to_bitboard(63), (0,0))
                self.assertEqual(Coordinate.to_bitboard(0), (7,7))
                with self.assertRaises(ValueError):
                    Coordinate.to_bitboard(64)
            
    def test_from_algebric(self):
                self.assertEqual(Coordinate.from_algebric("A1"), (0,0))
                self.assertEqual(Coordinate.from_algebric("H8"), (7,7))
                with self.assertRaises(ValueError):
                    Coordinate.from_algebric("L8")
                    Coordinate.from_algebric("A0")
                    Coordinate.from_algebric("A9")
                    Coordinate.from_algebric("a1")
                    Coordinate.from_algebric("a9")

if __name__=='__main__':
        unittest.main()