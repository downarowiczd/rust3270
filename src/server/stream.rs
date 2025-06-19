use std::io::Write;
use std::convert::{TryFrom};
use snafu::{Snafu, ensure};

use crate::encoding::Encoding;
use crate::server::aid::AID;
use crate::server::extended_field_attributes::ExtendedFieldAttribute;
use crate::server::wcc::{FieldAttribute, WCC};

#[derive(Clone, Debug, Snafu, Eq, PartialEq)]
pub enum StreamFormatError {
    #[snafu(display("Invalid AID: {:02x}", aid))]
    InvalidAID { aid: u8, },
    #[snafu(display("Record ended early"))]
    UnexpectedEOR,
    #[snafu(display("Invalid data"))]
    InvalidData,
}


pub trait OutputRecord {
    type Response;

    fn write_to(&self, writer: &mut dyn Write) -> std::io::Result<()>;
}

#[derive(Debug, Clone)]
pub struct WriteCommand {
    pub command: WriteCommandCode,
    pub wcc: WCC,
    pub orders: Vec<WriteOrder>,
}

#[derive(Copy, Clone, Debug)]
pub enum WriteCommandCode {
    Write,
    EraseWrite,
    EraseWriteAlternate,
    EraseAllUnprotected,
    WriteStructuredField,
}

