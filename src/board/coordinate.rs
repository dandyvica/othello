// standard board size is BOARD_SIZE
pub const BOARD_SIZE: usize = 8;

static ASCII_UPPER: [char; BOARD_SIZE] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {}

impl Coordinate {
    /// Useful conversion function to map (x,y) coordonate to linear index
    ///
    /// # Examples
    /// ```
    /// use othlib::board::coordinate::Coordinate;
    ///
    /// assert_eq!(Coordinate::to_linear((0,0)), 0);
    /// assert_eq!(Coordinate::to_linear((7,7)), 63);
    ///
    /// ```      
    #[inline(always)]
    pub fn to_linear(pt: (usize, usize)) -> usize {
        (pt.0 + BOARD_SIZE * pt.1) as usize
    }

    /// Useful conversion function from linear index to coordinates
    ///
    /// # Examples
    /// ```
    /// use othlib::board::coordinate::Coordinate;
    ///
    /// assert_eq!(Coordinate::from_linear(0), (0,0));
    /// assert_eq!(Coordinate::from_linear(63), (7,7));
    ///
    /// ```       
    #[inline(always)]
    pub fn from_linear(index: usize) -> (usize, usize) {
        (index % BOARD_SIZE, index / BOARD_SIZE)
    }

    /// Useful conversion function from linear index to bitboard coordinates
    ///
    /// # Examples
    /// ```
    /// use othlib::board::coordinate::Coordinate;
    ///
    /// assert_eq!(Coordinate::to_bitboard(63), (0,0));
    /// assert_eq!(Coordinate::to_bitboard(0), (7,7));
    ///
    /// ```  
    /// ```should_panic
    /// use othlib::board::coordinate::Coordinate;
    ///
    /// assert_eq!(Coordinate::to_bitboard(64), (0,0));
    ///
    /// ```
    #[inline(always)]
    pub fn to_bitboard(index: usize) -> (usize, usize) {
        if index > 63 {
            panic!("Index {} can't be greater than 63 !", index);
        }
        Coordinate::from_linear(63 - index)
    }

    /// Convert coordinate to algebric notation
    ///
    /// # Examples
    /// ```
    /// use othlib::board::coordinate::Coordinate;
    ///
    /// assert_eq!(Coordinate::to_algebric((0,0)), "A1");
    /// assert_eq!(Coordinate::to_algebric((7,7)), "H8");
    /// ```    
    #[inline(always)]
    pub fn to_algebric(pt: (usize, usize)) -> String {
        format!("{}{}", ASCII_UPPER[pt.0], pt.1 + 1)
    }

    /// Convert coordinate from algebric notation
    ///
    /// # Examples
    /// ```
    /// use othlib::board::coordinate::Coordinate;
    ///
    /// assert_eq!(Coordinate::from_algebric("A1"), (0,0));
    /// assert_eq!(Coordinate::from_algebric("D4"), (3,3));
    /// assert_eq!(Coordinate::from_algebric("H8"), (7,7));
    /// ```    
    #[inline(always)]
    pub fn from_algebric(algebric_coord: &str) -> (usize, usize) {
        let x = ASCII_UPPER
            .iter()
            .position(|&e| e == algebric_coord.chars().next().unwrap())
            .unwrap();
        let y = algebric_coord.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;

        (x, y - 1)
    }
}
