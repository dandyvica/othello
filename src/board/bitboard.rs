use std::collections::HashMap;

use crate::board::color::Color;
use crate::board::direction::{Direction, DIRECTIONS};

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
    /// use othlib::board::bitboard::BitBoard;
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
    /// use othlib::board::bitboard::BitBoard;
    ///
    /// let mut board = BitBoard::new();
    /// assert_eq!(board.nb_occupied(), 4);
    /// ```    
    pub fn nb_occupied(&self) -> u32 {
        64 - self.nb_empty()
    }

    pub fn shift(&self, dir: &Direction, color: Color) -> u64 {
        let bb = self.board[&color];

        BitBoard::shift_one_direction(dir, bb)
    }

    pub fn set_value(&mut self, pieces: u64, color: Color) {
        *self.board.get_mut(&color).unwrap() = pieces;
    }

    /// Shift player pieces into one of the 8 possible directions
    ///
    /// Examples
    ///
    /// use othlib::util::from_vec::FromVec;    
    ///
    /// let full = u64::max_value();
    ///
    /// let mut result = !u64::from_vec(vec![63,62,61,60,59,58,57,56]);
    /// assert_eq!(BitBoard::shift_one_direction(&Direction::Down, full), result);
    ///
    /// result = !u64::from_vec(vec![0,1,2,3,4,5,6,7]);
    /// assert_eq!(BitBoard::shift_one_direction(&Direction::Up, full), result);    
    ///
    /// result = !u64::from_vec(vec![63,55,47,39,31,23,15,7]);
    /// assert_eq!(BitBoard::shift_one_direction(&Direction::Right, full), result);    
    ///
    /// result = !u64::from_vec(vec![56,48,40,32,24,16,8,0]);
    /// assert_eq!(BitBoard::shift_one_direction(&Direction::Left, full), result);    
    ///
    /// result = !u64::from_vec(vec![63,62,61,60,59,58,57,56,55,47,39,31,23,15,7]);
    /// assert_eq!(BitBoard::shift_one_direction(&Direction::DownRight, full), result);    
    ///
    /// result = !u64::from_vec(vec![63,62,61,60,59,58,57,56,48,40,32,24,16,8,0]);
    /// assert_eq!(BitBoard::shift_one_direction(&Direction::DownLeft, full), result);    
    ///
    /// result = !u64::from_vec(vec![0,1,2,3,4,5,6,7,56,48,40,32,24,16,8]);
    /// assert_eq!(BitBoard::shift_one_direction(&Direction::UpLeft, full), result);    
    ///
    /// result = !u64::from_vec(vec![0,1,2,3,4,5,6,7,63,55,47,39,31,23,15]);
    /// assert_eq!(BitBoard::shift_one_direction(&Direction::UpRight, full), result);    
    /// ```
    pub fn shift_one_direction(dir: &Direction, value: u64) -> u64 {
        match dir {
            Direction::E => (value >> 1) & 0x7F7F7F7F7F7F7F7Fu64,
            Direction::SE => (value >> 9) & 0x007F7F7F7F7F7F7Fu64,
            Direction::S => (value >> 8) & 0xFFFFFFFFFFFFFFFFu64,
            Direction::SW => (value >> 7) & 0x00FEFEFEFEFEFEFEu64,
            Direction::W => (value << 1) & 0xFEFEFEFEFEFEFEFEu64,
            Direction::NW => (value << 9) & 0xFEFEFEFEFEFEFE00u64,
            Direction::N => (value << 8) & 0xFFFFFFFFFFFFFFFFu64,
            Direction::NE => (value << 7) & 0x7F7F7F7F7F7F7F00u64,
        }
    }

    pub fn generate_moves(my_disks: u64, opp_disks: u64) -> u64 {
        let empty_cells = !(my_disks | opp_disks);
        let mut legal_moves = 0u64;

        //assert((my_disks & opp_disks) == 0 && "Disk sets should be disjoint.");

        for dir in &DIRECTIONS {
            let mut x = BitBoard::shift_one_direction(dir, my_disks) & opp_disks;

            /* Add opponent disks adjacent to those, and so on. */
            x |= BitBoard::shift_one_direction(dir, x) & opp_disks;
            x |= BitBoard::shift_one_direction(dir, x) & opp_disks;
            x |= BitBoard::shift_one_direction(dir, x) & opp_disks;
            x |= BitBoard::shift_one_direction(dir, x) & opp_disks;
            x |= BitBoard::shift_one_direction(dir, x) & opp_disks;

            /* Empty cells adjacent to those are valid moves. */
            legal_moves |= BitBoard::shift_one_direction(dir, x) & empty_cells;
        }

        legal_moves
    }
}