impl WriteCommandCode {
    pub fn to_command_code(self) -> u8 {
        match self {
            WriteCommandCode::Write => 0xF1,
            WriteCommandCode::EraseWrite => 0xF5,
            WriteCommandCode::EraseWriteAlternate => 0x7E,
            WriteCommandCode::EraseAllUnprotected => 0x6F,
            WriteCommandCode::WriteStructuredField => 0xF3,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BufferAddressCalculator {
    pub width: u16,
    pub height: u16,
}

impl BufferAddressCalculator {
    pub fn encode_address(self, y: u16, x: u16) -> u16 {
        self.width * (y - 1) + (x - 1)
    }

    pub fn last_address(self) -> u16 {
        self.width * self.height - 1
    }

    pub fn decode_address(self, addr: u16) -> (u16, u16) {
        (addr / self.width, addr % self.width)
    }
}

#[derive(Clone, Debug)]
pub enum WriteOrder {
    StartField(FieldAttribute),
    StartFieldExtended(Vec<ExtendedFieldAttribute>),
    SetBufferAddress(u16),
    SetAttribute(ExtendedFieldAttribute),
    ModifyField(Vec<ExtendedFieldAttribute>),
    InsertCursor(u16),
    ProgramTab,
    RepeatToAddress(u16, char),
    EraseUnprotectedToAddress(u16),
    GraphicEscape(u8),
    SendText(String),
}

impl WriteOrder {

    pub fn serialize(&self, output: &mut Vec<u8>) {
        match self {
            WriteOrder::StartField(attr) => output.extend_from_slice(&[0x1D, attr.bits()]),
            WriteOrder::StartFieldExtended(attrs) => {
                output.extend_from_slice(&[0x29, attrs.len() as u8]);
                for attr in attrs {
                    attr.encode_into(&mut *output);
                }
            }
            WriteOrder::SetBufferAddress(addr) => output.extend_from_slice(&[0x11, (addr >> 8) as u8, (addr & 0xff) as u8]),
            WriteOrder::SetAttribute(attr) => {
                let (typ, val) = attr.clone().encoded();
                output.extend_from_slice(&[0x28, typ, val]);
            }
            WriteOrder::ModifyField(attrs) => {
                output.extend_from_slice(&[0x2C, attrs.len() as u8]);
                for attr in attrs {
                    attr.encode_into(&mut* output);
                }
            }
            WriteOrder::InsertCursor(addr) => output.extend_from_slice(&[0x11, (addr >> 8) as u8, (addr & 0xff) as u8]),
            WriteOrder::ProgramTab => output.push(0x05),
            WriteOrder::RepeatToAddress(addr, ch) => {
                output.extend_from_slice(&[0x3C, (addr >> 8) as u8, (addr & 0xff) as u8, crate::encoding::cp037::ENCODE_TBL[*ch as usize]])
            }
            WriteOrder::EraseUnprotectedToAddress(addr) => {
                output.extend_from_slice(&[0x12, (addr >> 8) as u8, (addr & 0xff) as u8])
            }
            WriteOrder::GraphicEscape(ch) => output.extend_from_slice(&[0x08, *ch]),
            WriteOrder::SendText(text) => {
                output.extend(crate::encoding::encode_ascii_to(text.chars(), &Encoding::CP037));
            }
        }
    }
}

impl WriteCommand {
    pub fn serialize(&self, output: &mut Vec<u8>) {
        output.push(self.command.to_command_code());
        output.push(self.wcc.to_ascii_compat());
        for order in self.orders.iter() {
            order.serialize(&mut *output);
        }
    }
}

impl Into<Vec<u8>> for &WriteCommand {
    fn into(self) -> Vec<u8> {
        let mut result = vec![];
        self.serialize(&mut result);
        result
    }
}

#[derive(Debug, Clone)]
pub struct IncomingRecord {
    pub aid: AID,
    pub addr: u16,
    pub orders: Vec<WriteOrder>,
}

fn parse_addr(encoded: &[u8]) -> Result<u16, StreamFormatError> {
    match encoded[0] >> 6 {
        0b00 => Ok(((encoded[0] as u16) << 8) + encoded[1] as u16),
        0b01 | 0b11 => {
            Ok((encoded[0] as u16 & 0x3F) << 6 | (encoded[1] as u16 & 0x3F))
        }
        _ => Err(StreamFormatError::InvalidData),
    }
}

impl IncomingRecord {
    pub fn parse_record(mut record: &[u8]) -> Result<Self, StreamFormatError> {
        if record.len() < 3 {
            return Err(StreamFormatError::UnexpectedEOR);
        }

        let aid = AID::try_from(record[0])?;
        let addr = parse_addr(&record[1..3])?;

        let mut result = Self {
            aid,
            addr,
            orders: vec![]
        };

        record = &record[3..];

        while record.len() > 0 {
            match record[0] {
                0x1D => {
                    ensure!(record.len() >= 2, UnexpectedEORSnafu);
                    result.orders.push(
                        WriteOrder::StartField(FieldAttribute::from_bits(record[1] & 0x3F)
                            .ok_or(StreamFormatError::InvalidData)?));
                    record = &record[2..];

                },
                0x29 => {
                    ensure!(record.len() >= 2, UnexpectedEORSnafu);
                    let (header, body) = record.split_at(2);
                    let count = header[1] as usize;
                    ensure!(body.len() >= count * 2, UnexpectedEORSnafu);
                    let (attrs, rest) = body.split_at(2 * count);
                    record = rest;

                    result.orders.push(
                        WriteOrder::StartFieldExtended(
                            attrs.chunks(2)
                            .map(ExtendedFieldAttribute::try_from)
                                .collect::<Result<Vec<ExtendedFieldAttribute>, StreamFormatError>>()?
                        )
                    )
                }
                0x11 => {
                    ensure!(record.len() >= 3, UnexpectedEORSnafu);
                    result.orders.push(WriteOrder::SetBufferAddress(parse_addr(&record[1..3])?));
                    record = &record[3..];
                }
                0x28 => {
                    ensure!(record.len() >= 3, UnexpectedEORSnafu);
                    result.orders.push(WriteOrder::SetAttribute(ExtendedFieldAttribute::try_from(&record[1..3])?));
                    record = &record[3..];
                }
                0x2C => {
                    ensure!(record.len() >= 2, UnexpectedEORSnafu);
                    let (header, body) = record.split_at(2);
                    let count = header[1] as usize;
                    ensure!(body.len() >= count * 2, UnexpectedEORSnafu);
                    let (attrs, rest) = body.split_at(2 * count);
                    record = rest;

                    result.orders.push(
                        WriteOrder::ModifyField(
                            attrs.chunks(2)
                                .map(ExtendedFieldAttribute::try_from)
                                .collect::<Result<Vec<ExtendedFieldAttribute>, StreamFormatError>>()?
                        )
                    )
                }
                0x13 => {
                    ensure!(record.len() >= 3, UnexpectedEORSnafu);
                    result.orders.push(WriteOrder::InsertCursor(parse_addr(&record[1..3])?));
                    record = &record[3..];
                }
                0x05 => {
                    result.orders.push(WriteOrder::ProgramTab);
                    record = &record[1..];
                }
                0x3C => {
                    ensure!(record.len() >= 4, UnexpectedEORSnafu);
                    result.orders.push(WriteOrder::RepeatToAddress(
                        parse_addr(&record[1..3])?,
                        crate::encoding::cp037::DECODE_TBL[record[4] as usize] as char,
                    ));
                    record = &record[4..]
                }
                0x12 => {
                    ensure!(record.len() >= 3, UnexpectedEORSnafu);
                    result.orders.push(WriteOrder::EraseUnprotectedToAddress(parse_addr(&record[1..3])?));
                    record = &record[3..];
                }
                0x08 => {
                    ensure!(record.len() >= 2, UnexpectedEORSnafu);
                    result.orders.push(WriteOrder::GraphicEscape(record[2]));
                    record = &record[2..];
                }
                0x40..=0xFF => {
                    let len = record.iter().position(|&v| v < 0x40).unwrap_or(record.len());
                    let data = record[..len]
                        .iter()
                        .map(|&v| crate::encoding::cp037::DECODE_TBL[v as usize] as char)
                        .collect();
                    result.orders.push(WriteOrder::SendText(data));
                    record = &record[len..];
                },
                _ => return Err(StreamFormatError::InvalidData)
            }
        }
        Ok(result)
    }
}