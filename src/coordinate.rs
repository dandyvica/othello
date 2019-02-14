use std::convert::From;

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
    /// assert_eq!(Coordinate::to_index(0,0,8), 0);
    /// assert_eq!(Coordinate::to_index(7,7,8), 63);
    ///
    /// ```      
    #[inline(always)]
    pub fn to_index(x: usize, y: usize, n: usize) -> usize {
        (x + n * y) as usize
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
impl From<(usize,usize)> for Coordinate {
    fn from(point: (usize,usize)) -> Self {
       Coordinate::new(point.0, point.1)
    }
}