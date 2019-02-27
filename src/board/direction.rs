/// Not possible to loop through enum variants yet
pub const DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::NE,
    Direction::E,
    Direction::SE,
    Direction::S,
    Direction::SW,
    Direction::W,
    Direction::NW,
];

/// All possible directions for moving pieces in a bitboard.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
