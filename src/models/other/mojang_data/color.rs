#[derive(Debug)]
pub enum MinecraftColor {
    Black,
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red
}

impl From<&str> for MinecraftColor {
    fn from(value: &str) -> Self {
        match value {
            "black" => MinecraftColor::Black,
            "white" => MinecraftColor::White,
            "orange" => MinecraftColor::Orange,
            "magenta" => MinecraftColor::Magenta,
            "lightblue" => MinecraftColor::LightBlue,
            "yellow" => MinecraftColor::Yellow,
            "lime" => MinecraftColor::Lime,
            "pink" => MinecraftColor::Pink,
            "gray" => MinecraftColor::Gray,
            "lightgray" => MinecraftColor::LightGray,
            "cyan" => MinecraftColor::Cyan,
            "purple" => MinecraftColor::Purple,
            "blue" => MinecraftColor::Blue,
            "brown" => MinecraftColor::Brown,
            "green" => MinecraftColor::Green,
            "red" => MinecraftColor::Red,
            _ => MinecraftColor::Black
        }
    }
}