#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

/// Return a color
///
/// # Examples
/// ```
/// use othlib::color::Color;
///
/// let p = Color::White;
/// assert_eq!(p.flip(), Color::Black);
/// assert_eq!(p.flip().flip(), Color::White);
/// ```
impl Color {
    pub fn flip(&self) -> Color {
        match *self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
