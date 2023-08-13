use std::io::{self, Read};

use elliptic_curves::helper::{
    bits_to_target, hash256, int_to_little_endian, little_endian_bytes_to_u64,
};

pub struct Block {
    version: u64,
    prev_block: Vec<u8>,
    merkle_root: Vec<u8>,
    timestamp: u64,
    bits: Vec<u8>,
    nonce: Vec<u8>,
}

impl Block {
    pub fn new(
        version: u64,
        prev_block: Vec<u8>,
        merkle_root: Vec<u8>,
        timestamp: u64,
        bits: Vec<u8>,
        nonce: Vec<u8>,
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

    pub fn parse(stream: &mut Vec<u8>) -> Result<Self, io::Error> {
        let mut stream = &stream[..];
        let mut version_buf = [0u8; 4];
        stream.read_exact(&mut version_buf)?;
        let version = little_endian_bytes_to_u64(&version_buf);

        let mut prev_block = [0u8; 32];
        stream.read_exact(&mut prev_block)?;
        prev_block.reverse();

        let mut merkle_root = [0u8; 32];
        stream.read_exact(&mut merkle_root)?;
        merkle_root.reverse();

        let mut timestamp_buf = [0u8; 4];
        stream.read_exact(&mut timestamp_buf)?;
        let timestamp = little_endian_bytes_to_u64(&timestamp_buf);

        let mut bits = [0u8; 4];
        stream.read_exact(&mut bits)?;

        let mut nonce = [0u8; 4];
        stream.read_exact(&mut nonce)?;

        Ok(Self {
            version,
            prev_block: prev_block.to_vec(),
            merkle_root: merkle_root.to_vec(),
            timestamp,
            bits: bits.to_vec(),
            nonce: nonce.to_vec(),
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = int_to_little_endian(self.version as u128, 4);
        let mut prev_block_clone = self.prev_block.clone();
        prev_block_clone.reverse();
        result.append(&mut prev_block_clone);

        let mut merkle_root_clone = self.merkle_root.clone();
        merkle_root_clone.reverse();
        result.append(&mut merkle_root_clone);

        result.append(&mut int_to_little_endian(self.timestamp as u128, 4));

        let mut bits_clone = self.bits.clone();
        bits_clone.reverse();
        result.append(&mut bits_clone);

        let mut nonce_clone = self.nonce.clone();
        nonce_clone.reverse();
        result.append(&mut nonce_clone);

        result
    }

    pub fn hash(&self) -> Vec<u8> {
        let s = self.serialize();
        let mut sha = hash256(&s);
        sha.reverse();

        sha.to_vec()
    }

    pub fn bip9(&self) -> bool {
        self.version >> 29 == 0b001
    }

    pub fn bip91(&self) -> bool {
        self.version >> 4 & 1 == 1
    }

    pub fn bip141(&self) -> bool {
        self.version >> 1 & 1 == 1
    }

    pub fn target(&self) -> Option<u64> {
        bits_to_target(&self.bits)
    }

    pub fn difficulty(&self) -> u64 {
        let lowest = 0xffff * 256 * (0x1d - 3);
        if let Some(target) = self.target() {
            return lowest / target;
        }
        panic!("cannot get target");
    }

    pub fn check_pow(&self) -> bool {
        let sha = hash256(&self.serialize());
        let proof = little_endian_bytes_to_u64(&sha);
        proof < self.target().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hex::FromHex;

    #[test]
    pub fn test_parse() {
        let hex = "020000208ec39428b17323fa0ddec8e887b\
4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3\
f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d";

        let mut stream = Vec::from_hex(hex).unwrap();
        let block = Block::parse(&mut stream).unwrap();

        assert_eq!(block.version, 0x20000002);

        let mut want = Vec::from_hex("000000000000000000fd0c220a0a8c3bc5a7b487e8c8de0dfa2373b12894c38e").unwrap();
        assert_eq!(block.prev_block , want);

        want = Vec::from_hex("be258bfd38db61f957315c3f9e9c5e15216857398d50402d5089a8e0fc50075b").unwrap();
        assert_eq!(block.merkle_root, want);

        assert_eq!(block.timestamp, 0x59a7771e);
        assert_eq!(block.bits, Vec::from_hex("e93c0118").unwrap());
        assert_eq!(block.nonce, Vec::from_hex("a4ffd71d").unwrap());
    }
}
