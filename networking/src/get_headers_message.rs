use elliptic_curves::helper::{int_to_little_endian, encode_varint};

use crate::message::Message;

const GET_HEADERS_COMMAND: &str = "getheaders";

#[derive(Debug, Default)]
pub struct GetHeadersMessage {
    version: u16,
    num_hashes: u16,
    start_block: Vec<u8>,
    end_block: Vec<u8>,
}


impl GetHeadersMessage {
    pub fn new(version: u16, num_hashes: u16, start_block: Vec<u8>, end_block: Vec<u8>) -> Self {
        Self {
            version,
            num_hashes,
            start_block,
            end_block,
        }
    }
}

impl Message for GetHeadersMessage {
    fn command() -> String where Self: Sized {
        GET_HEADERS_COMMAND.to_string()
    }

    fn parse(_stream: Vec<u8>) -> Self where Self: Sized {
        Self::default()
    }

    fn serialize(&self) -> Vec<u8> {
        let mut result = int_to_little_endian(self.version as u128, 4);
        result.append(&mut encode_varint(self.num_hashes as u128));

        let mut start_block_clone = self.start_block.clone();
        start_block_clone.reverse();
        result.append(&mut start_block_clone);

        let mut end_block_clone = self.end_block.clone();
        end_block_clone.reverse();
        result.append(&mut end_block_clone);

        result
    }
}
