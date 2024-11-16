pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Point2D {
    pub x: f64,
    pub y: f64,
    pub carte: char,
}
pub fn build_color(red: u8, blue: u8) -> Color {
    Color {
        red,
        green: 0,
        blue,
    }
}

pub fn adding_color(color1: &Color, color2: &Color) -> u8 {
    color1.red + color2.red
}
