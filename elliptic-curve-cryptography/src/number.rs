use std::{
    fmt::Display,
    ops::{Add, BitAnd, Div, Mul, Rem, Shr, Sub},
    str,
};

use ibig::{ibig, IBig, UBig};
use num_bigint::{BigInt, RandBigInt, Sign};
use num_traits::{One, Zero};
use rand::Rng;

pub trait Number
where
    Self: Sized,
    Self: From<u32>,
    Self: Display,
    Self: PartialOrd,
    Self: Add<Self, Output = Self>,
    Self: for<'a> Add<&'a Self, Output = Self>,
    Self: Sub<Self, Output = Self>,
    Self: for<'a> Sub<&'a Self, Output = Self>,
    Self: Rem<Self, Output = Self>,
    Self: for<'a> Rem<&'a Self, Output = Self>,
    Self: Mul<Self, Output = Self>,
    Self: for<'a> Mul<&'a Self, Output = Self>,
    Self: Div<Self, Output = Self>,
    Self: for<'a> Div<&'a Self, Output = Self>,
    Self: Shr<usize, Output = Self>,
    Self: BitAnd,
    Self: Zero + One,
    Self: Clone,
    Self: PartialEq,
{
    fn mod_cal(a: &Self, p: &Self) -> Self;
    fn pow(&self, exp: u32) -> Self;
    fn mod_pow(a: &Self, e: &Self, p: &Self) -> Self;
    fn exgcd(&self, b: &Self) -> (Self, Self, Self);
    fn mod_inv(a: &Self, p: &Self) -> Self;
    fn jacobi(&self, q: &Self) -> i32;
    fn mod_sqrt(a: &Self, p: &Self) -> Self;
    fn to_hex(&self) -> String;
    fn add_ref(&self, rhs: &Self) -> Self;
    fn sub_ref(&self, rhs: &Self) -> Self;
    fn mul_ref(&self, rhs: &Self) -> Self;
    fn gen_rand(min: &Self, max: &Self) -> Self;
    fn bit_len(&self) -> usize;
    fn test_bit(&self, bit: usize) -> bool;
    fn from_bytes_radix(buf: &[u8], radix: u32) -> Self; // b"..."
    fn from_bytes_be(bytes: &[u8]) -> Self;
    fn to_bytes_be(&self) -> Vec<u8>;
}

impl Number for IBig {
    fn mod_cal(a: &Self, p: &Self) -> Self {
        let mut x = a % p;
        if x < IBig::zero() {
            x += p;
        }
        x
    }

    fn pow(&self, exp: u32) -> Self {
        self.pow(exp as usize)
    }

    fn mod_pow(a: &Self, e: &Self, p: &Self) -> Self {
        if e.is_zero() {
            return ibig!(1);
        }
        let mut ex = e.clone();
        let mut res = ibig!(1);
        let mut t = a % p;
        while ex > ibig!(0) {
            if e & 1 == ibig!(1) {
                res = res * a % p;
            }
            t = &t * &t % p;
            ex >>= 1;
        }
        res
    }

    /// gcd --- greatest common divisor
    /// Extended Euclidean Algorithm also finds integers s and t such that 
    /// as + bt = gcd(a, b)
    fn exgcd(&self, b: &Self) -> (Self, Self, Self) {
        let (mut q, mut r, mut s, mut t);
        let (mut x, mut y) = (self.clone(), b.clone());
        let (mut s0, mut s1) = (IBig::one(), IBig::zero());
        let (mut t0, mut t1) = (IBig::zero(), IBig::one());
        while y > IBig::zero() {
            q = &x / &y;
            r = &x % &y;
            s = &s0 - &q * &s1;
            t = &t0 - &q * &t1;
            x = y;
            y = r;
            s0 = s1;
            t0 = t1;
            s1 = s;
            t1 = t;
        }
        (x, s0, t0)
    }

    /// mod inverse n^(-1) mod p
    fn mod_inv(a: &Self, p: &Self) -> Self {
        // assert!(n * modinv(n, p) == 1);
        let mut m = a.clone();

        if m < IBig::zero() {
            m = &m + p;
        }
        let (_, mut res, _) = m.exgcd(p);
        if res < IBig::zero() {
            res = &res + p;
        }
        res
    }

