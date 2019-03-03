import othello

board = othello.SVGBoard(600, 600)
board.draw_pieces_from_bitboard(2**64-1, othello.Color.red)
board.close()

print(board)
