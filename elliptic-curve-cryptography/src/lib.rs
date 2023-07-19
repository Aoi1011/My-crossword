//!
//! Elliptic Curve Cryptography
//!

use std::str;

use elliptic_curves::Point;
use finite_fields::FieldElement;
use ibig::IBig;
use num_traits::Num;

pub mod ec_param;
pub mod ecc;
pub mod ecc_a;
pub mod ecc_j;
pub mod number;

const A: &[u8; 64] = b"0000000000000000000000000000000000000000000000000000000000000000";
const B: &[u8; 64] = b"0000000000000000000000000000000000000000000000000000000000000007";
const N: &[u8; 64] = b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

#[derive(Debug, Clone)]
pub struct S256Point(Point);

impl S256Point {
}
