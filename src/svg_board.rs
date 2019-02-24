use std::convert::From;

use crate::color::Color;
use crate::coordinate::Coordinate;

const SVG_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 9 9" width="600" height="600">"#;
const SVG_FOOTER: &str = "</svg>";

pub struct SVGBoard {
    svg_tags: String,
}

impl SVGBoard {
    pub fn new() -> SVGBoard {
        let mut s = String::from(SVG_HEADER);

        // draw board background
        s += &format!(
            "{}",
            r#"<rect x="1" y="1" width="9" height="9" style="fill:#009000;"/>"#
        );

        // row headers
        for row in 1..=8 {
            s += &format!(
                r#"<text x="0.4" y="{}" font-family="Verdana" font-size="0.5">{}</text>"#,
                (row as f64) + 0.6,
                row
            );
        }

        // col headers
        for (i, col) in chars!('A'..='H').iter().enumerate() {
            s += &format!(
                r#"<text x="{}" y="0.8" font-family="Verdana" font-size="0.5">{}</text>"#,
                (i as f64) + 1.3,
                col
            );
        }

        // horizontal grid
        s += &format!(
            r#"<line x1="1" y1="{}" x2="9" y2="{}" style="stroke:black;stroke-width:0.05" />"#,
            1.025, 1.025
        );
        for row in 2..=9 {
            s += &format!(
                r#"<line x1="1" y1="{}" x2="9" y2="{}" style="stroke:black;stroke-width:0.05" />"#,
                row, row
            );
        }

        // horizontal grid
        s += &format!(
            r#"<line x1="1" y1="{}" x2="9" y2="{}" style="stroke:black;stroke-width:0.05" />"#,
            1.025, 1.025
        );
        for row in 2..=9 {
            s += &format!(
                r#"<line x1="1" y1="{}" x2="9" y2="{}" style="stroke:black;stroke-width:0.05" />"#,
                row, row
            );
        }

        // vertical grid
        for col in 1..=8 {
            s += &format!(
                r#"<line x1="{}" y1="1" x2="{}" y2="9" style="stroke:black;stroke-width:0.05" />"#,
                col, col
            );
        }
        s += &format!(
            r#"<line x1="{}" y1="1" x2="{}" y2="9" style="stroke:black;stroke-width:0.05" />"#,
            9.0 - 0.025,
            9.0 - 0.025
        );

        SVGBoard { svg_tags: s }
    }

    /// Add SVG end tag </svg>
    pub fn close(&mut self) {
        self.svg_tags += SVG_FOOTER;
    }

    /// Just return the list of tags as a string
    pub fn get_tags(&self) -> &str {
        &self.svg_tags
    }

    /// Add bit numbers in the Othello squares. 63 is the most significant bit and therefore is at (0,0) or A1
    pub fn draw_bit_indexes(&mut self) {
        for bit in (0..=63).rev() {
            let coord = Coordinate::to_bitboard_coordinate(bit);

            self.svg_tags += &format!(
                r#"<text x="{}" y="{}" text-anchor="middle" font-family="Verdana" font-size="0.3" fill="red">{}</text>"#,
                (coord.0 as f64) + 1.5,
                (coord.1 as f64) + 1.6,
                bit
            );
        }
    }

    /// Draw black or white piece using a bitboard (u64 int)
    pub fn draw_pieces_u64(&mut self, pieces: u64, color: Color) {
        // get color
        let svg_color = match color {
            Color::Black => "black",
            Color::White => "white",
        };

        // convert binary digit to string
        let bits = format!("{:#066b}", pieces);

        // loop through bits and draw a circle whose color is the one of the `color` argument
        for (i, c) in bits.chars().skip(2).enumerate() {
            // if bit is '1', draw a circle
            if c == '1' {
                // calculate coordinates
                let coord = Coordinate::to_bitboard_coordinate(63 - i);

                // draw circle with valid color
                self.svg_tags += &format!(
                    r#"<circle cx="{}" cy="{}" r="0.4" fill="{}" />"#,
                    (coord.0 as f64) + 1.5,
                    (coord.1 as f64) + 1.5,
                    svg_color
                );
            }
        }
    }

    /// Draw black or white piece using a (x,y) coordinates    
    pub fn draw_pieces_xy(&mut self, coord: (usize, usize), color: Color) {
        // draw circle with valid color
        self.svg_tags += &format!(
            r#"<circle cx="{}" cy="{}" r="0.4" fill="{}" />"#,
            (coord.0 as f64) + 1.5,
            (coord.1 as f64) + 1.5,
            match color {
                Color::Black => "black",
                Color::White => "white",
            }
        );
    }
}

/// Create a SVG board for a single u64 value
impl From<(u64, Color)> for SVGBoard {
    fn from(args: (u64, Color)) -> Self {
        // create new board
        let mut svg = SVGBoard::new();

        // get color
        let svg_color = match args.1 {
            Color::Black => "black",
            Color::White => "white",
        };

        // convert binary digit to string
        let bits = format!("{:#066b}", args.0);

        // loop through bits and draw a circle whose color is the one of the `color` argument
        for (i, c) in bits.chars().skip(2).enumerate() {
            // if bit is '1', draw a circle
            if c == '1' {
                // calculate coordinates
                let coord = Coordinate::to_bitboard_coordinate(63 - i);

                // draw circle with valid color
                svg.svg_tags += &format!(
                    r#"<circle cx="{}" cy="{}" r="0.4" fill="{}" />"#,
                    (coord.0 as f64) + 1.5,
                    (coord.1 as f64) + 1.5,
                    svg_color
                );
            }
        }

        svg
    }
}
