use std::io::Read;

use bitcoin::consensus::Decodable;
use elliptic_curves::helper::read_varint;

pub struct Script(Vec<u8>);

pub mod op;

enum Cmd {

}

impl Script {
    pub fn new(cmds: Vec<u8>) -> Self {
        Self(cmds)
    }

    pub fn parse(stream: &mut Vec<u8>) -> Self {
        let stream = &stream[..];
        let length = read_varint(&mut stream);
        let mut cmds = Vec::new();
        let mut count = 0;

        while count < length {
            let mut buf = [0u8; 1];
            stream.read_exact(&mut buf).unwrap();
            count += 1;
            let current_byte = buf[0];

            if current_byte >= 1 && current_byte <= 75 {
                let n = current_byte;
                let mut buf = vec![0; n as usize];
                cmds.append(stream.read_exact(&mut buf));
                count += n;
            }
        }

        Self(cmds)
        
    }
}

impl Decodable for Script {
    fn consensus_decode_from_finite_reader<R: std::io::Read + ?Sized>(
        _reader: &mut R,
    ) -> Result<Self, bitcoin::consensus::encode::Error> {
        Ok(Self(Vec::new()))
    }
}

#[cfg(test)]
mod tests {}
