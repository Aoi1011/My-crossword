use block::Block;
use elliptic_curves::helper::read_varint;

use crate::message::Message;

const HEADERS_MESSAGE_COMMAND: &str = "headers";

pub struct HeadersMessage {
    pub blocks: Vec<Block>,
}

impl HeadersMessage {
    pub fn new(blocks: Vec<Block>) -> Self {
        Self { blocks }
    }
}

impl Message for HeadersMessage {
    fn command() -> String
    where
        Self: Sized,
    {
        HEADERS_MESSAGE_COMMAND.to_string()
    }

    fn parse(stream: Vec<u8>) -> Self
    where
        Self: Sized,
    {
        let mut stream = &stream[..];
        let num_headers = read_varint(&mut stream).unwrap();

        let mut blocks = Vec::new();
        for _ in 0..num_headers {
            blocks.push(Block::parse(&mut stream.to_vec()).unwrap());
            let num_txs = read_varint(&mut stream).unwrap();
            if num_txs != 0 {
                panic!("number of txs not 0");
            }
        }

        Self { blocks }
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use hex::FromHex;

    use crate::message::Message;

    use super::HeadersMessage;

    #[test]
    fn test_headers_message_parse() {
        let hex_msg = "0200000020df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0793a500000000000000ef445fef2ed495c275892206ca533e7411907971013ab83e3b47bd0d692d14d4dc7c835b67d8001ac157e670000000002030eb2540c41025690160a1014c577061596e32e426b712c7ca00000000000000768b89f07044e6130ead292a3f51951adbd2202df447d98789339937fd006bd44880835b67d8001ade09204600";
        let stream = Vec::from_hex(hex_msg).unwrap();
        let headers = HeadersMessage::parse(stream);
        
        assert_eq!(headers.blocks.len(), 2);
    }
}
