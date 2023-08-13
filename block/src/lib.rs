pub struct Block {
    version: u64,
    prev_block: u64,
    merkle_root: u64,
    timestamp: u32,
    bits: u64,
    nonce: u32,
}

impl Block {
    pub fn new(
        version: u64,
        prev_block: u64,
        merkle_root: u64,
        timestamp: u32,
        bits: u64,
        nonce: u32,
    ) -> Self {
        Self {
            version,
            prev_block,
            merkle_root,
            timestamp,
            bits,
            nonce,
        }
    }

    pub fn parse(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
