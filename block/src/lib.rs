use std::io::{self, Read};

use elliptic_curves::helper::{
    bits_to_target, hash256, int_to_little_endian, little_endian_bytes_to_u64, merkle_root,
};
use num_bigint::BigUint;
use num_traits::FromPrimitive;

pub const GENESIS_BLOCK: &str = "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c";
pub const TESTNET_GENESIS_BLOCK: &str = "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4adae5494dffff001d1aa4ae18";
pub const LOWEST_BITS: &str = "ffff001d";

pub struct Block {
    version: u64,
    prev_block: Vec<u8>,
    merkle_root: Vec<u8>,
    timestamp: u64,
    bits: Vec<u8>,
    nonce: Vec<u8>,
    tx_hashes: Vec<[u8; 32]>,
}

impl Block {
    pub fn new(
        version: u64,
        prev_block: Vec<u8>,
        merkle_root: Vec<u8>,
        timestamp: u64,
        bits: Vec<u8>,
        nonce: Vec<u8>,
        tx_hashes: Vec<[u8; 32]>,
    ) -> Self {
        Self {
            version,
            prev_block,
            merkle_root,
            timestamp,
            bits,
            nonce,
            tx_hashes,
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
            tx_hashes: Vec::new(),
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
        result.append(&mut bits_clone);

        let mut nonce_clone = self.nonce.clone();
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

    pub fn target(&self) -> BigUint {
        bits_to_target(&self.bits).unwrap()
    }

    pub fn difficulty(&self) -> BigUint {
        let a = BigUint::from_u64(0xffff).unwrap();
        let base = BigUint::from_u64(256).unwrap();
        let lowest = a * base.pow(0x1d - 3);
        let target = self.target();
        lowest / target
    }

    pub fn check_pow(&self) -> bool {
        let sha = hash256(&self.serialize());
        let proof = little_endian_bytes_to_u64(&sha);
        BigUint::from_u64(proof).unwrap() < self.target()
    }

    pub fn validate_merkle_root(&self) -> bool {
        let mut hashes: Vec<[u8; 32]> = self
            .tx_hashes
            .iter()
            .map(|h| {
                let mut hash = *h;
                hash.reverse();
                hash
            })
            .collect();

        let mut root = merkle_root(&mut hashes);
        root.reverse();

        root == self.merkle_root
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    use hex::FromHex;
    use num_traits::Num;

    #[test]
    fn test_parse() {
        let hex = "020000208ec39428b17323fa0ddec8e887b\
4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3\
f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d";

        let mut stream = Vec::from_hex(hex).unwrap();
        let block = Block::parse(&mut stream).unwrap();

        assert_eq!(block.version, 0x20000002);

        let mut want =
            Vec::from_hex("000000000000000000fd0c220a0a8c3bc5a7b487e8c8de0dfa2373b12894c38e")
                .unwrap();
        assert_eq!(block.prev_block, want);

        want = Vec::from_hex("be258bfd38db61f957315c3f9e9c5e15216857398d50402d5089a8e0fc50075b")
            .unwrap();
        assert_eq!(block.merkle_root, want);

        assert_eq!(block.timestamp, 0x59a7771e);
        assert_eq!(block.bits, Vec::from_hex("e93c0118").unwrap());
        assert_eq!(block.nonce, Vec::from_hex("a4ffd71d").unwrap());
    }

    #[test]
    fn test_serialize() {
        let mut block_row = Vec::from_hex("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let block = Block::parse(&mut block_row).unwrap();
        assert_eq!(block.serialize(), block_row);
    }

    #[test]
    fn test_hash() {
        let mut block_row = Vec::from_hex("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let block = Block::parse(&mut block_row).unwrap();
        assert_eq!(
            block.hash(),
            Vec::from_hex("0000000000000000007e9e4c586439b0cdbe13b1370bdd9435d76a644d047523")
                .unwrap()
        );
    }

    #[test]
    fn test_bip9() {
        let mut block_row = Vec::from_hex("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let mut block = Block::parse(&mut block_row).unwrap();
        assert!(block.bip9());

        block_row = Vec::from_hex("0400000039fa821848781f027a2e6dfabbf6bda920d9ae61b63400030000000000000000ecae536a304042e3154be0e3e9a8220e5568c3433a9ab49ac4cbb74f8df8e8b0cc2acf569fb9061806652c27").unwrap();
        block = Block::parse(&mut block_row).unwrap();
        assert!(!block.bip9());
    }

    #[test]
    fn test_bip91() {
        let mut block_row = Vec::from_hex("1200002028856ec5bca29cf76980d368b0a163a0bb81fc192951270100000000000000003288f32a2831833c31a25401c52093eb545d28157e200a64b21b3ae8f21c507401877b5935470118144dbfd1").unwrap();
        let mut block = Block::parse(&mut block_row).unwrap();
        assert!(block.bip91());

        block_row = Vec::from_hex("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        block = Block::parse(&mut block_row).unwrap();
        assert!(!block.bip91());
    }

    #[test]
    fn test_bip141() {
        let mut block_row = Vec::from_hex("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let mut block = Block::parse(&mut block_row).unwrap();
        assert!(block.bip141());

        block_row = Vec::from_hex("0000002066f09203c1cf5ef1531f24ed21b1915ae9abeb691f0d2e0100000000000000003de0976428ce56125351bae62c5b8b8c79d8297c702ea05d60feabb4ed188b59c36fa759e93c0118b74b2618").unwrap();
        block = Block::parse(&mut block_row).unwrap();
        assert!(!block.bip141());
    }

    #[test]
    fn test_target() {
        let mut block_row = Vec::from_hex("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let block = Block::parse(&mut block_row).unwrap();
        let target = block.target();
        assert_eq!(
            target,
            BigUint::from_str_radix("13ce9000000000000000000000000000000000000000000", 16).unwrap()
        );
        assert_eq!(
            block.difficulty(),
            BigUint::from_str("888171856257").unwrap()
        );
    }

    #[test]
    fn test_difficulty() {
        let mut block_row = Vec::from_hex("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let block = Block::parse(&mut block_row).unwrap();
        assert_eq!(
            block.difficulty(),
            BigUint::from_str("888171856257").unwrap()
        );
    }

    #[test]
    fn test_check_pow() {
        let mut block_row = Vec::from_hex("04000000fbedbbf0cfdaf278c094f187f2eb987c86a199da22bbb20400000000000000007b7697b29129648fa08b4bcd13c9d5e60abb973a1efac9c8d573c71c807c56c3d6213557faa80518c3737ec1").unwrap();
        let block = Block::parse(&mut block_row).unwrap();
        assert!(block.check_pow());

        block_row = Vec::from_hex("04000000fbedbbf0cfdaf278c094f187f2eb987c86a199da22bbb20400000000000000007b7697b29129648fa08b4bcd13c9d5e60abb973a1efac9c8d573c71c807c56c3d6213557faa80518c3737ec1").unwrap();
        let block = Block::parse(&mut block_row).unwrap();
        assert!(block.check_pow());
    }
}
