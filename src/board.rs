use std::fmt;

use crate::coordinate::Coordinate;
use crate::piece::Piece;
use crate::square::Square;

//const BOARD_SIZE: usize = 8 * 8;
pub const OTHELLO_BOARD_DIMENSION: usize = 8;

pub struct Board {
    dimension: usize,
    squares: Vec<Square>,
}

impl Board {
    /// A new empty board having `board_dimension` size
    pub fn new(board_dimension: usize) -> Board {
        // volontarly exclude odd dimensions
        if board_dimension % 2 != 0 {
            panic!("Only even board dimensions are possible!");
        }

        Board {
            dimension: board_dimension,
            squares: vec![Square::Empty; board_dimension * board_dimension],
        }
    }

    /// Initialize to the Othello start position
    pub fn init(&mut self) {
        // initialize game position to start
        let center = self.dimension / 2;

        self.squares[Coordinate::to_index(center - 1, center - 1, self.dimension)] =
            Square::Occupied(Piece::White);
        self.squares[Coordinate::to_index(center, center, self.dimension)] =
            Square::Occupied(Piece::White);
        self.squares[Coordinate::to_index(center - 1, center, self.dimension)] =
            Square::Occupied(Piece::Black);
        self.squares[Coordinate::to_index(center, center - 1, self.dimension)] =
            Square::Occupied(Piece::Black);
    }

    ///  Get the piece at (x,y)
    ///
    /// # Examples
    /// ```
    /// use othlib::piece::Piece;
    /// use othlib::square::Square;
    /// use othlib::board::Board;
    ///
    /// let mut board = Board::new(8);
    /// board.init();
    /// assert_eq!(board.get_piece(3,3).unwrap(), &Square::Occupied(Piece::White));
    /// assert_eq!(board.get_piece(3,4).unwrap(), &Square::Occupied(Piece::Black));
    /// assert_eq!(board.get_piece(0,0).unwrap(), &Square::Empty);
    ///
    /// ```
    pub fn get_piece(&self, x: usize, y: usize) -> Option<&Square> {
        self.squares.get(Coordinate::to_index(x, y, self.dimension))
    }

    ///  Set the piece at (x,y) without verifying if the move is allowed
    ///
    /// # Examples
    /// ```
    /// use othlib::piece::Piece;
    /// use othlib::square::Square;
    /// use othlib::board::Board;
    ///
    /// let mut board = Board::new(8);
    /// board.init();
    /// board.set_piece(0,0,Piece::Black);
    ///
    /// assert_eq!(board.free_squares().len(), 8*8-5);
    ///
    /// ```
    pub fn set_piece(&mut self, x: usize, y: usize, p: Piece) {
        match self
            .squares
            .get_mut(Coordinate::to_index(x, y, self.dimension))
        {
            Some(v) => *v = Square::Occupied(p),
            None => panic!("({},{}) cordinates are out of the board"),
        }
    }

    /// Get all coordinates for empty squares
    ///
    /// # Examples
    /// ```
    /// use othlib::piece::Piece;
    /// use othlib::square::Square;
    /// use othlib::board::Board;
    ///
    /// let mut board = Board::new(8);
    /// board.init();
    /// assert_eq!(board.free_squares().len(), 8*8-4);
    ///
    /// ```
    pub fn free_squares(&self) -> Vec<Coordinate> {
        let mut v = Vec::new();

        for (i, sq) in self.squares.iter().enumerate() {
            if sq == &Square::Empty {
                v.push(Coordinate::to_coordinate(i, self.dimension));
            }
        }

        v
    }

    /// Get all coordinates for occupied squares for a player
    ///
    /// # Examples
    /// ```
    /// use othlib::piece::Piece;
    /// use othlib::square::Square;
    /// use othlib::board::Board;
    ///
    /// let mut board = Board::new(8);
    /// board.init();
    /// assert_eq!(board.free_squares().len(), 8*8-4);
    ///
    /// ```
    pub fn occupied_squares(&self, p: Piece) -> Vec<Coordinate> {
        let mut v = Vec::new();

        for (i, sq) in self.squares.iter().enumerate() {
            if sq != &Square::Occupied(p) {
                v.push(Coordinate::to_coordinate(i, self.dimension));
            }
        }

        v
    }

    /// Load a board from a string
    ///
    /// # Examples
    /// ```
    /// let position = r#"--------
    /// --------
    /// --WB----
    /// --------
    /// --------
    /// --------
    /// --------
    /// --------"#;
    ///
    /// use othlib::board::Board;
    ///
    /// let board = Board::load(position);
    /// assert_eq!(board.free_squares().len(), 8*8-2);
    ///
    /// ```
    pub fn load(data: &str) -> Board {
        // get the number of lines which is the board dimension
        let dim = data.lines().count();

        println!("string={}, dim={}", data, dim);

        // create an empty board
        let mut board = Board::new(dim);

        // read input string and fill the board
        for (y, row) in data.lines().enumerate() {
            for (x, c) in row.trim().chars().enumerate() {
                match c {
                    'B' => board.set_piece(x, y, Piece::Black),
                    'W' => board.set_piece(x, y, Piece::White),
                    '-' => (),
                    _ => panic!("Not a valid piece !"),
                }
            }
        }

        board
    }

    pub fn allowed_moves(&self, player: Piece) -> Vec<Coordinate> {
        let mut v = Vec::new();

        for free_sq in &self.free_squares() {
            for adjacent_sq in free_sq.adjacent(self.dimension) {
                v.push(adjacent_sq);
            }
        }

        v
    }
}

/// Print out board using unicode chars
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("");

        for y in 0..self.dimension {
            for x in 0..self.dimension {
                s += &format!("{}", self.get_piece(x, y).unwrap());
            }
            s += &format!("\n");
        }

        write!(f, "{}", s)
    }
}
