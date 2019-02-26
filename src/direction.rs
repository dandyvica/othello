/// Not possible to loop through enum variants yet
pub const DIRECTIONS: [Direction; 8] = [
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
];

/// All possible directions for moving pieces in a bitboard.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Up,
    UpRight,
}
