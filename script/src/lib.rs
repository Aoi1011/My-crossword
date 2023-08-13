use std::{
    io::{self, Read},
    ops::Add,
};

use bitcoin::consensus::Decodable;
use elliptic_curves::helper::{
    encode_varint, int_to_little_endian, little_endian_bytes_to_u64, read_varint,
};

pub mod op;

#[derive(Debug, Clone)]
pub enum Cmd {
    OpCode(u8),
    PushData(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Script {
    pub cmds: Vec<Cmd>,
}

impl Script {
    pub fn new(cmds: Vec<Cmd>) -> Self {
        Self { cmds }
    }

    pub fn parse<R: Read>(mut s: R) -> Result<Self, io::Error> {
        let length = read_varint(&mut s)?;

        let mut cmds = Vec::new();
        let mut count = 0;

        while count < length {
            let mut current_byte = [0u8; 1];
            s.read_exact(&mut current_byte)?;

            let current_byte = current_byte[0];
            count += 1;

            if (1..=75).contains(&current_byte) {
                let n = current_byte;
                let mut cmd_data = vec![0; n as usize];
                s.read_exact(&mut cmd_data)?;
                cmds.push(Cmd::PushData(cmd_data));
                count += n as u64;
            } else if current_byte == 76 {
                let mut data_length_buf = [0u8; 1];
                s.read_exact(&mut data_length_buf)?;
                let data_length = little_endian_bytes_to_u64(&data_length_buf) as usize;
                let mut cmd_data = vec![0u8; data_length];
                s.read_exact(&mut cmd_data)?;
                cmds.push(Cmd::PushData(cmd_data));
                count += (data_length + 1) as u64;
            } else if current_byte == 77 {
                let mut data_length_buf = [0u8; 2];
                s.read_exact(&mut data_length_buf)?;
                let data_length = little_endian_bytes_to_u64(&data_length_buf) as usize;
                let mut cmd_data = vec![0u8; data_length];
                s.read_exact(&mut cmd_data)?;
                cmds.push(Cmd::PushData(cmd_data));
                count += (data_length + 2) as u64;
            } else {
                cmds.push(Cmd::OpCode(current_byte));
            }
        }

        if count != length {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "passing script failed",
            ));
        }

        Ok(Self { cmds })
    }

    pub fn raw_serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();

        for cmd in &self.cmds {
            match cmd {
                Cmd::OpCode(op_code) => {
                    res.append(&mut int_to_little_endian(*op_code as u128, 1));
                }

                Cmd::PushData(ref data) => {
                    let length = data.len();
                    if length < 75 {
                        res.append(&mut int_to_little_endian(length as u128, 1));
                    } else if length > 75 && length < 0x100 {
                        res.append(&mut int_to_little_endian(76_u128, 1));
                        res.append(&mut int_to_little_endian(length as u128, 1));
                    } else if length >= 0x100 && length <= 520 {
                        res.append(&mut int_to_little_endian(77_u128, 1));
                        res.append(&mut int_to_little_endian(length as u128, 2));
                    } else {
                        panic!("too long an cmd")
                    }

                    res.append(&mut data.clone());
                }
            }
        }

        res
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = self.raw_serialize();
        let total = result.len();

        let mut encoded = encode_varint(total as u128);
        encoded.append(&mut result);
        encoded
    }
}

impl Add for Script {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Vec::new();
        res.append(&mut self.cmds.clone());
        res.append(&mut rhs.cmds.clone());

        Self { cmds: res }
    }
}

impl Decodable for Script {
    fn consensus_decode_from_finite_reader<R: std::io::Read + ?Sized>(
        _reader: &mut R,
    ) -> Result<Self, bitcoin::consensus::encode::Error> {
        Ok(Self { cmds: Vec::new() })
    }
}

#[cfg(test)]
mod tests {}
