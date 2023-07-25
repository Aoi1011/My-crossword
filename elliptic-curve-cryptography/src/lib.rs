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

#[derive(Debug, Clone)]
pub struct S256Point(Point);

impl S256Point {}
