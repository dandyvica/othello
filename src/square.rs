use std::fmt;

use crate::coordinate::Coordinate;
use crate::piece::Piece;

#[derive(Debug, Clone, PartialEq)]
// A square is either free, or occupied by a black or white piece
pub enum Status {
    Empty,
    Occupied(Piece),
}

// It's convenient to internally keep the coordinates of the square within the board
pub struct Square {
    pub status: Status,
    pub coord: Coordinate,
}

impl Square {
    pub fn new(s: Status, c: Coordinate) -> Square {
        Square {
            status: s,
            coord: c,
        }
    }

    pub fn is_white(&self) -> bool {
        self.status == Status::Occupied(Piece::White)
    }
    pub fn is_black(&self) -> bool {
        !self.is_white()
    }
    pub fn is_color(&self, p: Piece) -> bool {
        self.status == Status::Occupied(p)
    }
    pub fn is_free(&self) -> bool {
        self.status == Status::Empty
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum Square {
//     Empty,
//     Occupied(Piece),
// }

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.status {
            Status::Empty => write!(f, "-"),
            Status::Occupied(Piece::Black) => write!(f, "B"),
            Status::Occupied(Piece::White) => write!(f, "W"),
        }
    }
}

impl Default for Square {
    fn default() -> Self {
        Square {
            status: Status::Empty,
            coord: Coordinate::new(0, 0),
        }
    }
}
