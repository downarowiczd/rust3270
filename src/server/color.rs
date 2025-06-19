use crate::server::stream::StreamFormatError;

#[derive(Copy, Clone, Debug, Hash)]
pub enum Color {
    Default,
    NeutralBG,
    Blue,
    Red,
    Pink,
    Green,
    Turquoise,
    Yellow,
    NeutralFG,
    Black,
    DeepBlue,
    Orange,
    Purple,
    PaleGreen,
    PaleTurquoise,
    Grey,
    White,
}

impl From<Color> for u8 {
    fn from(val: Color) -> Self {
        match val {
            Color::Default => 0x00,
            Color::NeutralBG => 0xF0,
            Color::Blue => 0xF1,
            Color::Red => 0xF2,
            Color::Pink => 0xF3,
            Color::Green => 0xF4,
            Color::Turquoise => 0xF5,
            Color::Yellow => 0xF6,
            Color::NeutralFG => 0xF7,
            Color::Black => 0xF8,
            Color::DeepBlue => 0xF9,
            Color::Orange => 0xFA,
            Color::Purple => 0xFB,
            Color::PaleGreen => 0xFC,
            Color::PaleTurquoise => 0xFD,
            Color::Grey => 0xFE,
            Color::White => 0xFF,
        }
    }
}

impl TryFrom<u8> for Color {
    type Error = StreamFormatError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Color::Default,
            0xF0 => Color::NeutralBG,
            0xF1 => Color::Blue,
            0xF2 => Color::Red,
            0xF3 => Color::Pink,
            0xF4 => Color::Green,
            0xF5 => Color::Turquoise,
            0xF6 => Color::Yellow,
            0xF7 => Color::NeutralFG,
            0xF8 => Color::Black,
            0xF9 => Color::DeepBlue,
            0xFA => Color::Orange,
            0xFB => Color::Purple,
            0xFC => Color::PaleGreen,
            0xFD => Color::PaleTurquoise,
            0xFE => Color::Grey,
            0xFF => Color::White,
            _ => return Err(StreamFormatError::InvalidData),
        })
    }
}
