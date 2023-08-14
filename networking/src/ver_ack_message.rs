pub struct VerAckMessage {}

impl VerAckMessage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(_s: &mut Vec<u8>) -> Self {
        Self {}
    }

    pub fn serialize(&self) -> Vec<u8> {
        b"".to_vec()
    }
}
