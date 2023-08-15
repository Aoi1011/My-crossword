use crate::message::Message;

const VER_ACK_COMMAND: &str = "verack";

pub struct VerAckMessage {}

impl VerAckMessage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Message for VerAckMessage {
    fn command() -> String {
        VER_ACK_COMMAND.to_string()
    }

    fn parse(stream: Vec<u8>) -> Self {
        Self {}
    }

    fn serialize(&self) -> Vec<u8> {
        b"".to_vec()
    }
}
