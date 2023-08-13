use std::io::{self, Read};

use bitcoin::consensus::Decodable;
use elliptic_curves::helper::{little_endian_bytes_to_u64, read_varint};

pub mod op;

pub enum Cmd {
    OpCode(u8),
    PushData(Vec<u8>),
}

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
