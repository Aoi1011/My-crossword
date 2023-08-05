use std::ops::Add;

use finite_fields::{FieldElement, P};
use helper::{encode_base58_checksum, hash160};
use num_bigint::BigUint;
use num_traits::{FromPrimitive, Num, One, Zero};
use signature::Signature;

pub mod helper;
pub mod private_key;
pub mod signature;

const A: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const B: &str = "0000000000000000000000000000000000000000000000000000000000000007";
const N: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
    pub a: FieldElement,
    pub b: FieldElement,
}

impl Point {
    // x.is_none() && y.is_none() -> the point at infinity
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: Option<FieldElement>,
        b: Option<FieldElement>,
    ) -> Self {
        let a = if a.is_none() {
            FieldElement::new(BigUint::from_str_radix(A, 16).unwrap(), None)
        } else {
            a.unwrap()
        };

        let b = if b.is_none() {
            FieldElement::new(BigUint::from_str_radix(B, 16).unwrap(), None)
        } else {
            b.unwrap()
        };

        if let (Some(x_field), Some(y_field)) = (x.clone(), y.clone()) {
            if y_field.to_the_power_of(BigUint::from_u8(2).unwrap())
                != x_field.to_the_power_of(BigUint::from_u8(3).unwrap())
                    + (a.clone() * x_field.clone())
                    + b.clone()
            {
                panic!("({:?}, {:?}) is not on the curve", x_field, y_field);
            }
        }

        Self { a, b, x, y }
    }

    pub fn equal(&self, other: Option<Point>) -> bool {
        *self == other.unwrap()
    }

    pub fn not_equal(&self, other: Option<Point>) -> bool {
        *self != other.unwrap()
    }

    pub fn rmul(&self, coefficient: BigUint) -> Self {
        let mut coef = coefficient;
        let mut current = self.clone();
        let mut result = Self::new(None, None, Some(self.a.clone()), Some(self.b.clone()));
        while coef.clone() != BigUint::zero() {
            if coef.clone() % (BigUint::one() + BigUint::one()) == BigUint::one() {
                result = result + current.clone();
            }
            current = current.clone() + current.clone();
            coef >>= 1;
        }
        result
    }

    pub fn is_infinity(&self) -> bool {
        if self.x.is_none() && self.y.is_none() {
            return true;
        }
        false
    }

    pub fn get_point_g() -> Self {
        let str_x = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let str_y = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
        let gx = BigUint::from_str_radix(str_x, 16).unwrap();
        let gy = BigUint::from_str_radix(str_y, 16).unwrap();

        let p = BigUint::from_u8(2).unwrap().pow(256_u32)
            - BigUint::from_u8(2).unwrap().pow(32_u32)
            - BigUint::from_u32(977).unwrap();

        let x = FieldElement::new(gx, Some(p.clone()));
        let y = FieldElement::new(gy, Some(p.clone()));

        let seven = FieldElement::new(BigUint::from_u8(7).unwrap(), None);
        let zero = FieldElement::new(BigUint::zero(), None);

        Self {
            x: Some(x),
            y: Some(y),
            a: zero,
            b: seven,
        }
    }

    pub fn verify(&self, z: &BigUint, sig: &Signature) -> bool {
        let n = BigUint::from_str_radix(N, 16).unwrap();
        let g = Point::get_point_g();

        let s_inv =
            FieldElement::mod_pow(&sig.s, n.clone() - (BigUint::one() + BigUint::one()), &n);
        let u = z * s_inv.clone() % n.clone();
        let v = sig.r.clone() * s_inv.clone() % n.clone();

        let total = g.rmul(u) + self.rmul(v);

        total.x.unwrap().num == sig.r
    }

    pub fn sec(&self, compressed: bool) -> Vec<u8> {
        let prefix_bytes: Vec<u8>;
        let x_bytes: Vec<u8>;
        let mut result = Vec::new();
        let concated_res: Vec<u8>;

        if compressed {
            if self.y.clone().unwrap().num % (BigUint::one() + BigUint::one()) == BigUint::zero() {
                prefix_bytes = b"\x02".to_vec();
                x_bytes = self.x.clone().unwrap().num.to_bytes_be();
            } else {
                prefix_bytes = b"\x03".to_vec();
                x_bytes = self.x.clone().unwrap().num.to_bytes_be();
            }
            result.push(prefix_bytes);
            result.push(x_bytes);

            concated_res = result.concat();
        } else {
            prefix_bytes = b"\x04".to_vec();
            x_bytes = self.x.clone().unwrap().num.to_bytes_be();
            let y_bytes = self.y.clone().unwrap().num.to_bytes_be();

            result.push(prefix_bytes);
            result.push(x_bytes);
            result.push(y_bytes);

            concated_res = result.concat();
        }

        concated_res
    }

    // when we get a serialized SEC pubkey, we can write a parse method to figure out which y we
    // need:
    pub fn parse(sec_bin: Vec<u8>) -> Self {
        if let Some(zero) = sec_bin.get(0) {
            if zero == &4 {
                let (left, right) = sec_bin.split_at(33);
                let x = BigUint::from_bytes_be(&left[1..]);
                let y = BigUint::from_bytes_be(&right);
                return Self::new(
                    Some(FieldElement::new(x, None)),
                    Some(FieldElement::new(y, None)),
                    None,
                    None,
                );
            }
        }

        let is_even = if sec_bin.get(0) == Some(&2) {
            true
        } else {
            false
        };

        let x = FieldElement::new(BigUint::from_bytes_be(&sec_bin[1..]), None);

        // right side of the equation y ^ 2 = x^ 3 + 7
        let alpha = x.to_the_power_of(BigUint::from_u8(3).unwrap())
            + FieldElement::new(BigUint::from_u8(7).unwrap(), None);
        // solve for left side
        let beta = alpha.sqrt();

        let even_beta: FieldElement;
        let odd_beta: FieldElement;
        if beta.clone().num % (BigUint::one() + BigUint::one()) == BigUint::zero() {
            even_beta = beta.clone();
            odd_beta = FieldElement::new(
                BigUint::from_str_radix(P, 16).unwrap() - beta.clone().num,
                None,
            );
        } else {
            even_beta = FieldElement::new(
                BigUint::from_str_radix(P, 16).unwrap() - beta.clone().num,
                None,
            );
            odd_beta = beta.clone();
        }

        if is_even {
            Self::new(Some(x), Some(even_beta), None, None)
        } else {
            Self::new(Some(x), Some(odd_beta), None, None)
        }
    }

    pub fn hash160(&self, compressed: bool) -> Vec<u8> {
        hash160(&self.sec(compressed))
    }

    pub fn address(&self, compressed: bool, testnet: bool) -> String {
        let h160 = self.hash160(compressed);
        let prefix;
        let mut b = Vec::new();

        if testnet {
            prefix = b"\x6f";
        } else {
            prefix = b"\x00";
        }
        b.append(&mut prefix.to_vec());
        b.append(&mut h160.to_vec());

        encode_base58_checksum(&mut b)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, rhs);
        }

        if self.x.is_none() {
            return rhs;
        }

        if rhs.x.is_none() {
            return self;
        }

        if self.x == rhs.x && self.y != rhs.y {
            return Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            };
        }

        if self.x != rhs.x {
            // s = (y2 - y1) / (x2 - x1)
            let s = (rhs.y.unwrap() - self.y.clone().unwrap())
                / (rhs.x.clone().unwrap() - self.x.clone().unwrap());
            // x3 = s ^ 2 - x1 - x2
            let x3 = s.to_the_power_of(BigUint::from_u8(2).unwrap())
                - self.x.clone().unwrap()
                - rhs.x.clone().unwrap();
            // y3 = s(x1 - x3) - y1
            let y3 = s * (self.x.unwrap() - x3.clone()) - self.y.unwrap();
            return Self {
                x: Some(x3),
                y: Some(y3),
                a: self.a,
                b: self.b,
            };
        }

        if self == rhs {
            let x_prime = self.x.clone().unwrap().prime;
            // s = (3x1 ^ 2 + a) / (2y1)
            let s = ((FieldElement::new(BigUint::from_u8(3).unwrap(), Some(x_prime.clone()))
                * self
                    .x
                    .clone()
                    .unwrap()
                    .to_the_power_of(BigUint::from_u8(2).unwrap()))
                + self.a.clone())
                / (FieldElement::new(BigUint::from_u8(2).unwrap(), Some(x_prime.clone()))
                    * (self.y.clone().unwrap()));
            // x3 = s ^ 2 - 2x1
            let x3 = s.to_the_power_of(BigUint::from_u8(2).unwrap())
                - (FieldElement::new(BigUint::from_u8(2).unwrap(), Some(x_prime.clone()))
                    * self.x.clone().unwrap());
            // y3 = s(x1 - x3) - y1
            let y3 = s * (self.x.unwrap() - x3.clone()) - self.y.unwrap();

            return Self {
                x: Some(x3),
                y: Some(y3),
                a: self.a,
                b: self.b,
            };
        }

        if self == rhs
            && self.y.unwrap()
                == FieldElement::zero(self.x.clone().unwrap().prime) * self.x.unwrap()
        {
            return Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            };
        }

        Self {
            x: rhs.x,
            y: rhs.y,
            a: rhs.a,
            b: rhs.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use finite_fields::FieldElement;
    use num_bigint::BigUint;
    use num_traits::{FromPrimitive, Num, One};

    use crate::{helper::hash256, private_key::PrivateKey, signature::Signature, Point, N};

    #[test]
    fn test_secp256k1() {
        let x = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let y = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

        let gx = BigUint::from_str_radix(x, 16).unwrap();
        let gy = BigUint::from_str_radix(y, 16).unwrap();

        let p = BigUint::from_u8(2).unwrap().pow(256_u32)
            - BigUint::from_u8(2).unwrap().pow(32_u32)
            - BigUint::from_u32(977).unwrap();

        assert_eq!(
            gy.pow(2) % &p,
            (gx.pow(3) + BigUint::from_u32(7).unwrap()) % &p
        );
    }

    #[test]
    fn test_infinity() {
        let n = BigUint::from_str_radix(N, 16).unwrap();
        let g = Point::get_point_g();

        let ng = g.rmul(n);

        assert!(ng.is_infinity());
    }

    #[test]
    fn test_verify_signature() {
        let z = "bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423";
        let r = "37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6";
        let s = "8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec";
        let px = "04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574";
        let py = "82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4";

        let px = BigUint::from_str_radix(px, 16).unwrap();
        let py = BigUint::from_str_radix(py, 16).unwrap();
        let z = BigUint::from_str_radix(z, 16).unwrap();
        let r = BigUint::from_str_radix(r, 16).unwrap();
        let s = BigUint::from_str_radix(s, 16).unwrap();

        let point = Point::new(
            Some(FieldElement::new(px, None)),
            Some(FieldElement::new(py, None)),
            None,
            None,
        );

        let sig = Signature::new(r, s);

        assert!(point.verify(&z, &sig))
    }

    #[test]
    fn test_create_signature() {
        let e = BigUint::from_bytes_be(&hash256(b"my secret"));
        // 0x231c6f3d980a6b0fb7152f85cee7eb52bf92433d9919b9c5218cb08e79cce78
        let z = BigUint::from_bytes_be(&hash256(b"my message"));
        let n = BigUint::from_str_radix(N, 16).unwrap();

        let k = BigUint::from_u32(1234567890).unwrap();

        let g = Point::get_point_g();

        // 0x2b698a0f0a4041b77e63488ad48c23e8e8838dd1fb7520408b121697b782ef22
        let r = g.rmul(k.clone()).x.unwrap().num;

        let k_inv = FieldElement::mod_pow(&k, n.clone() - (BigUint::one() + BigUint::one()), &n);

        // 0xbb14e602ef9e3f872e25fad328466b34e6734b7a0fcd58b1eb635447ffae8cb9
        let _s = (z.clone() + r.clone() * e.clone()) * k_inv % n;

        let point = g.rmul(e.clone());

        // Point {
        //     x: 028d003eab2e428d11983f3e97c3fa0addf3b42740df0d211795ffb3be2f6c52,
        //     y: 0ae987b9ec6ea159c78cb2a937ed89096fb218d9e7594f02b547526d8cd309e2,
        //
        // }
        // println!(
        //     "Point.x: {:?}, Point.y: {:?}",
        //     point.x.unwrap().num.to_str_radix(16),
        //     point.y.unwrap().num.to_str_radix(16)
        // );

        let pri_key = PrivateKey::new(e.clone());
        let sig = pri_key.sign(z.clone());

        assert!(point.verify(&z, &sig))
    }

    #[test]
    fn test_sec() {
        let mut pri_key = PrivateKey::new(BigUint::from_u16(5000).unwrap());
        let mut serialized = pri_key.point.sec(false);
        let mut res = String::new();
        for byte in serialized {
            res.push_str(format!("{:02x}", byte).as_str());
        }

        assert_eq!(
            res,
            "04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4\
f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10"
        );
        res.clear();

        pri_key = PrivateKey::new(BigUint::from_u64(2018_u64.pow(5)).unwrap());
        serialized = pri_key.point.sec(false);
        res = String::new();
        for byte in serialized {
            res.push_str(format!("{:02x}", byte).as_str());
        }

        assert_eq!(
            res,
            "04027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023\
c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06"
        );
        res.clear();
    }

    #[test]
    fn test_compressed_sec() {
        let mut pri_key = PrivateKey::new(BigUint::from_u16(5001).unwrap());
        let mut serialized = pri_key.point.sec(true);
        let mut res = String::new();
        for byte in serialized {
            res.push_str(format!("{:02x}", byte).as_str());
        }

        assert_eq!(
            res,
            "0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1"
        );
        res.clear();

        pri_key = PrivateKey::new(BigUint::from_u64(2019_u64.pow(5)).unwrap());
        serialized = pri_key.point.sec(true);
        res = String::new();
        for byte in serialized {
            res.push_str(format!("{:02x}", byte).as_str());
        }

        assert_eq!(
            res,
            "02933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb8701"
        );
        res.clear();
    }

    #[test]
    fn test_address() {
        let mut pri_key = PrivateKey::new(BigUint::from_u16(5002).unwrap());
        let mut address = pri_key.point.address(false, true);

        assert_eq!(address, "mmTPbXQFxboEtNRkwfh6K51jvdtHLxGeMA");

        pri_key = PrivateKey::new(BigUint::from_u64(2020_u64.pow(5)).unwrap());
        address = pri_key.point.address(true, true);

        assert_eq!(address, "mopVkxp8UhXqRYbCYJsbeE1h1fiF64jcoH");

        pri_key = PrivateKey::new(BigUint::from_str_radix("12345deadbeef", 16).unwrap());
        address = pri_key.point.address(true, false);

        assert_eq!(address, "1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1");

        let hello_world = hex::encode("Hello World");
        pri_key = PrivateKey::new(BigUint::from_str_radix(&hello_world, 16).unwrap());
        address = pri_key.point.address(true, true);

        println!("{:?}", address);
    }
}