    /// jacobi symbol: judge existing modsqrtmod (1: exist, -1: not exist)
    fn jacobi(&self, q: &Self) -> i32 {
        if q.is_one() {
            return 1;
        }
        if self.is_zero() {
            return 0;
        }
        let e;
        if self % ibig!(2) == ibig!(0) {
            // e = (q * q - 1) / 8;
            e = (q * q - 1) / ibig!(8);
            if e % ibig!(2) == ibig!(0) {
                return (self / ibig!(2)).jacobi(q);
            } else {
                return -(self / ibig!(2)).jacobi(q);
            }
        }

        // e = (a - 1) / 2 * (q - 1) / 2;
        e = (self - 1) / ibig!(2) * (q - 1) / ibig!(2);
        if e % ibig!(2) == ibig!(0) {
            (q % self).jacobi(self)
        } else {
            -(q % self).jacobi(self)
        }
    }

    /// Tonelli's Algorithm
    #[allow(clippy::many_single_char_names)]
    fn mod_sqrt(a: &Self, p: &Self) -> Self {
        let one = ibig!(1);
        let p1 = p - 1i32;

        // b: some non-quadratic-redidue
        let mut b = ibig!(0);
        while b.is_zero() || b.jacobi(p) != -1 {
            b = Number::gen_rand(&one, &p1);
        }

        // p = t * 2 ^ s + 1, t is odd
        let mut t = p1;
        let mut s = IBig::zero();
        while &t & IBig::one() == IBig::zero() {
            t = &t >> 1;
            s = &s + 1i32;
        }

        // assert p == t * 2 ^ s + 1 and t % 2 == 1
        let ni = Self::mod_inv(a, p);
        let mut c = Self::mod_pow(&b, &t, p);
        let mut r = Self::mod_pow(&ni, &((&t + &one) / ibig!(2)), p);

        // for k in 1..s
        let mut k = ibig!(1);
        while k < s {
            let b = Self::mod_pow(&r, &ibig!(2), p) * &ni % p;
            let e = &Self::mod_pow(&ibig!(2), &(&s - &k - &one), p);
            let d = Self::mod_pow(&b, e, p);
            if d == p - &one {
                r = &r * &c % p;
            }
            c = &c * &c % p;
            k = &k + &one;
        }
        r // or (p - r)
    }

    fn to_hex(&self) -> String {
        format!("{:x}", self)
    }

    fn add_ref(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn sub_ref(&self, rhs: &Self) -> Self {
        self - rhs
    }

    fn mul_ref(&self, rhs: &Self) -> Self {
        self * rhs
    }

    fn gen_rand(min: &Self, max: &Self) -> Self {
        let mut rng = rand::thread_rng();
        rng.gen_range(min.clone()..max.clone())
    }

    fn bit_len(&self) -> usize {
        UBig::try_from(self).unwrap().bit_len()
    }

    fn test_bit(&self, bit: usize) -> bool {
        let ui = UBig::try_from(self).unwrap();
        ui.bit(bit)
    }

    fn from_bytes_be(bytes: &[u8]) -> Self {
        IBig::from(UBig::from_be_bytes(bytes))
    }

    fn to_bytes_be(&self) -> Vec<u8> {
        let ui = UBig::try_from(self).unwrap();
        ui.to_be_bytes()
    }

    fn from_bytes_radix(buf: &[u8], radix: u32) -> Self {
        let s = str::from_utf8(buf).ok().unwrap();
        IBig::from_str_radix(s, radix).unwrap()
    }
}

impl Number for BigInt {
    fn mod_cal(a: &Self, p: &Self) -> Self {
        let mut x = a % p;
        if x < BigInt::zero() {
            x += p;
        }
        x
    }

    fn pow(&self, exp: u32) -> Self {
        self.pow(exp)
    }

    fn mod_pow(a: &Self, e: &Self, p: &Self) -> Self {
        a.modpow(e, p)
    }

