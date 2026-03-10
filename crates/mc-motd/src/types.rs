use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MotdNode {
    pub text: String,
    pub style: MotdStyle,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MotdStyle {
    pub color: Option<McColor>,
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub strikethrough: bool,
    pub obfuscated: bool,
}

#[derive(Debug, Clone, Serialize)]
pub enum McColor {
    Named(String),
    Hex(String),
}

impl McColor {
    pub fn to_css(&self) -> String {
        match self {
            McColor::Named(name) => named_color_to_hex(name).to_string(),
            McColor::Hex(hex) => hex.clone(),
        }
    }
}

pub fn named_color_to_hex(name: &str) -> &str {
    match name {
        "black" => "#000000",
        "dark_blue" => "#0000AA",
        "dark_green" => "#00AA00",
        "dark_aqua" => "#00AAAA",
        "dark_red" => "#AA0000",
        "dark_purple" => "#AA00AA",
        "gold" => "#FFAA00",
        "gray" => "#AAAAAA",
        "dark_gray" => "#555555",
        "blue" => "#5555FF",
        "green" => "#55FF55",
        "aqua" => "#55FFFF",
        "red" => "#FF5555",
        "light_purple" => "#FF55FF",
        "yellow" => "#FFFF55",
        "white" => "#FFFFFF",
        _ => "#FFFFFF",
    }
}

/// Map legacy formatting code character to a color name.
pub fn code_to_color(code: char) -> Option<&'static str> {
    match code {
        '0' => Some("black"),
        '1' => Some("dark_blue"),
        '2' => Some("dark_green"),
        '3' => Some("dark_aqua"),
        '4' => Some("dark_red"),
        '5' => Some("dark_purple"),
        '6' => Some("gold"),
        '7' => Some("gray"),
        '8' => Some("dark_gray"),
        '9' => Some("blue"),
        'a' | 'A' => Some("green"),
        'b' | 'B' => Some("aqua"),
        'c' | 'C' => Some("red"),
        'd' | 'D' => Some("light_purple"),
        'e' | 'E' => Some("yellow"),
        'f' | 'F' => Some("white"),
        _ => None,
    }
}
