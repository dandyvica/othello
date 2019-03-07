use std::convert::From;
use std::string::ToString;

use crate::board::coordinate::{Coordinate, BOARD_SIZE};
use crate::board::direction::{Direction, DIRECTIONS};

pub struct BitBoard {
    pub bits: u64,
}

impl BitBoard {
    /// Just a wraopper around an `u64` value
    pub fn new(bits: u64) -> BitBoard {
        BitBoard { bits: bits }
    }

    /// useful static methods
    pub fn zero() -> BitBoard {
        BitBoard::new(0)
    }
    pub fn max_value() -> BitBoard {
        BitBoard::new(u64::max_value())
    }

    /// Return i-th row of a bitboard
    ///
    /// # Examples
    /// ```
    /// use othlib::board::bitboard::BitBoard;
    ///
    /// let mut bitboard = BitBoard::max_value();
    /// assert_eq!(bitboard.row(0), [1;8]);
    ///
    /// bitboard = BitBoard::new(0b00000000_11111111_11111111_11111111_11111111_11111111_11111111_00000000);
    /// assert_eq!(bitboard.row(0), vec![0;8]);
    /// assert_eq!(bitboard.row(1), vec![1;8]);        
    /// assert_eq!(bitboard.row(2), vec![1;8]);        
    /// assert_eq!(bitboard.row(3), vec![1;8]);        
    /// assert_eq!(bitboard.row(4), vec![1;8]);        
    /// assert_eq!(bitboard.row(5), vec![1;8]);        
    /// assert_eq!(bitboard.row(6), vec![1;8]);        
    /// assert_eq!(bitboard.row(7), vec![0;8]);  
    ///
    /// ```
    pub fn row(&self, i: usize) -> Vec<usize> {
        if i > 7 {
            panic!(format!("Row number {} is wrong !", i));
        }
        let bits = Vec::<usize>::from(self);

        bits[BOARD_SIZE * i..BOARD_SIZE * (i + 1)].to_vec()
    }

    /// Return j-th column of a bitboard
    ///
    /// # Examples
    /// ```
    /// use othlib::board::bitboard::BitBoard;
    ///
    /// let mut bitboard = BitBoard::max_value();
    /// assert_eq!(bitboard.col(0), [1;8]);
    ///
    /// bitboard = BitBoard::new(0b01111111_01111111_01111111_01111111_01111111_01111111_01111111_00000000);
    /// assert_eq!(bitboard.col(0), vec![0;8]);
    /// assert_eq!(bitboard.col(1), vec![1,1,1,1,1,1,1,0]);        
    /// assert_eq!(bitboard.col(2), vec![1,1,1,1,1,1,1,0]);        
    /// assert_eq!(bitboard.col(3), vec![1,1,1,1,1,1,1,0]);        
    /// assert_eq!(bitboard.col(4), vec![1,1,1,1,1,1,1,0]);        
    /// assert_eq!(bitboard.col(5), vec![1,1,1,1,1,1,1,0]);        
    /// assert_eq!(bitboard.col(6), vec![1,1,1,1,1,1,1,0]);        
    /// assert_eq!(bitboard.col(7), vec![1,1,1,1,1,1,1,0]);  
    /// ```
    pub fn col(&self, j: usize) -> Vec<usize> {
        if j > 7 {
            panic!(format!("Col number {} is wrong !", j));
        }
        let bits = Vec::<usize>::from(self);
        let col: Vec<_> = bits
            .iter()
            .enumerate()
            .filter(|&(index, _)| (index % BOARD_SIZE) == j)
            .map(|e| *e.1)
            .collect();

        col
    }

