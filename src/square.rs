use std::fmt;

use crate::piece::Piece;

#[derive(Debug, Clone, PartialEq)]
pub enum Square {
    Empty,
    Occupied(Piece),
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Empty => write!(f, "-"),
            Square::Occupied(Piece::Black) => write!(f, "B"),
            Square::Occupied(Piece::White) => write!(f, "W"),
        }
    }
}
