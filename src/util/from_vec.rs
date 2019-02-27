pub trait FromVec {
    fn from_vec(indexes: Vec<usize>) -> Self;
}

impl FromVec for u64 {
    /// Useful conversion function from a vec of indexes to a 64-bit integer.
    ///
    /// # Argument
    ///
    /// * `v`: vector of indexes where a bit value of 1 is needed.
    ///
    /// # Examples
    /// ```
    /// use othlib::util::from_vec::FromVec;
    ///
    /// let v: Vec<_> = (0..=63).map(|i| i).collect();
    ///
    /// assert_eq!(u64::from_vec(v), u64::max_value());
    /// assert_eq!(u64::from_vec(vec![0,1]), 3u64);
    ///
    /// ```     
    fn from_vec(indexes: Vec<usize>) -> u64 {
        // lots of '0'
        let mut v = vec!['0'; 64];

        // indexes are positions where I need '1'
        for i in &indexes {
            v[63 - *i as usize] = '1';
        }

        // export Vec<char> as a string
        let s: String = v.iter().collect();

        // convert to u64
        u64::from_str_radix(&s, 2).unwrap()
    }
}
