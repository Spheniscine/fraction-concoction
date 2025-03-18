use crate::utils::Fraction;
use super::Color;

#[derive(Clone, Copy, Debug)]
pub struct Beaker {
    amount: Fraction,
    fill: Option<Color>
}