import string

from othello.svg_doc import SVGDoc
from othello.color import Color
from othello.coordinate import Coordinate
from othello.bitboard import Bitboard

""" Used to draw an empty Othello standard 8x8 board in SVG """


class SVGBoard:
    # regular board size which could be modified by classmethod to build specific board sizes """
    board_size = 8
    board_dimension = 64

    def __init__(self, width: int, height: int, draw_headers: bool = True) -> None:
        # save with & height for reuse in reset() method
        self.width = width
        self.height = height
        self.draw_headers = draw_headers

        self.doc = self.__build_empty(width, height)

    """ build a specific SVGBoard of size n """
    @classmethod
    def custom(cls, n: int) -> "SVGBoard":
        # modify sizes
        cls.board_size = n
        cls.board_dimension = n*n

        return cls

    """ combine 2 SVG outputs side by side """
    @staticmethod
    def combine(*svgs) -> None:
        s = ''
        for svg in svgs:
            s += f'{svg}'
        return f'<div style="white-space: nowrap">{s}</div>'

    """ reset is rebuilding an empty board """

    def reset(self, draw_headers: bool = True):
        self.draw_headers = draw_headers        
        self.doc = self.__build_empty(self.width, self.height)

    """ draw pieces from a bitboard (64 bits unsigned integer) with radius """

    def draw_pieces_from_bitboard(self, bitboard: "Bitboard", color: Color, radius: int = 0.4) -> None:
        for i in bitboard.bit_list('1'):
            self.draw_piece(Coordinate.to_bitboard(
                SVGBoard.board_dimension-1-i), color, radius)

    """ draw piece at algebric coordinate """

    def draw_piece_from_algebric(self, alg_coord: str, color: Color) -> None:
        self.draw_piece(Coordinate.from_algebric(alg_coord), color, 0.4)

    """ draw a single piece at (x,y) where (0,0) is the top leftmost square (= A1) """

    def draw_piece(self, coord: (int, int), color: Color, radius: int) -> None:
        # draw piece as a circle
        self.doc.draw('circle', cx=(
            coord[0]+1.5), cy=(coord[1]+1.5), r=radius, style=f"fill:{color.name}")

    """ draw a text at (x,y) where (0,0) is the top leftmost square (= A1) """

    def draw_text(self, coord: (int, int), text: str) -> None:
        self.doc.draw('text', **{"x": coord[0], "y": coord[1], "text-anchor": "middle",
                                 "style": "font-family:Verdana;font-size:0.5px;fill:black", "content": text})

    """ draw text below the board """

    def draw_legend(self, legend: str) -> None:
        # pass kw args as dict because of the 'text-anchor' containing minus sign
        self.draw_text(((SVGBoard.board_size+2)//2, SVGBoard.board_size+2-0.4), legend)

    """ close SVG tags """

    def close(self) -> None:
        self.doc.close()

    """ get SVG tags """

    def __str__(self) -> str:
        return self.doc.tags

    """ build new SGV empty board. This is used by ctor and reset functions """

    def __build_empty(self, width: int, height: int) -> str:
        view_box = f"0 0 {SVGBoard.board_size+2} {SVGBoard.board_size+2}"

        svgdoc = SVGDoc(viewBox=view_box, width=width, height=height)

        # draw board background
        svgdoc.draw('rect', x=1, y=1, width=SVGBoard.board_size,
                    height=SVGBoard.board_size, style="fill:#009000;")

        # need headers?
        if self.draw_headers:

            # row headers
            for row in range(1, SVGBoard.board_size+1):
                svgdoc.draw('text', x=0.4, y=(row + 0.6),
                            style="font-family:Verdana;font-size:0.5px", content=str(row))

            # col headers
            letters = list(string.ascii_uppercase)[0:SVGBoard.board_size]
            for i, col in enumerate(letters):
                svgdoc.draw('text', x=(i+1.3), y=0.8,
                            style="font-family:Verdana;font-size:0.5px", content=str(col))

        # horizontal grid: first rule is different to match perfectly
        svgdoc.draw('line', x1=1, y1=1.025, x2=SVGBoard.board_size+1, y2=1.025,
                    style="stroke:black;stroke-width:0.05")
        for row in range(2, SVGBoard.board_size+2):
            svgdoc.draw('line', x1=1, y1=str(row), x2=SVGBoard.board_size+1, y2=str(row),
                        style="stroke:black;stroke-width:0.05")

        # vertical grid: last rule is different
        for col in range(1, SVGBoard.board_size+1):
            svgdoc.draw('line', x1=str(col), y1=1, x2=str(col), y2=SVGBoard.board_size+1.025,
                        style="stroke:black;stroke-width:0.05")
        svgdoc.draw('line', x1=SVGBoard.board_size+0.975, y1=1, x2=SVGBoard.board_size+1, y2=SVGBoard.board_size+1.025,
                    style="stroke:black;stroke-width:0.05")

        return svgdoc
