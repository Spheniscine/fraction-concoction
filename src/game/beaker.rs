use crate::utils::Fraction;
use super::Color;

#[derive(Clone, Copy, Debug)]
pub struct Beaker {
    pub amount: Fraction,
    pub fill: Option<Color>
}