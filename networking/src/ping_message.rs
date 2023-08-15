use std::io::Read;

use crate::message::Message;

pub const PING_COMMAND: &str = "ping";

struct PingMessage {
    nonce: Vec<u8>,
}

impl PingMessage {
    pub fn new(nonce: Vec<u8>) -> Self {
        Self { nonce }
    }
}

impl Message for PingMessage {
    fn command() -> String
    where
        Self: Sized,
    {
        PING_COMMAND.to_string()
    }

    fn parse(stream: Vec<u8>) -> Self
    where
        Self: Sized,
    {
        let mut stream = &stream[..];
        let mut nonce = [0u8; 8];
        stream.read_exact(&mut nonce).unwrap();

        Self {
            nonce: nonce.to_vec(),
        }
    }

    fn serialize(&self) -> Vec<u8> {
        self.nonce.clone()
    }
}
