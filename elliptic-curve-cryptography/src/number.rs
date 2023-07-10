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
}
