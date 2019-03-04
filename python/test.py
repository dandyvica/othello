import othello

board = othello.SVGBoard(600, 600)
board.reset()
board.draw_pieces_from_bitboard(othello.Bitboard(12345), othello.Color.red)
board.close()

#print(board)

Board10 = othello.SVGBoard.custom(5)
board10 = Board10(600,600, False)
board10.draw_piece_from_algebric("E4", othello.Color.red)
board10.draw_legend("this is a test")
board10.close()
print(board10)

print(othello.SVGBoard.combine("a", "b", "c"))



