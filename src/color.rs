#[derive(Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

use Color::*;

// Surround a string with ASCII control characters to color it
pub fn color_as(c: Color, s: &str) -> String {
    return format!("\x1b[{}m{}\x1b[0m", code(c), s);
}

fn code(c: Color) -> String {
    match c {
        Black => "30",
        Red => "31",
        Green => "32",
        Yellow => "33",
        Blue => "34",
        Magenta => "35",
        Cyan => "36",
        White => "37",
    }
    .to_owned()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_color() {
        // uncomment to check if the colors are printed correctly
        // use super::*;
        // println!("{:+}", color_as(Blue, "hello"));
        // println!("{:+}", color_as(Green, "hello"));
    }
}
