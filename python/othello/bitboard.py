from othello.coordinate import Coordinate

""" Represent a bitboard of Othello pieces (a 64-bit integer) """
class Bitboard:
    def __init__(self, u64: int) -> None:
        if u64 > 2**64-1:
            raise ValueError(f"Index {u64} can't be greater than 63 !")

        self.bits = u64

    """ Create a btiboard from a list of algebric coordinates """
    @classmethod
    def from_algebric_list(cls, algs: list) -> cls:
        bits: int = 0

        # for all algebric coordinates, get the linear index
        for alg in algs:
            bit: int = Coordinate.to_bitboard(63-Coordinate.from_algebric(alg))

            bits |= bit

        return Bitboard(bits)


    