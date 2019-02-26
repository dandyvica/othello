// A very simple struct with static methods to create simple SVG tags used in Othello board

pub struct SVGDoc {}

impl SVGDoc {
    pub fn new_document(view_box: &str, width: f32, height: f32) -> String {
        format!(r#"<?xml version="1.0" encoding="UTF-8"?><svg xmlns="http://www.w3.org/2000/svg" viewBox="{}" width="{}" height="{}">"#, view_box, width, height)
    }

    pub fn circle(center: (f32, f32), radius: f32, style: &str) -> String {
        format!(
            r#"<circle cx="{}" cy="{}" r="{}" style="{}" />"#,
            center.0, center.1, radius, style
        )
    }

    pub fn rect(x: (f32, f32), width: f32, height: f32, style: &str) -> String {
        format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" style="{}"/>"#,
            x.0, x.1, width, height, style
        )
    }

    pub fn text(x: (f32, f32), style: &str, text: &str) -> String {
        format!(
            r#"<text x="{}" y="{}" style="{}">{}</text>"#,
            x.0, x.1, style, text
        )
    }

    pub fn text_with_anchor(x: (f32, f32), anchor: &str, style: &str, text: &str) -> String {
        format!(
            r#"<text x="{}" y="{}" text-anchor="{}" style="{}">{}</text>"#,
            x.0, x.1, anchor, style, text
        )
    }

    pub fn line(x: (f32, f32), y: (f32, f32), style: &str) -> String {
        format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" style="{}" />"#,
            x.0, x.1, y.0, y.1, style
        )
    }

    pub fn end_document() -> String {
        String::from("</svg>")
    }
}
