use crate::color::Color;
use crate::coordinate::Coordinate;
use crate::point::Point;
use crate::svg::SVG;

pub struct SVGBoard {
    svg_tags: String,
}

impl SVGBoard {
    pub fn new() -> SVGBoard {
        let mut s = SVG::new_document("0 0 10 10", 600f32, 600f32);

        // draw board background
        s += &SVG::rect((1f32, 1f32), 8f32, 8f32, "fill:#009000;");

        // row headers
        for row in 1..=8 {
            s += &SVG::text(
                (0.4f32, (row as f32) + 0.6f32),
                "font-family:Verdana;font-size:0.5px",
                &row.to_string(),
            );
        }

        // col headers
        for (i, col) in chars!('A'..='H').iter().enumerate() {
            s += &SVG::text(
                ((i as f32) + 1.3f32, 0.8f32),
                "font-family:Verdana;font-size:0.5px",
                &col.to_string(),
            );
        }

        // horizontal grid: first rule is different to match perfectly
        s += &SVG::line(
            (1f32, 1.025f32),
            (9f32, 1.025f32),
            "stroke:black;stroke-width:0.05",
        );
        for row in 2..=9 {
            s += &SVG::line(
                (1f32, row as f32),
                (9f32, row as f32),
                "stroke:black;stroke-width:0.05",
            );
        }

        // vertical grid: last rule is different
        for col in 1..=8 {
            s += &SVG::line(
                (col as f32, 1f32),
                (col as f32, 9f32),
                "stroke:black;stroke-width:0.05",
            );
        }
        s += &SVG::line(
            (8.975f32, 1f32),
            (8.975f32, 9f32),
            "stroke:black;stroke-width:0.05",
        );

        SVGBoard { svg_tags: s }
    }

    /// Add SVG end tag </svg>
    pub fn close(&mut self) {
        self.svg_tags += &SVG::end_document();
    }

    /// Just return the list of tags as a string
    pub fn get_tags(&self) -> &str {
        &self.svg_tags
    }

    /// Write SVG tags into a file
    pub fn write(&self, file_name: &str) {
        use std::fs;
        fs::write(file_name, &self.svg_tags).expect("Unable to write SVG file");
    }

    /// Write a legend below the SVG board
    pub fn draw_legend(&mut self, legend: &str) {
        self.svg_tags += &SVG::text_with_anchor(
            (5f32,9.6f32),
            "middle",
            "font-family:Verdana;font-size:0.5px;fill:black",
            legend,
        );
    }

    /// Add bit numbers in the Othello squares. 63 is the most significant bit and therefore is at (0,0) or A1
    pub fn draw_bit_indexes(&mut self) {
        for bit in (0..=63).rev() {
            let coord = Coordinate::to_bitboard_coordinate(bit);

            self.svg_tags += &SVG::text_with_anchor(
                ((coord.0 as f32) + 1.5f32, (coord.1 as f32) + 1.6f32),
                "middle",
                "font-family:Verdana;font-size:0.3px;fill:red",
                &bit.to_string(),
            );
        }
    }

    /// Draw black or white piece using a bitboard (u64 int)
    pub fn draw_pieces_from_u64(&mut self, pieces: u64, color: Color) {
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

                let style = format!("fill={}", svg_color);

                self.svg_tags += &SVG::circle(
                    ((coord.0 as f32) + 1.5f32, (coord.1 as f32) + 1.5f32),
                    0.4f32,
                    &style,
                )
            }
        }
    }

    /// Draw black or white piece using a (x,y) coordinates    
    pub fn draw_piece_from_xy(&mut self, xy_coord: (usize, usize), color: Color) {
        // draw circle with valid color
        self.svg_tags += &SVG::circle(
            ((xy_coord.0 as f32) + 1.5f32, (xy_coord.1 as f32) + 1.5f32),
            0.4f32,
            &format!(
                "fill:{}",
                match color {
                    Color::Black => "black",
                    Color::White => "white",
                }
            ),
        );
    }

    /// Draw black or white piece using algebric coordinates
    pub fn draw_piece_from_algebric(&mut self, algebric_coord: &str, color: Color) {
        let xy_coord = Point::from_algebric(algebric_coord);
        self.draw_piece_from_xy(xy_coord, color);
    }

    /// Draw black or white pieces using a vector algebric coordinates
    pub fn draw_pieces_from_algebric(&mut self, algebric_coord_vec: Vec<&str>, color: Color) {
        for coord in &algebric_coord_vec {
            self.draw_piece_from_algebric(coord, color.clone());
        }
    }
}

// /// Create a SVG board for a single u64 value
// impl From<(u64, Color)> for SVGBoard {
//     fn from(args: (u64, Color)) -> Self {
//         // create new board
//         let mut svg = SVGBoard::new();

//         // get color
//         let svg_color = match args.1 {
//             Color::Black => "black",
//             Color::White => "white",
//         };

//         // convert binary digit to string
//         let bits = format!("{:#066b}", args.0);

//         // loop through bits and draw a circle whose color is the one of the `color` argument
//         for (i, c) in bits.chars().skip(2).enumerate() {
//             // if bit is '1', draw a circle
//             if c == '1' {
//                 // calculate coordinates
//                 let coord = Coordinate::to_bitboard_coordinate(63 - i);

//                 // draw circle with valid color
//                 svg.svg_tags += &format!(
//                     r#"<circle cx="{}" cy="{}" r="0.4" fill="{}" />"#,
//                     (coord.0 as f64) + 1.5,
//                     (coord.1 as f64) + 1.5,
//                     svg_color
//                 );
//             }
//         }

//         svg
//     }
// }
