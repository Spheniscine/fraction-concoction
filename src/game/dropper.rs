use crate::utils::Fraction;
use super::Color;

#[derive(Clone, Copy, Debug)]
pub struct Dropper {
    pub capacity: Fraction,
    pub fill: Option<Color>
}