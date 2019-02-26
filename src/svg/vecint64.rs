#[derive(Debug)]
pub struct VecInt64 {
    v64: Vec<usize>,
}

impl VecInt64 {
    // new vector is initialized to [63,62,.....,2,1,0]
    pub fn new() -> Self {
        VecInt64 {
            v64: (0..=63).rev().collect(),
        }
    }

    // right shift
    pub fn rshift(&mut self, n: usize) -> &mut Self {
        for _i in 1..=n {
            self.v64.insert(0,0);
        }

        self.v64.truncate(64);

        self
    }

    // left shift
    pub fn lshift(&mut self, n: usize) -> &mut Self {
        for _i in 1..=n {
            self.v64.pop();
            self.v64.push(0);
        }

        self
    }

    // mask
    pub fn mask(&mut self, mask: u64) -> &mut Self {
        let bit_mask = format!("{:#066b}", mask);

        for (i, c) in bit_mask.chars().skip(2).enumerate() {
            if c == '0' {
                self.v64[i] = 0;
            }
        }

        self
    }
}