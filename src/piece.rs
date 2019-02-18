#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Piece {
    White,
    Black,
}

/// Return a piece
///
/// # Examples
/// ```
/// use othlib::piece::Piece;
///
/// let p = Piece::White;
/// assert_eq!(p.swap(), Piece::Black);
/// assert_eq!(p.swap().swap(), Piece::White);
/// ```
impl Piece {
    pub fn swap(&self) -> Piece {
        match *self {
            Piece::White => Piece::Black,
            Piece::Black => Piece::White,
        }
    }
}
