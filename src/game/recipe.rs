use crate::utils::Fraction;
use super::{Color, NUM_INGREDIENTS};

#[derive(Clone, Debug)]
pub struct Recipe {
    pub index: usize,
    pub name: String,
    pub ingredients: [(Fraction, Color); NUM_INGREDIENTS],
}