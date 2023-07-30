use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{FromPrimitive, ToPrimitive, Zero};

const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

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
