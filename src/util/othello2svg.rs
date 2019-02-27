// Simple utility to convert an Othello board to an SVG file representation.

use std::env;

extern crate othlib;

use othlib::board::bitboard::BitBoard;
use othlib::board::color::Color;
use othlib::board::direction::Direction;
use othlib::svg::svg_board::SVGBoard;
use othlib::svg::vecint64::VecInt64;
use othlib::util::from_vec::FromVec;

// single argument: output file location
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
            println!("othello2svg <svg_file>");
            ::std::process::exit(1);
    }

    // file name is the 2nd argument
    let svg_file = &args[1];

    // //let mut bb = BitBoard::new();
    // //bb.set_value(u64::max_value(), Color::White);
    // //let dir = bb.shift(Direction::Up_Right, Color::White);

    // //svg.draw_pieces_from_algebric(vec!["D4", "D5", "D6", "E5"], Color::White);
    // //svg.draw_pieces_from_algebric(vec!["E4", "E6", "F6", "G6"], Color::Black);

    // //let black = u64::from_vec(vec![17, 18, 19, 35]);
    // //let white = u64::from_vec(vec![20, 27, 28, 36]);

    // //let moves = BitBoard::generate_moves(black,white);

    // //svg.draw_bit_indexes();

    // // create brand new empty board
    // let mut svg = SVGBoard::new();
    // let mut v = VecInt64::new();    

    // // South    

    // // draw pieces depending on set bits in the u64 value
    // let mask = 0b11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110;
    // let value = (u64::max_value() << 1) & mask;
    // svg.draw_pieces_from_u64(value, Color::White);

    // // trick to keep track of the piece number
    // let mut v = VecInt64::new();
    // svg.draw_pieces_from_vecint64(v.lshift(1).mask(mask));

    // // add identification
    // svg.draw_legend("Left");

    // // write out file
    // svg.close();
    // svg.write(svg_file);

    // //println!("{}", svg.get_tags());

    generate_svg(8, 'R', u64::max_value(), "Moving South", "doc/images/full_board_south.svg");
    generate_svg(8, 'L', u64::max_value(), "Moving North", "doc/images/full_board_north.svg");
    generate_svg(1, 'R', 0b01111111_01111111_01111111_01111111_01111111_01111111_01111111_01111111, "Moving East", "doc/images/full_board_east.svg");
    generate_svg(1, 'L', 0b11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110, "Moving West", "doc/images/full_board_west.svg");
    generate_svg(9, 'R', 0b00000000_01111111_01111111_01111111_01111111_01111111_01111111_01111111, "Moving South East", "doc/images/full_board_south_east.svg");
    generate_svg(7, 'R', 0b00000000_11111110_11111110_11111110_11111110_11111110_11111110_11111110, "Moving South West", "doc/images/full_board_south_west.svg");
    generate_svg(9, 'L', 0b11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110, "Moving North West", "doc/images/full_board_north_west.svg");
    generate_svg(7, 'L', 0b01111111_01111111_01111111_01111111_01111111_01111111_01111111_01111111, "Moving North East", "doc/images/full_board_north_east.svg");

}

pub fn generate_svg(shift: usize, dir: char, mask: u64, legend: &str, filename: &str) {

    let mut svg = SVGBoard::new();
    let mut v = VecInt64::new();    

    // compute value and vecint64
    let value = match dir {
        'L' => (u64::max_value() << shift) & mask,
        'R' => (u64::max_value() >> shift) & mask,
        _ => unimplemented!("Not implemented!"),
    };

    match dir {
        'L' => v.lshift(shift).mask(mask),
        'R' => v.rshift(shift).mask(mask),
        _ => unimplemented!("Not implemented!"),
    };    

    // draw pieces depending on set bits in the u64 value
    svg.draw_pieces_from_u64(value, Color::White);

    // trick to keep track of the piece number
    svg.draw_pieces_from_vecint64(&mut v);

    // add identification
    svg.draw_legend(legend);

    // write out file
    svg.close();
    svg.write(filename);    
}
