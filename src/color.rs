/// Represents a color in Minecraft.
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    /// yep uh thats white
    White = 0,
    /// orange, thats like a fruit right?
    Orange = 1,
    /// magenta is a beautiful color
    Magenta = 2,
    /// water!!
    LightBlue = 3,
    /// piss
    Yellow = 4,
    /// green but slightly more white
    Lime = 5,
    /// pinkie pie
    Pink = 6,
    /// gray
    Gray = 7,
    /// gray but not gray
    LightGray = 8,
    /// still cyan
    Cyan = 9,
    /// yep, thats another color
    Purple = 10,
    /// blue?
    Blue = 11,
    /// i hope you like brown
    Brown = 12,
    /// actual green
    Green = 13,
    /// red like my nightmares
    Red = 14,
    /// me when i close my eyes
    Black = 15,
}

impl From<u8> for Color {
    fn from(id: u8) -> Self {
        Self::from_id(id).unwrap()
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

impl Color {
    /// Converts a color ID to a `Color`.
    #[allow(dead_code)]
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(Self::White),
            1 => Some(Self::Orange),
            2 => Some(Self::Magenta),
            3 => Some(Self::LightBlue),
            4 => Some(Self::Yellow),
            5 => Some(Self::Lime),
            6 => Some(Self::Pink),
            7 => Some(Self::Gray),
            8 => Some(Self::LightGray),
            9 => Some(Self::Cyan),
            10 => Some(Self::Purple),
            11 => Some(Self::Blue),
            12 => Some(Self::Brown),
            13 => Some(Self::Green),
            14 => Some(Self::Red),
            15 => Some(Self::Black),
            _ => None,
        }
    }

    /// Converts a string to a `Color`.
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        let s = s.as_str();

        match s {
            "white" => Some(Self::White),
            "orange" => Some(Self::Orange),
            "magenta" => Some(Self::Magenta),
            "light_blue" => Some(Self::LightBlue),
            "yellow" => Some(Self::Yellow),
            "lime" => Some(Self::Lime),
            "pink" => Some(Self::Pink),
            "gray" => Some(Self::Gray),
            "light_gray" => Some(Self::LightGray),
            "cyan" => Some(Self::Cyan),
            "purple" => Some(Self::Purple),
            "blue" => Some(Self::Blue),
            "brown" => Some(Self::Brown),
            "green" => Some(Self::Green),
            "red" => Some(Self::Red),
            "black" => Some(Self::Black),
            _ => None,
        }
    }

    /// Converts a `Color` to a string.
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::White => "white",
            Self::Orange => "orange",
            Self::Magenta => "magenta",
            Self::LightBlue => "light_blue",
            Self::Yellow => "yellow",
            Self::Lime => "lime",
            Self::Pink => "pink",
            Self::Gray => "gray",
            Self::LightGray => "light_gray",
            Self::Cyan => "cyan",
            Self::Purple => "purple",
            Self::Blue => "blue",
            Self::Brown => "brown",
            Self::Green => "green",
            Self::Red => "red",
            Self::Black => "black",
        }
    }

    /// Converts a `Color` to its hex value.
    #[allow(dead_code)]
    pub fn to_hex(&self) -> &'static str {
        match self {
            Self::White => "#FFFFFF",
            Self::Orange => "#D87F33",
            Self::Magenta => "#B24CD8",
            Self::LightBlue => "#6699D8",
            Self::Yellow => "#E5E533",
            Self::Lime => "#7FCC19",
            Self::Pink => "#F27FA5",
            Self::Gray => "#4C4C4C",
            Self::LightGray => "#999999",
            Self::Cyan => "#4C7F99",
            Self::Purple => "#7F3FB2",
            Self::Blue => "#334CB2",
            Self::Brown => "#664C33",
            Self::Green => "#667F33",
            Self::Red => "#993333",
            Self::Black => "#191919",
        }
    }
}

/// Represents an RGB color.
#[derive(Debug, Clone, PartialEq)]
pub struct RGB(i32);

impl From<RGB> for i32 {
    fn from(rgb: RGB) -> Self {
        rgb.0
    }
}

impl RGB {
    /// Creates a new `RGB` color.
    pub fn new(i: i32) -> Self {
        RGB(i)
    }

    /// Converts an `RGB` color to a hex string.
    pub fn to_hex(&self) -> String {
        let r = (self.0 >> 16) & 0xFF;
        let g = (self.0 >> 8) & 0xFF;
        let b = self.0 & 0xFF;

        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Converts each rgb value to an i32
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        // Formula: Red<<16 + Green<<8 + Blue
        // see: https://minecraft.wiki/w/Data_component_format#dyed_color
        RGB((r as i32) << 16 | (g as i32) << 8 | b as i32)
    }
}
