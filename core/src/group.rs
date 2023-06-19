use crate::field::{Field, FieldStorage};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
// A group element of the secp256k1 curve, in affine coordinates.
pub struct Affine {
    pub x: Field,
    pub y: Field,
    pub infinity: bool,
}

#[derive(Debug, Clone, Copy)]
// A group of element of the secp256k1 curve, in jacobian coordinates.
pub struct Jacobian {
    pub x: Field,
    pub y: Field,
    pub z: Field,
    pub infinity: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
// Affine coordinate group element compact storage.
pub struct AffineStorage {
    pub x: FieldStorage,
    pub y: FieldStorage,
}

impl Default for Affine {
    fn default() -> Self {
        Self {
            x: Field::default(),
            y: Field::default(),
            infinity: false,
        }
    }
}

impl Default for Jacobian {
    fn default() -> Self {
        Self {
            x: Field::default(),
            y: Field::default(),
            z: Field::default(),
            infinity: false,
        }
    }
}

impl Default for AffineStorage {
    fn default() -> Self {
        Self {
            x: FieldStorage::default(),
            y: FieldStorage::default(),
        }
    }
}