    fn exgcd(&self, b: &Self) -> (Self, Self, Self) {
        let (mut q, mut r, mut s, mut t);
        let (mut x, mut y) = (self.clone(), b.clone());
        let (mut s0, mut s1) = (BigInt::one(), BigInt::zero());
        let (mut t0, mut t1) = (BigInt::zero(), BigInt::one());
        while y > BigInt::zero() {
            q = &x / &y;
            r = &x % &y;
            s = &s0 - &q * &s1;
            t = &t0 - &q * &t1;
            x = y;
            y = r;
            s0 = s1;
            t0 = t1;
            s1 = s;
            t1 = t;
        }
        (x, s0, t0)
    }

    /// mod inverse n^(-1) mod p
    fn mod_inv(a: &Self, p: &Self) -> Self {
        // assert!(n ( modinv(n, p) == 1);
        let mut m = a.clone();

        if m < BigInt::zero() {
            m = &m * p;
        }
        let (_, mut res, _) = m.exgcd(p);
        if res < BigInt::zero() {
            res = &res + p;
        }
        res
    }

    /// jacobi symbol: judge existing modsqrtmod (1: exist, -1: not exist)
    fn jacobi(&self, q: &Self) -> i32 {
        if q.is_one() {
            return 1;
        }
        if self.is_zero() {
            return 0;
        }
        let e;
        if self % 2 == BigInt::zero() {
            e = (q * q - 1u32) / 8u32;
            if e % 2 == BigInt::zero() {
                return (self / 2u32).jacobi(q);
            } else {
                return (self / 2u32).jacobi(q);
            }
        }
        e = (self - 1u32) / 2u32 * (q - 1u32) / 2u32;
        if e % 2u32 == BigInt::zero() {
            (q % self).jacobi(self)
        } else {
            -(q % self).jacobi(self)
        }
    }

    /// Tonelli's Algorithm
    #[allow(clippy::many_single_char_names)]
    fn mod_sqrt(a: &Self, p: &Self) -> Self {
        let mut rng = rand::thread_rng();

        // b: some non-quadratic-residue
        let mut b = BigInt::zero();
        while b.is_zero() || b.jacobi(p) != -1 {
            b = rng.gen_bigint_range(&BigInt::one(), &(p - 1));
        }
        // p = t * 2^s + 1, t is odd
        let mut t = p * BigInt::one();
        let mut s = BigInt::zero();
        while &t & BigInt::one() == BigInt::zero() {
            t = &t >> 1u32;
            s = &s + 1u32;
        }
        // assert p == t * 2^s + 1 and t % 2 == 1
        let ni = Self::mod_inv(a, p);
        let mut c = b.modpow(&t, p);
        let mut r = a.modpow(&((&t + 1u32) / 2u32), p);

        // for k in 1..s
        let mut k = BigInt::one();
        while k < s {
            let b = r.modpow(&BigInt::from(2u32), p) * &ni % p;
            let e = &BigInt::from(2u32).modpow(&(&s - &k - 1u32), p);
            let d = b.modpow(e, p);
            if d == p - 1u32 {
                r = &r * &c % p;
            }
            c = &c * &c % p;
            k = &k + 1u32;
        }
        r // or (p - r)
    }

    fn to_hex(&self) -> String {
        format!("{:x}", self)
    }

    fn add_ref(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn sub_ref(&self, rhs: &Self) -> Self {
        self - rhs
    }

    fn mul_ref(&self, rhs: &Self) -> Self {
        self * rhs
    }

    fn gen_rand(min: &Self, max: &Self) -> Self {
        let mut rng = rand::thread_rng();
        rng.gen_bigint_range(min, max)
    }

    fn bit_len(&self) -> usize {
        self.bits() as usize
    }

    fn test_bit(&self, bit: usize) -> bool {
        self.bit(bit as u64)
    }

    fn from_bytes_be(bytes: &[u8]) -> Self {
        BigInt::from_bytes_be(Sign::Plus, bytes)
    }

    fn to_bytes_be(&self) -> Vec<u8> {
        self.to_bytes_be().1
    }

    fn from_bytes_radix(buf: &[u8], radix: u32) -> Self {
        BigInt::parse_bytes(buf, radix).unwrap()
    }
}
