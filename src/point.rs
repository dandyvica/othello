static ASCII_UPPER: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

#[derive(Debug, Clone, PartialEq)]
pub struct Point {}

impl Point {
    /// Useful conversion function to map (x,y) coordonate to linear index
    ///
    /// # Examples
    /// ```
    /// use othlib::point::Point;
    ///
    /// assert_eq!(Point::to_index((0,0)), 0);
    /// assert_eq!(Point::to_index((7,7)), 63);
    ///
    /// ```      
    #[inline(always)]
    pub fn to_index(pt: (usize, usize)) -> usize {
        (pt.0 + 8 * pt.1) as usize
    }

    /// Useful conversion function from linear index to coordinates
    ///
    /// # Examples
    /// ```
    /// use othlib::point::Point;
    ///
    /// assert_eq!(Point::to_coordinate(0), (0,0));
    /// assert_eq!(Point::to_coordinate(63), (7,7));
    ///
    /// ```       
    #[inline(always)]
    pub fn to_coordinate(index: usize) -> (usize, usize) {
        (index % 8, index / 8)
    }

    /// Useful conversion function from linear index to bitboard coordinates
    ///
    /// # Examples
    /// ```
    /// use othlib::point::Point;
    ///
    /// assert_eq!(Point::to_bitboard_coordinate(63), (0,0));
    /// assert_eq!(Point::to_bitboard_coordinate(0), (7,7));
    ///
    /// ```  
    /// ```should_panic
    /// use othlib::point::Point;
    ///
    /// assert_eq!(Point::to_bitboard_coordinate(64), (0,0));
    ///
    /// ```
    #[inline(always)]
    pub fn to_bitboard_coordinate(index: usize) -> (usize, usize) {
        if index > 63 {
            panic!("Index {} can't be greater than 63 !", index);
        }
        Point::to_coordinate(63 - index)
    }

    /// Convert coordinate to algebric notation
    ///
    /// # Examples
    /// ```
    /// use othlib::point::Point;
    ///
    /// assert_eq!(Point::to_algebric((0,0)), "A1");
    /// assert_eq!(Point::to_algebric((7,7)), "H8");
    /// ```    
    #[inline(always)]
    pub fn to_algebric(pt: (usize, usize)) -> String {
        format!("{}{}", ASCII_UPPER[pt.0], pt.1 + 1)
    }

    /// Convert coordinate from algebric notation
    ///
    /// # Examples
    /// ```
    /// use othlib::point::Point;
    ///
    /// assert_eq!(Point::from_algebric("A1"), (0,0));
    /// assert_eq!(Point::from_algebric("D4"), (3,3));
    /// assert_eq!(Point::from_algebric("H8"), (7,7));
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
