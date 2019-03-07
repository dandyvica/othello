import othello

board = othello.SVGBoard(600, 600)

# example position
bits_b = othello.Bitboard(0b01000000_11011110_01000110_00101110_00011010_00011100_00000000_00000000)
bits_w = othello.Bitboard(0b00111110_00100000_00111000_00010000_00100000_00000000_00000000_00000000)

board.draw_pieces_from_bitboard(bits_b, othello.Color.black)
board.draw_pieces_from_bitboard(bits_w, othello.Color.white)

board.draw_pieces_from_bitboard(bits_w.possible_moves(bits_b), othello.Color.red, 0.1)   
board.close()



