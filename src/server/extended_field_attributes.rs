use bitflags::bitflags;
use crate::server::{color::Color, highlighting::Highlighting, stream::StreamFormatError, transparency::Transparency, wcc::{make_ascii_translatable, FieldAttribute}};

bitflags! {
    #[derive(Debug, Clone, Hash)]
    pub struct FieldOutline: u8 {
        const NO_OUTLINE = 0;
        const UNDERLINE = 0b0001;
        const RIGHT = 0b0010;
        const OVERLINE = 0b0100;
        const LEFT = 0b1000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Hash)]
    pub struct FieldValidation: u8 {
        const MANDATORY_FILL = 0b100;
        const MANDATORY_ENTRY = 0b010;
        const TRIGGER = 0b001;
    }
}

#[derive(Clone, Debug, Hash)]
pub enum ExtendedFieldAttribute {
    AllAttributes,
    ExtendedHighlighting(Highlighting),
    ForegroundColor(Color),
    CharacterSet(u8),
    BackgroundColor(Color),
    Transparency(Transparency),
    FieldAttribute(FieldAttribute),
    FieldValidation(FieldValidation),
    FieldOutlining(FieldOutline),
}

impl TryFrom<&[u8]> for ExtendedFieldAttribute {
    type Error = StreamFormatError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(StreamFormatError::UnexpectedEOR);
        }
        Ok(match (value[0], value[1]) {
            (0x00, 0x00) => ExtendedFieldAttribute::AllAttributes,
            (0x00, _) => return Err(StreamFormatError::InvalidData),
            (0xC0, fa) => ExtendedFieldAttribute::FieldAttribute(FieldAttribute::from_bits(fa & 0x3F).ok_or(StreamFormatError::InvalidData)?),
            (0x41, v) => ExtendedFieldAttribute::ExtendedHighlighting(v.try_into()?),
            (0x45, v) => ExtendedFieldAttribute::BackgroundColor(v.try_into()?),
            (0x42, v) => ExtendedFieldAttribute::ForegroundColor(v.try_into()?),
            (0x43, v) => ExtendedFieldAttribute::CharacterSet(v),
            (0xC2, v) => ExtendedFieldAttribute::FieldOutlining(FieldOutline::from_bits(v).ok_or(StreamFormatError::InvalidData)?),
            (0x46, v) => ExtendedFieldAttribute::Transparency(v.try_into()?),
            (0xC1, v) => ExtendedFieldAttribute::FieldValidation(FieldValidation::from_bits(v).ok_or(StreamFormatError::InvalidData)?),
            _ => return Err(StreamFormatError::InvalidData),
        })
    }
}

impl ExtendedFieldAttribute {
    pub fn encoded(self) -> (u8, u8) {
        match self {
            ExtendedFieldAttribute::AllAttributes => (0x00,0x00),
            ExtendedFieldAttribute::FieldAttribute(fa) => (0xC0, make_ascii_translatable(fa.bits())),
            ExtendedFieldAttribute::ExtendedHighlighting(fa) => (0x41, fa.into()),
            ExtendedFieldAttribute::BackgroundColor(c) => (0x45, c.into()),
            ExtendedFieldAttribute::ForegroundColor(c) => (0x42, c.into()),
            ExtendedFieldAttribute::CharacterSet(cs) => (0x43, cs.into()),
            ExtendedFieldAttribute::FieldOutlining(fo) => (0xC2, fo.bits()),
            ExtendedFieldAttribute::Transparency(v) => (0x46, v.into()),
            ExtendedFieldAttribute::FieldValidation(v) => (0xC1, v.bits()),
        }
    }

    pub fn encode_into(&self, output: &mut Vec<u8>) {
        let (typ, val) = self.clone().encoded();
        output.extend_from_slice(&[typ, val]);
    }

}

impl Into<ExtendedFieldAttribute> for &ExtendedFieldAttribute {
    fn into(self) -> ExtendedFieldAttribute {
            self.clone()
        }
}