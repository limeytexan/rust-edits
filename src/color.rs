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

pub trait ColorAs {
    fn color_as(self, c: Color) -> String;
}

impl ColorAs for String {
    // Surround a string with ASCII control characters to color it
    fn color_as(self, c: Color) -> String {
        format!("\x1b[{}m{}\x1b[0m", code(c), self)
    }
}

impl ColorAs for char {
    // Surround a string with ASCII control characters to color it
    fn color_as(self, c: Color) -> String {
        self.to_string().color_as(c)
    }
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
        // println!("{:+}", "hello".to_string().color_as(Blue));
        // println!("{:+}", "hello".to_string().color_as(Green));
    }
}
