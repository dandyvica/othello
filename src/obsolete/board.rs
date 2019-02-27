use std::fmt;

use crate::coordinate::Coordinate;
use crate::piece::Piece;
use crate::square::{Square, Status};

//const BOARD_SIZE: usize = 8 * 8;
pub const OTHELLO_BOARD_DIMENSION: usize = 8;

pub struct Board {
    pub dimension: usize,
    pub squares: Vec<Square>,
}

impl Board {
    /// A new empty board having `board_dimension` size
    ///
    /// # Examples
    /// ```
    /// use othlib::board::Board;
    ///
    /// let mut board = Board::new(8);
    /// assert_eq!(board.dimension, 8);
    /// assert_eq!(board.squares.len(), 8*8);
    /// ```    
    /// ```should_panic
    /// use othlib::board::Board;
    ///
    /// let mut board = Board::new(9);
    /// ```   
    pub fn new(board_dimension: usize) -> Board {
        // volontarly exclude odd dimensions
        if board_dimension % 2 != 0 {
            panic!("Only even board dimensions are possible!");
        }

        // total number of elements = number of squares in the board
        let n = board_dimension * board_dimension;

        // set Vec dimension now
        let mut v = Vec::with_capacity(n);

        // create squares of the board
        for i in 0..n {
            v.push(Square {
                status: Status::Empty,
                coord: Coordinate::to_coordinate(i, board_dimension),
            })
        }

        Board {
            dimension: board_dimension,
            squares: v,
        }
    }

    /// Initialize to the Othello start position
    ///
    /// # Examples
    /// ```
    /// use othlib::board::Board;
    ///
    /// let mut board = Board::new(8);
    /// board.init();
    /// assert_eq!(board.free_squares().len(), 8*8-4);
    /// ```    
    pub fn init(&mut self) {
        // initialize game position to start
        let center = self.dimension / 2;

        self.set_piece((center - 1, center - 1), Piece::White);
        self.set_piece((center, center), Piece::White);
        self.set_piece((center - 1, center), Piece::Black);
        self.set_piece((center, center - 1), Piece::Black);
    }

    ///  Get the piece at (x,y)
    ///
    /// # Examples
    /// ```
    /// use othlib::piece::Piece;
    /// use othlib::square::{Status,Square};
    /// use othlib::board::Board;
    ///
    /// let mut board = Board::new(8);
    /// board.init();
    /// assert_eq!(board.get_piece((3,3)).unwrap().status, Status::Occupied(Piece::White));
    /// assert_eq!(board.get_piece((3,4)).unwrap().status, Status::Occupied(Piece::Black));
    /// assert_eq!(board.get_piece((0,0)).unwrap().status, Status::Empty);
    ///
    /// ```
    pub fn get_piece<T: Into<Coordinate>>(&self, value: T) -> Option<&Square> {
        // convert value
        let coord = value.into();
        self.squares.get(coord.to_index(self.dimension))
    }
    pub fn at(&self, coord: &Coordinate) -> Option<&Square> {
        self.squares.get(coord.to_index(self.dimension))
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
    /// board.set_piece((0,0),Piece::Black);
    ///
    /// assert_eq!(board.free_squares().len(), 8*8-5);
    ///
    /// ```
    pub fn set_piece<T: Into<Coordinate>>(&mut self, value: T, p: Piece) {
        // convert value
        let coord: Coordinate = value.into();

        match self.squares.get_mut(coord.to_index(self.dimension)) {
            Some(v) => v.status = Status::Occupied(p),
            None => panic!("({},{}) cordinates are out of the board", coord.x, coord.y),
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
    pub fn free_squares(&self) -> Vec<&Square> {
        self.filter(|sq| sq.status == Status::Empty)
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
    pub fn occupied_squares(&self, p: Piece) -> Vec<&Square> {
        self.filter(|sq| sq.status != Status::Empty)
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
                    'B' => board.set_piece((x, y), Piece::Black),
                    'W' => board.set_piece((x, y), Piece::White),
                    '-' => (),
                    _ => panic!("Not a valid piece !"),
                }
            }
        }

        board
    }

    // pub is_move_valid(&self, coord: &Coordinate, piece: Piece) -> bool {
    //     if !self.at(coord).is_free() {
    //         panic!("{} is not free!", coord);
    //     }
    // }

    pub fn allowed_moves(&self, player: Piece) -> Vec<Coordinate> {
        let mut v = Vec::new();

        for sq in self.occupied_squares(player) {
            for coord in sq.coord.adjacent(self.dimension) {
                if self.at(&coord).unwrap().is_free() {
                    v.push(coord);
                }
            }
        }

        v
    }

    /// Count the number of discs from a colour
    ///
    /// # Examples
    /// ```
    /// use othlib::piece::Piece;
    /// use othlib::square::Square;
    /// use othlib::board::Board;
    ///
    /// let mut board = Board::new(8);
    /// board.init();
    /// assert_eq!(board.value(Piece::Black), 2);
    ///
    /// ```
    pub fn value(&self, p: Piece) -> usize {
        self.filter(|sq| sq.status == Status::Occupied(p)).len()
    }

    /// Filter squares according to predicate
    fn filter<F>(&self, pred: F) -> Vec<&Square>
    where
        F: Fn(&Square) -> bool,
    {
        let v: Vec<_> = self.squares.iter().filter(|e| pred(e)).collect();
        v
    }
}

/// Print out board using unicode chars
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("");

        for y in 0..self.dimension {
            for x in 0..self.dimension {
                s += &format!("{}", self.get_piece((x, y)).unwrap());
            }
            s += &format!("\n");
        }

        write!(f, "{}", s)
    }
}
