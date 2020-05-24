#[derive(Debug)]
#[allow(dead_code)]
enum Color{
    Core { red: u8, green: u8, blue: u8 },
    Rgb(u8, u8, u8),
    Hex(String),
    Black,
    White
}

impl Color {
    fn is_black(&self) -> bool {
        match self {
            Color::Core{red, green, blue} => {
                (*red as u16) + (*green as u16) + (*blue as u16) == 0
            },
            Color::Rgb(r, g, b) => {
                (*r as u16) + (*g as u16) + (*b as u16) == 0
            },
            Color::Hex(s) => s == "#000" || s == "#000000",
            Color::Black => true,
            Color::White => false,
        }
    }
}

fn main() {
    let colors: [Color; 5] = [
        Color::Rgb(255, 255, 255),
        Color::Core{ red: 0, green: 0, blue: 0},
        Color::Hex(String::from("#ff0")),
        Color::Black,
        Color::White,
    ];

    // println!("{:?}", color);
    
    for color in colors.iter() {
        println!("{:?} is black ? {}", color, color.is_black());
    }
}
