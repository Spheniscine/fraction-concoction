use crate::utils::Fraction;
use super::Color;

#[derive(Clone, Copy, Debug)]
pub struct Dropper {
    capacity: Fraction,
    fill: Option<Color>
}