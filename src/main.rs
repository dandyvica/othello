extern crate othlib;
use othlib::board::Board;

fn main() {
    let position = r#"--------
    --------
    --WB----
    --------
    -------
    --------
    --------
    --------"#;

    // let new_game = Board::new(8);
    // println!("{}", new_game);

    let board = Board::load(position);
    println!("{}", board);
}
