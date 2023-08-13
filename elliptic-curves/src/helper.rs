use std::io::{Error, Read};

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

pub fn int_to_little_endian(mut n: u128, size: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(size);
    for _ in 0..size {
        result.push((n & 0xff) as u8);
        n >>= 8;
    }
    // Reversing the order to match little-endian representation
    result.reverse();
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

pub fn bits_to_target(bits: &Vec<u8>) -> Option<u64> {
    if let Some(exponent) = bits.last() {
        let mut bits_clone = bits.clone();
        bits_clone.reverse();
        let coefficient = little_endian_bytes_to_u64(&bits_clone);
        return Some(coefficient * 256_u64.pow((*exponent - 3) as u32));
    };
    None
}

#[cfg(test)]
mod tests {
    use super::encode_base58;

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
}
