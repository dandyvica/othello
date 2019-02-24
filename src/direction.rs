/// All possible directions for moving pieces in a bitboard.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Right,
    Down_Right,
    Down,
    Down_Left,
    Left,
    Up_Left,
    Up,
    Up_Right,
}
