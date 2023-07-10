use std::{
    fmt::Display,
    ops::{Add, BitAnd, Div, Mul, Rem, Shr, Sub},
};

use num_traits::{One, Zero};

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
    fn from_bytes_radix(but: &[u8], radix: u32) -> Self; // b"..."
    fn from_bytes_be(bytes: &[u8]) -> Self;
    fn to_bytes_be(&self) -> Vec<u8>;
}
