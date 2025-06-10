pub(crate) mod cp037;
pub enum Encoding {
    CP037,
}

impl Encoding {
    pub fn encode_table(&self) -> &'static [u8; 256] {
        match self {
            Encoding::CP037 => &crate::encoding::cp037::ENCODE_TBL,
            // Add more encodings here
        }
    }
    pub fn decode_table(&self) -> &'static [u8; 256] {
        match self {
            Encoding::CP037 => &crate::encoding::cp037::DECODE_TBL,
            // Add more encodings here
        }
    }
}

// Generic function to encode from ASCII to target encoding.
pub fn encode_ascii_to(
    stream: impl Iterator<Item = char>,
    encoding: &Encoding,
) -> impl Iterator<Item = u8> {
    let tbl = encoding.encode_table();
    stream.map(move |ch| {
        let idx = ch as usize;
        if idx < 256 {
            let code = tbl[idx];
            if code < 0x40 { 0x40 } else { code }
        } else {
            0x40
        }
    })
}

// Generic function to decode from target encoding to ASCII.
pub fn decode_to_ascii(
    stream: impl Iterator<Item = u8>,
    encoding: &Encoding,
) -> impl Iterator<Item = char> {
    let tbl = encoding.decode_table();
    stream.map(move |byte| tbl[byte as usize] as char)
}
