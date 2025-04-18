use crate::utils::Fraction;
use super::{Color, NUM_INGREDIENTS};

#[derive(Clone, Copy, Debug)]
pub struct Ingredient {
    pub amount: Fraction,
    pub color: Color,
    pub done: bool
}

#[derive(Clone, Debug)]
pub struct Recipe {
    pub index: usize,
    pub name: String,
    pub ingredients: [Ingredient; NUM_INGREDIENTS],
}