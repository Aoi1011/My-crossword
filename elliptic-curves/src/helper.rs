use std::{
    io::{Error, Read},
    thread::current,
};

use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{FromPrimitive, ToPrimitive, Zero};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

pub const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn hash160(s: &[u8]) -> Vec<u8> {
    let mut hasher1 = Sha256::new();
    hasher1.update(s);
    let digest = hasher1.finalize();

    let res = Ripemd160::digest(digest);
    res.to_vec()
}

pub fn hash256(s: &[u8]) -> [u8; 32] {
    // First round of SHA-256
    let mut hasher1 = Sha256::new();
    hasher1.update(s);
    let first_round_digest = hasher1.finalize();

    // Second round of SHA-256
    let mut hasher2 = Sha256::new();
    hasher2.update(first_round_digest);
    let final_digest = hasher2.finalize();

    // Convert the final_digest to an array of 32 bytes
    let mut result = [0u8; 32];
    result.copy_from_slice(&final_digest);

    result
}

pub fn encode_base58(s: &Vec<u8>) -> String {
    let mut result = String::new();
    let mut count = 0;

    for c in s {
        if c == &0 {
            count += 1;
        } else {
            break;
        }
    }

    let num = BigUint::from_bytes_be(&s);
    let prefix = "1".repeat(count);

    let radix = BigUint::from_u8(58).unwrap();

    let mut num = num;
    while num > BigUint::zero() {
        let (new_num, rem) = num.div_rem(&radix);
        num = new_num;
        let index = rem
            .to_usize()
            .expect("Remainder must be within usize bounds");
        result.insert(
            0,
            BASE58_ALPHABET.chars().nth(index).expect("Invalid index"),
        );
    }

    prefix + &result
}

pub fn encode_base58_checksum(b: &mut Vec<u8>) -> String {
    let hashed_b = &hash256(b)[..4];
    b.append(&mut hashed_b.to_vec());
    encode_base58(b)
}

// pub fn decode_base58(s: &Vec<u8>) -> Vec<u8> {
//     let mut num = 0;
//
//     for c in s {
//         num *= 58;
//         num += BASE58_ALPHABET.chars().
//     }
// }

pub fn little_endian_bytes_to_u64(bytes: &[u8]) -> u64 {
    let mut result = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        result += (byte as u64) << (8 * i);
    }
    result
}

pub fn int_to_little_endian(n: u128, size: usize) -> Vec<u8> {
    let mut result = vec![0; size];
    result.copy_from_slice(&n.to_le_bytes()[..size]);
    result
}

/// read_varint reads a variable integer from a stream
pub fn read_varint<R: Read>(stream: &mut R) -> Result<u64, Error> {
    let mut i_buf = [0u8; 1];
    stream.read_exact(&mut i_buf)?;

    let i = i_buf[0];

    match i {
        0xfd => {
            // 0xfd means the next two bytes are the number
            let mut buf = [0u8; 2];
            stream.read_exact(&mut buf)?;
            Ok(little_endian_bytes_to_u64(&buf))
        }
        0xfe => {
            // 0xfe means the next four bytes are the number
            let mut buf = [0u8; 4];
            stream.read_exact(&mut buf)?;
            Ok(little_endian_bytes_to_u64(&buf))
        }
        0xff => {
            // 0xff means the next eight bytes are the number
            let mut buf = [0u8; 8];
            stream.read_exact(&mut buf)?;
            Ok(u64::from(buf[0])
                | (u64::from(buf[1]) << 8)
                | (u64::from(buf[2]) << 16)
                | (u64::from(buf[3]) << 24)
                | (u64::from(buf[4]) << 32)
                | (u64::from(buf[5]) << 40)
                | (u64::from(buf[6]) << 48)
                | (u64::from(buf[7]) << 56))
        }
        _ => Ok(i as u64),
    }
}

/// encodes an integer as a varint
pub fn encode_varint(i: u128) -> Vec<u8> {
    match i {
        i if i < 0xfd => vec![i as u8],
        i if i < 0x10000 => {
            let mut result = Vec::new();
            result.push(0xfd);
            result.extend_from_slice(&int_to_little_endian(i, 2));
            result
        }
        i if i < 0x100000000 => {
            let mut result = Vec::new();
            result.push(0xfe);
            result.extend_from_slice(&int_to_little_endian(i, 4));
            result
        }
        i if i < 0x10000000000000000 => {
            let mut result = Vec::new();
            result.push(0xff);
            result.extend_from_slice(&int_to_little_endian(i, 8));
            result
        }
        _ => {
            panic!("integer too large: {}", i)
        }
    }
}

