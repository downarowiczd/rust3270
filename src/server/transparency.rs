use crate::server::stream::StreamFormatError;

#[derive(Copy, Clone, Debug, Hash)]
pub enum Transparency {
    Default,
    Or,
    Xor,
    Opaque,
}

impl From<Transparency> for u8 {
    fn from(v: Transparency) -> u8 {
        match v {
            Transparency::Default => 0x00,
            Transparency::Or => 0xF0,
            Transparency::Xor => 0xF1,
            Transparency::Opaque => 0xF2,
        }
    }
}

impl TryFrom<u8> for Transparency {
    type Error = StreamFormatError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Transparency::Default,
            0xF0 => Transparency::Or,
            0xF1 => Transparency::Xor,
            0xF2 => Transparency::Opaque,
            _ => return Err(StreamFormatError::InvalidData)
        })
    }
}