use crate::field::Field;

pub struct Affine {
    pub x: Field,
    pub y: Field,
    pub infinity: bool,
}
