#[derive(Debug)]
pub struct VecInt64 {
    pub v64: Vec<i8>,
}

impl VecInt64 {
    /// New vector is initialized to [63,62,.....,2,1,0]
    ///
    /// # Examples
    /// ```
    /// use othlib::svg::vecint64::VecInt64;
    ///
    /// let v = VecInt64::new();
    /// assert!(v.v64.contains(&63));
    /// assert!(v.v64.contains(&0));
    /// assert!(!v.v64.contains(&64));
    /// ``` 
    pub fn new() -> Self {
        VecInt64 {
            v64: (0..=63).rev().collect(),
        }
    }

    /// Right shift and add 0 in the head
    ///
    /// # Examples
    /// ```
    /// use othlib::svg::vecint64::VecInt64;
    ///
    /// let mut v = VecInt64::new();
    /// v.rshift(8);
    /// assert!(v.v64.contains(&63));
    /// assert!(!v.v64.contains(&0));
    /// assert!(v.v64.contains(&-1));
    /// assert!(!v.v64.contains(&7));
    /// ```     
    pub fn rshift(&mut self, n: usize) -> &mut Self {
        for _i in 1..=n {
            self.v64.insert(0,-1);
        }

        self.v64.truncate(64);

        self
    }

    /// Left shift and add 0 to the tail
    ///
    /// # Examples
    /// ```
    /// use othlib::svg::vecint64::VecInt64;
    ///
    /// let mut v = VecInt64::new();
    /// v.lshift(8);
    /// assert!(!v.v64.contains(&63));
    /// assert!(v.v64.contains(&0));
    /// assert!(v.v64.contains(&54));
    /// ```  
    pub fn lshift(&mut self, n: usize) -> &mut Self {
        for _i in 1..=n {
            self.v64.remove(0);
            self.v64.push(-1);
        }

        self
    }

    /// Set element to 0 where bit is set to 0 in the `mask` variable
    ///
    /// # Examples
    /// ```
    /// use othlib::svg::vecint64::VecInt64;
    ///
    /// let mut v = VecInt64::new();
    /// v.mask(0x8080808080808080);
    /// assert!(v.v64.contains(&63));
    /// assert!(!v.v64.contains(&62));
    /// assert!(v.v64.contains(&55));
    /// ```  
    pub fn mask(&mut self, mask: u64) -> &mut Self {
        let bit_mask = format!("{:#066b}", mask);

        for (i, c) in bit_mask.chars().skip(2).enumerate() {
            if c == '0' {
                self.v64[i] = -1;
            }
        }

        self
    }
}