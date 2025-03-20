use crate::utils::Fraction;
use super::{Color, NUM_INGREDIENTS};

#[derive(Clone, Copy, Debug)]
pub struct Ingredient {
    amount: Fraction,
    color: Color,
    done: bool
}

#[derive(Clone, Debug)]
pub struct Recipe {
    pub index: usize,
    pub name: String,
    pub ingredients: [Ingredient; NUM_INGREDIENTS],
}