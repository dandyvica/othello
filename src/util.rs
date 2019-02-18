//#[macro_use] extern crate unic_char_range;
//use unic_char_range::CharRange;

pub fn draw_board() -> String {
    // use an html table
    let mut s = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    s += r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 9 9" width="600" height="600">"#;

    s += &format!(
        "{}",
        r#"<rect x="1" y="1" width="9" height="9" style="fill:#009000;"/>"#
    );

    // col headers
    for (i, col) in chars!('A'..='H').iter().enumerate() {
        s += &format!(r#"<text x="{}" y="0.8" font-family="Verdana" font-size="0.5">{}</text>"#, (i as f64)+1.3, col);
    }

    // row headers
    for row in 1..=8 {
        s += &format!(r#"<text x="0.4" y="{}" font-family="Verdana" font-size="0.5">{}</text>"#, (row as f64)+0.6, row);
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
        9.0-0.025, 9.0-0.025
    );    
  

    s += "</svg>";

    s
}
