extern crate othlib;

use othlib::color::Color;
use othlib::svg_board::SVGBoard;
use othlib::util::Colorable;
use othlib::bitboard::BitBoard;
use othlib::direction::Direction;

fn main() {

    let mut bb = BitBoard::new();
    bb.set_value(u64::max_value(), Color::White);
    let dir = bb.shift(Direction::Up_Right, Color::White);

    let mut svg = SVGBoard::new();

    svg.draw_pieces_u64(dir, Color::White);

    svg.draw_bit_indexes();
    svg.close();

    println!("{}", svg.get_tags());
}


// pub enum Direction {
//     Right,
//     Down_Right,
//     Down,
//     Down_Left,
//     Left,
//     Up_Left,
//     Up,
//     Up_Right,
// }