pub fn bits_to_target(bits: &Vec<u8>) -> Option<BigUint> {
    if let Some(exponent) = bits.last() {
        let bits_clone = bits.clone();
        let bits_without_last = bits_clone.split_last().map(|(_, rest)| rest).unwrap();
        let coefficient = little_endian_bytes_to_u64(&bits_without_last);
        let coefficient = BigUint::from_u64(coefficient).unwrap();
        let base = BigUint::from_u64(256).unwrap();
        let res = coefficient * base.pow(*exponent as u32 - 3);
        return Some(res);
    };
    None
}

pub fn merkle_parent(hash1: &mut Option<Vec<u8>>, hash2: &mut Option<Vec<u8>>) -> Vec<u8> {
    let mut res = Vec::new();
    match (hash1, hash2) {
        (Some(h1), Some(h2)) => {
            res.append(&mut h1.to_vec());
            res.append(&mut h2.to_vec());
            hash256(&res).to_vec()
        }
        _ => Vec::new(),
    }
}

pub fn merkle_parent_level(hashes: &mut Vec<Option<Vec<u8>>>) -> Vec<Option<Vec<u8>>> {
    if hashes.len() == 1 {
        panic!("cannot take a parent level with only 1 item");
    }

    if hashes.len() % 2 == 1 {
        let last = hashes.last().unwrap();
        let last_clone = last.clone();
        hashes.push(last_clone);
    }

    let parent_level = hashes
        .chunks(2)
        .map(|hash| {
            let mut first_clone = hash[0].clone();
            let mut second_clone = hash[1].clone();
            let parent = merkle_parent(&mut first_clone, &mut second_clone);
            Some(parent)
        })
        .collect::<Vec<Option<Vec<u8>>>>();

    parent_level
}

// pub fn merkle_root(hashes: &mut Vec<[u8; 32]>) -> Vec<u8> {
//     let mut current_level = hashes.clone();
//     while current_level.len() > 1 {
//         if let Some(level) = merkle_parent_level(&mut current_level) {
//             current_level = level;
//         }
//     }
//
//     current_level[0].to_vec()
// }

#[cfg(test)]
mod tests {
    use hex::{FromHex, ToHex};

    use super::{encode_base58, merkle_parent, merkle_parent_level};

    #[test]
    fn test_encode_base58() {
        let mut h = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
        let mut s = hex::decode(h).unwrap();
        let mut res = encode_base58(&s);

        assert_eq!(res, "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6");

        h = "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c";
        s = hex::decode(h).unwrap();
        res = encode_base58(&s);

        assert_eq!(res, "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd");

        h = "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6";
        s = hex::decode(h).unwrap();
        res = encode_base58(&s);

        assert_eq!(res, "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7");
    }

    #[test]
    fn test_merkle_parent() {
        let hash0 = hex::decode("c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5")
            .unwrap();
        let hash1 = hex::decode("c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5")
            .unwrap();

        let parent = merkle_parent(&mut Some(hash0), &mut Some(hash1));

        assert_eq!(
            parent.encode_hex::<String>(),
            "8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd".to_string()
        );
    }

    #[test]
    fn test_merkle_parent_level() {
        let hex_hashes = vec![
            "c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5",
            "c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5",
            "f391da6ecfeed1814efae39e7fcb3838ae0b02c02ae7d0a5848a66947c0727b0",
            "3d238a92a94532b946c90e19c49351c763696cff3db400485b813aecb8a13181",
            "10092f2633be5f3ce349bf9ddbde36caa3dd10dfa0ec8106bce23acbff637dae",
        ];
        let mut hashes: Vec<Option<Vec<u8>>> = hex_hashes
            .iter()
            .map(|h| Some(Vec::from_hex(h).unwrap()))
            .collect();

        let parent = merkle_parent_level(&mut hashes);

        let mut res = Vec::new();
        for bytes in parent {
            if let Some(byte) = bytes {
                res.push(byte.encode_hex::<String>());
            }
        }

        let expect = vec![
            "8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd",
            "7f4e6f9e224e20fda0ae4c44114237f97cd35aca38d83081c9bfd41feb907800",
            "3ecf6115380c77e8aae56660f5634982ee897351ba906a6837d15ebc3a225df0",
        ];

        assert_eq!(res, expect);
    }
}
