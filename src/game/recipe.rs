use serde::{Deserialize, Serialize};

use crate::utils::Fraction;
use super::{Color, NUM_INGREDIENTS};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub amount: Fraction,
    pub color: Color,
    pub done: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub index: usize,
    pub name: String,
    pub ingredients: [Ingredient; NUM_INGREDIENTS],
}