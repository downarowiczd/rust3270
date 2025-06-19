use crate::server::stream::StreamFormatError;

#[derive(Copy, Clone, Debug, Hash)]
pub enum Highlighting {
    Default = 0x00,
    Normal = 0xF0,
    Blink = 0xF1,
    Reverse = 0xF2,
    Underscore = 0xF4,
}

impl TryFrom<u8> for Highlighting {
    type Error = StreamFormatError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            0x00 => Highlighting::Default,
            0xF0 => Highlighting::Normal,
            0xF1 => Highlighting::Blink,
            0xF2 => Highlighting::Reverse,
            0xF4 => Highlighting::Underscore,
            _ => return Err(StreamFormatError::InvalidData),
        })
    }
}

impl From<Highlighting> for u8 {
    fn from(val: Highlighting) -> Self {
        val as u8
    }
}