    /// Shift towards direction
    ///
    /// # Examples
    /// ```
    /// use othlib::board::bitboard::BitBoard;
    /// use othlib::board::direction::Direction;
    ///
    /// let bitboard = BitBoard::max_value();
    ///
    /// let mut shifted = bitboard.shr(&Direction::N);
    /// for i in 0..=6 {
    ///     assert_eq!(shifted.row(i), vec![1;8]);
    /// }
    /// assert_eq!(shifted.row(7), vec![0;8]);
    ///
    /// shifted = bitboard.shr(&Direction::NE);
    /// for i in 0..=6 {
    ///     assert_eq!(shifted.row(i), [0,1,1,1,1,1,1,1]);
    /// }
    /// assert_eq!(shifted.row(7), vec![0;8]);
    /// assert_eq!(shifted.col(0), vec![0;8]);
    /// for i in 1..=7 {
    ///     assert_eq!(shifted.col(i), [1,1,1,1,1,1,1,0]);
    /// }
    ///  
    /// shifted = bitboard.shr(&Direction::E);
    /// for i in 0..=7 {
    ///     assert_eq!(shifted.row(i), [0,1,1,1,1,1,1,1]);
    /// }
    /// assert_eq!(shifted.col(0), vec![0;8]);
    /// for i in 1..=7 {
    ///     assert_eq!(shifted.col(i), vec![1;8]);
    /// }
    ///  
    /// shifted = bitboard.shr(&Direction::SE);
    /// assert_eq!(shifted.row(0), vec![0;8]);
    /// for i in 1..=7 {
    ///     assert_eq!(shifted.row(i), [0,1,1,1,1,1,1,1]);
    /// }
    /// assert_eq!(shifted.col(0), vec![0;8]);
    /// for i in 1..=7 {
    ///     assert_eq!(shifted.col(i), [0,1,1,1,1,1,1,1]);
    /// }
    ///   
    /// shifted = bitboard.shr(&Direction::S);
    /// assert_eq!(shifted.row(0), vec![0;8]);
    /// for i in 1..=7 {
    ///     assert_eq!(shifted.row(i), vec![1;8]);
    /// }
    /// for i in 0..=7 {
    ///     assert_eq!(shifted.col(i), [0,1,1,1,1,1,1,1]);
    /// }
    ///   
    /// shifted = bitboard.shr(&Direction::SW);
    /// assert_eq!(shifted.row(0), vec![0;8]);
    /// for i in 1..=7 {
    ///     assert_eq!(shifted.row(i), [1,1,1,1,1,1,1,0]);
    /// }
    /// for i in 0..=6 {
    ///     assert_eq!(shifted.col(i), [0,1,1,1,1,1,1,1]);
    /// }
    /// assert_eq!(shifted.col(7), vec![0;8]);
    ///   
    /// shifted = bitboard.shr(&Direction::W);
    /// for i in 0..=7 {
    ///     assert_eq!(shifted.row(i), [1,1,1,1,1,1,1,0]);
    /// }
    /// for i in 0..=6 {
    ///     assert_eq!(shifted.col(i), vec![1;8]);
    /// }
    /// assert_eq!(shifted.col(7), vec![0;8]);
    ///   
    /// shifted = bitboard.shr(&Direction::NW);
    /// for i in 0..=6 {
    ///     assert_eq!(shifted.row(i), [1,1,1,1,1,1,1,0]);
    /// }
    /// assert_eq!(shifted.row(7), vec![0;8]);
    /// for i in 0..=6 {
    ///     assert_eq!(shifted.col(i), [1,1,1,1,1,1,1,0]);
    /// }
    /// assert_eq!(shifted.col(7), vec![0;8]);
    ///   
    /// ```
    pub fn shr(&self, dir: &Direction) -> BitBoard {
        match dir {
            Direction::E => BitBoard::new((self.bits >> 1) & 0x7F7F7F7F7F7F7F7Fu64),
            Direction::SE => BitBoard::new((self.bits >> 9) & 0x007F7F7F7F7F7F7Fu64),
            Direction::S => BitBoard::new((self.bits >> 8) & 0xFFFFFFFFFFFFFFFFu64),
            Direction::SW => BitBoard::new((self.bits >> 7) & 0x00FEFEFEFEFEFEFEu64),
            Direction::W => BitBoard::new((self.bits << 1) & 0xFEFEFEFEFEFEFEFEu64),
            Direction::NW => BitBoard::new((self.bits << 9) & 0xFEFEFEFEFEFEFE00u64),
            Direction::N => BitBoard::new((self.bits << 8) & 0xFFFFFFFFFFFFFFFFu64),
            Direction::NE => BitBoard::new((self.bits << 7) & 0x7F7F7F7F7F7F7F00u64),
        }
    }

