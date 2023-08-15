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
