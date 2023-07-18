//!
//! Elliptic Curve Calculation Library
//!
pub mod ec_param;
pub mod ecc_a;
pub mod ecc_j;
pub mod number;
pub mod ecc;

const A: &[u8; 64] = b"0000000000000000000000000000000000000000000000000000000000000000";
const B: &[u8; 64] = b"0000000000000000000000000000000000000000000000000000000000000007";
const N: &[u8; 64] = b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

pub struct S256Field(FieldElement);

pub struct S256Point(Point);
