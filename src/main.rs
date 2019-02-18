extern crate othlib;
use othlib::util::draw_board;

fn main() {
    let position = r#"--------
    --------
    --WB----
    --BW----
    -------
    --------
    --------
    --------"#;

    // let new_game = Board::new(8);
    // println!("{}", new_game);

    // let board = Board::load(position);
    // println!("{}", board);
    println!("{}", draw_board());
}
