use std::collections::HashMap;

use crate::color::Color;
use crate::direction::Direction;

pub struct BitBoard {
    board: HashMap<Color, u64>,
}

impl BitBoard {
    pub fn new() -> BitBoard {
        let mut h = HashMap::new();

        h.insert(
            Color::Black,
            0b00000000_00000000_00000000_00010000_00001000_00000000_00000000_00000000,
        );
        h.insert(
            Color::White,
            0b00000000_00000000_00000000_00001000_00010000_00000000_00000000_00000000,
        );

        BitBoard { board: h }
    }

    /// Count the number of free squares in an Othello bitboard
    ///
    /// # Examples
    /// ```
    /// use othlib::bitboard::BitBoard;
    ///
    /// let mut board = BitBoard::new();
    /// assert_eq!(board.nb_empty(), 8*8-4);
    /// ```    
    pub fn nb_empty(&self) -> u32 {
        (self.board[&Color::Black] | self.board[&Color::White]).count_zeros()
    }

    /// Count the number of occupied squares in an Othello bitboard
    ///
    /// # Examples
    /// ```
    /// use othlib::bitboard::BitBoard;
    ///
    /// let mut board = BitBoard::new();
    /// assert_eq!(board.nb_occupied(), 4);
    /// ```    
    pub fn nb_occupied(&self) -> u32 {
        64 - self.nb_empty()
    }

    pub fn shift(&self, dir: Direction, color: Color) -> u64 {
        let bb = self.board[&color];

        match dir {
            Direction::Right => (bb >> 1) & 0x7F7F7F7F7F7F7F7Fu64,
            Direction::Down_Right => (bb >> 9) & 0x007F7F7F7F7F7F7Fu64,
            Direction::Down => (bb >> 8) & 0xFFFFFFFFFFFFFFFFu64,
            Direction::Down_Left => (bb >> 7) & 0x00FEFEFEFEFEFEFEu64,
            Direction::Left => (bb << 1) & 0xFEFEFEFEFEFEFEFEu64,
            Direction::Up_Left => (bb << 9) & 0xFEFEFEFEFEFEFE00u64,
            Direction::Up => (bb << 8) & 0xFFFFFFFFFFFFFFFFu64,
            Direction::Up_Right => (bb << 7) & 0x7F7F7F7F7F7F7F00u64,
        }
    }

    pub fn set_value(&mut self, pieces: u64, color: Color) {
        *self.board.get_mut(&color).unwrap() = pieces;
    }
}
