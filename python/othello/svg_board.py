from othello.svg_doc import SVGDoc
from othello.color import Color
from othello.coordinate import Coordinate

""" Used to draw an empty Othello standard 8x8 board in SVG """


class SVGBoard:
    def __init__(self, width: int, height: int) -> None:
        self.doc = SVGDoc(viewBox="0 0 10 10", width=width, height=height)

        # draw board background
        self.doc.draw('rect', x=1, y=1, width=8,
                      height=8, style="fill:#009000;")

        # row headers
        for row in range(1, 9):
            self.doc.draw('text', x=0.4, y=(row + 0.6),
                          style="font-family:Verdana;font-size:0.5px", content=str(row))

        # col headers
        for i, col in enumerate(list('ABCDEFGH')):
            self.doc.draw('text', x=(i+1.3), y=0.8,
                          style="font-family:Verdana;font-size:0.5px", content=str(col))

        # horizontal grid: first rule is different to match perfectly
        self.doc.draw('line', x1=1, y1=1.025, x2=9, y2=1.025,
                      style="stroke:black;stroke-width:0.05")
        for row in range(2, 10):
            self.doc.draw('line', x1=1, y1=str(row), x2=9, y2=str(row),
                          style="stroke:black;stroke-width:0.05")

        # vertical grid: last rule is different
        for col in range(1, 9):
            self.doc.draw('line', x1=str(col), y1=1, x2=str(col), y2=9,
                          style="stroke:black;stroke-width:0.05")
        self.doc.draw('line', x1=8.975, y1=1, x2=9, y2=8.975,
                      style="stroke:black;stroke-width:0.05")

    """ draw pieces from a bitboard (64 bits unsigned integer) """

    def draw_pieces_from_bitboard(self, bitboard: int, color: Color) -> None:
        # check whether the integer is not over 64 bits
        if bitboard > 2 ** 64-1:
            raise ValueError(f"{bitboard} is not a 64 bits integer!")

        # convert into a binary string
        converted: str = "{0:064b}".format(bitboard)

        # loop through bits and draw a circle whose color is the one of the `color` argument
        for i, c in enumerate(converted):
            # if bit is '1', draw a circle
            if c == '1':
                # calculate coordinates
                coord: (int, int) = Coordinate.to_bitboard(63-i)

                # draw piece
                self.draw_piece(coord, color)

    """ draw piece at algebric coordinate """

    def draw_piece_algebric(self, alg_coord: str, color: Color) -> None:
        self.draw_piece(Coordinate.from_algebric(alg_coord), color)

    """ draw a single piece at (x,y) where (0,0) is the top leftmost square (= A1) """

    def draw_piece(self, coord: (int, int), color: Color) -> None:
        #style: str = format(f"fill:{color.name}")

        # draw piece as a circle
        self.doc.draw('circle', cx=(
            coord[0]+1.5), cy=(coord[1]+1.5), r=0.4, style=f"fill:{color.name}")

    """ draw text below the board """

    def draw_legend(self, legend: str) -> None:
        # pass kw args as dict because of the 'text-anchor' containing minus sign
        self.doc.draw('text', **{"x": 5, "y": 9.6, "text-anchor": "middle", "style": "font-family:Verdana;font-size:0.5px;fill:black", "content": legend})

    """ close SVG tags """

    def close(self) -> None:
        self.doc.close()

    """ get SVG tags """

    def __str__(self) -> str:
        return self.doc.tags
