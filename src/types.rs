pub enum Colours {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Colours {
    pub fn as_str(&self) -> &'static str {
        match self {
            Colours::Black => "30",
            Colours::Red => "31",
            Colours::Green => "32",
            Colours::Yellow => "33",
            Colours::Blue => "34",
            Colours::Magenta => "35",
            Colours::Cyan => "36",
            Colours::White => "37",
        }
    }
}