    /// Return the possible moves using the line cap moves algorithm
    ///
    /// # Examples
    /// ```
    /// use othlib::board::bitboard::BitBoard;
    ///
    /// let mut black = BitBoard::new(0b01000000_11011110_01000110_00101110_00011010_00011100_00000000_00000000);
    /// let mut white = BitBoard::new(0b00111110_00100000_00111000_00010000_00100000_00000000_00000000_00000000);
    ///
    /// let mut moves = white.line_cap_moves(&black);
    /// assert_eq!(moves.bits, 9223796178917988864u64);
    ///
    /// black = BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000001_00000000_00000000);
    /// white = BitBoard::new(0b11111111_11111111_11111111_11111110_11111100_11111100_11111110_11111111);
    ///
    /// moves = white.line_cap_moves(&black);
    /// assert_eq!(moves.bits, 0);
    /// 
    /// 
    /// ```
    pub fn line_cap_moves(&self, opponent_player: &BitBoard) -> BitBoard {
        let mut possible_moves = 0u64;

        // calculate empty squares
        let empty = !(self.bits | opponent_player.bits);

        // for each direction, follow opponent pieces till an empty square is found
        for dir in &DIRECTIONS {
            let mut candidates = opponent_player.bits & (self.shr(dir)).bits;

            while candidates != 0u64 {
                let shifted = (BitBoard::new(candidates).shr(dir)).bits;
                possible_moves |= empty & shifted;

                candidates = opponent_player.bits & shifted;
            }
        }

        BitBoard::new(possible_moves)
    }
}

/// Convert a bitboard value to a string of 0 or 1
///
/// # Examples
/// ```
/// use othlib::board::bitboard::BitBoard;
///
/// let bitboard = BitBoard::max_value();
/// assert_eq!(bitboard.to_string(), "1".repeat(64));
/// ```    
impl ToString for BitBoard {
    fn to_string(&self) -> String {
        let mut bits = format!("{:#066b}", self.bits);

        // 2 first chars are '0b'
        bits.remove(0);
        bits.remove(0);

        bits
    }
}

/// Convert a bitboard value to a vector of 0 or 1 int values
///
/// # Examples
/// ```
/// use std::convert::From;
/// use othlib::board::bitboard::BitBoard;
///
/// let v = Vec::<usize>::from(&BitBoard::max_value());
/// assert_eq!(v, vec![1;64]);
/// ```    
impl From<&BitBoard> for Vec<usize> {
    fn from(bits: &BitBoard) -> Self {
        // convert to 01 string
        let as_str = bits.to_string();

        let v: Vec<usize> = as_str
            .chars()
            .map(|b| b.to_digit(10).unwrap() as usize)
            .collect();

        v
    }
}

/// Convert a bitboard value to a vector of algebric coordinates
///
/// # Examples
/// ```
/// use std::convert::From;
/// use othlib::board::bitboard::BitBoard;
///
/// let v = Vec::<String>::from(&BitBoard::new(17293822569102704640u64));
/// assert_eq!(v, vec!["A1","B1", "C1", "D1"]);
///
/// let v = Vec::<String>::from(&BitBoard::new(3u64));
/// assert_eq!(v, vec!["G8", "H8"]);
///
/// let v = Vec::<String>::from(&BitBoard::new(1<<63));
/// assert_eq!(v, vec!["A1"]);
/// ```    
impl From<&BitBoard> for Vec<String> {
    fn from(bits: &BitBoard) -> Self {
        // convert to 01 string
        let as_str = bits.to_string();

        let mut v = Vec::new();

        for (index, bit) in as_str.chars().enumerate() {
            if bit == '1' {
                let coord = Coordinate::from_linear(index);
                v.push(Coordinate::to_algebric(coord));
            }
        }
        v
    }
}

/// Build a bitboard from a list a algebric coordinates
///
/// # Examples
/// ```
/// use std::convert::From;
/// use othlib::board::bitboard::BitBoard;
///
/// let bitboard = BitBoard::from(vec!["A1","B1", "C1", "D1"]);
/// assert_eq!(bitboard.bits, 17293822569102704640);
/// ```    
impl From<Vec<&str>> for BitBoard {
    fn from(alg_vec: Vec<&str>) -> Self {
        let mut bits = 0u64;

        // build for each coordinate
        for alg_coord in &alg_vec {
            let coord = Coordinate::to_linear(Coordinate::from_algebric(alg_coord));
            bits += 1 << (63 - coord);
        }

        BitBoard::new(bits)
    }
}
