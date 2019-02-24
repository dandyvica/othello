use std::convert::From;

static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    /// Simple constructor
    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x: x, y: y }
    }

    /// Useful conversion function to map (x,y) coordonate to linear index
    ///
    /// # Examples
    /// ```
    /// use othlib::coordinate::Coordinate;
    ///
    /// let origin = Coordinate::new(0,0);
    /// let last = Coordinate::new(7,7);
    ///
    /// assert_eq!(origin.to_index(8), 0);
    /// assert_eq!(last.to_index(8), 63);
    ///
    /// ```      
    //#[inline(always)]
    pub fn to_index(&self, n: usize) -> usize {
        (self.x + n * self.y) as usize
    }

    /// Useful conversion function from linear index to coordinates
    ///
    /// # Examples
    /// ```
    /// use othlib::coordinate::Coordinate;
    ///
    /// assert_eq!(Coordinate::to_coordinate(0, 8), Coordinate{x: 0, y: 0});
    /// assert_eq!(Coordinate::to_coordinate(35, 8), Coordinate{x: 3, y: 4});
    /// assert_eq!(Coordinate::to_coordinate(28, 8), Coordinate{x: 4, y: 3});
    /// assert_eq!(Coordinate::to_coordinate(63, 8), Coordinate{x: 7, y: 7});
    ///
    /// ```       
    #[inline(always)]
    pub fn to_coordinate(index: usize, n: usize) -> Coordinate {
        Coordinate {
            x: index % n,
            y: index / n,
        }
    }

    #[inline(always)]
    pub fn to_bitboard_coordinate(index: usize) -> (usize, usize) {
        ((63 - index) % 8, (63 - index) / 8)
    }

    /// Convert coordinate to algebric notation
    ///
    /// # Examples
    /// ```
    /// use othlib::coordinate::Coordinate;
    ///
    /// let mut coord = Coordinate::new(0, 0);
    /// assert_eq!(coord.to_algebric(), "A1");
    ///
    /// coord = Coordinate::new(25, 25);
    /// assert_eq!(coord.to_algebric(), "Z26");
    /// ```       
    pub fn to_algebric(&self) -> String {
        format!("{}{}", ASCII_UPPER[self.x], self.y + 1)
    }

    /// List coordinates
    pub fn adjacent(&self, n: usize) -> Vec<Coordinate> {
        let mut v = Vec::new();

        match (self.x, self.y) {
            // top left square
            (0, 0) => {
                v.push(Coordinate::new(0, 1));
                v.push(Coordinate::new(1, 1));
                v.push(Coordinate::new(1, 0));
            }
            // bottom left square
            (0, m) if m == n => {
                v.push(Coordinate::new(0, n - 2));
                v.push(Coordinate::new(1, n - 2));
                v.push(Coordinate::new(1, n - 1));
            }
            // top right square
            (m, 0) if m == n => {
                v.push(Coordinate::new(n - 2, 0));
                v.push(Coordinate::new(n - 2, 1));
                v.push(Coordinate::new(n - 1, 1));
            }
            // bottom right square
            (p, q) if p == n && q == n => {
                v.push(Coordinate::new(n - 2, n - 1));
                v.push(Coordinate::new(n - 1, n - 2));
                v.push(Coordinate::new(n - 2, n - 2));
            }
            // all others: no need to check to overflow
            (_, _) => {
                v.push(Coordinate::new(n - 1, n - 1));
                v.push(Coordinate::new(n - 1, n));
                v.push(Coordinate::new(n - 1, n + 1));

                v.push(Coordinate::new(n, n - 1));
                v.push(Coordinate::new(n, n + 1));

                v.push(Coordinate::new(n + 1, n - 1));
                v.push(Coordinate::new(n + 1, n));
                v.push(Coordinate::new(n + 1, n + 1));
            }
        }

        v
    }
}

/// Useful conversion from a tuple
impl From<(usize, usize)> for Coordinate {
    fn from(point: (usize, usize)) -> Self {
        Coordinate::new(point.0, point.1)
    }
}
// impl From<&(usize, usize)> for Coordinate {
//     fn from(point: &(usize, usize)) -> Self {
//         Coordinate::new(point.0, point.1)
//     }
// }

/// Useful conversion from algebric coordinates (e.g.: "E5")
///
/// # Examples
/// ```
/// use othlib::coordinate::Coordinate;
///
/// let mut c1 = Coordinate::from("A1");
/// assert_eq!(c1.x, 0);
/// assert_eq!(c1.y, 0);
///
/// c1 = Coordinate::from("H8");
/// assert_eq!(c1.x, 7);
/// assert_eq!(c1.y, 7);
///
/// ```
impl From<&str> for Coordinate {
    fn from(alg: &str) -> Self {
        // sanity checks
        if alg.len() != 2 {
            panic!("{} is not a valid algebric coordinate", alg);
        }

        // convert str to Vec
        let char_vec: Vec<char> = alg.to_lowercase().chars().collect();

        if !char_vec[0].is_ascii_alphabetic() || !char_vec[1].is_ascii_digit() {
            panic!("{} is not a valid algebric coordinate", alg);
        }

        // convert coordinates
        let x = char_vec[0] as usize - 'a' as usize;
        let y = char_vec[1].to_digit(10).unwrap() as usize - 1;

        Coordinate::new(x, y)
    }
}
