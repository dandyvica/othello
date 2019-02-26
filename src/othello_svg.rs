extern crate othlib;

use othlib::bitboard::BitBoard;
use othlib::color::Color;
use othlib::direction::Direction;
use othlib::svg_board::SVGBoard;
use othlib::util::Colorable;


fn main() {
    //let mut bb = BitBoard::new();
    //bb.set_value(u64::max_value(), Color::White);
    //let dir = bb.shift(Direction::Up_Right, Color::White);

    let mut svg = SVGBoard::new();

    //svg.draw_pieces_from_algebric(vec!["D4", "D5", "D6", "E5"], Color::White);
    //svg.draw_pieces_from_algebric(vec!["E4", "E6", "F6", "G6"], Color::Black);

    let black = u64::from_vec(vec![17,18,19,35]);
    let white = u64::from_vec(vec![20,27,28,36]);

    let moves = BitBoard::generate_moves(black,white);
    //svg.draw_pieces_from_u64(moves, Color::Black);

    svg.draw_bit_indexes();

    svg.draw_legend("Othello empty board");

    svg.close();

    svg.write("empty_board.svg");

    //println!("{}", svg.get_tags());
}
