use serde::{Deserialize, Serialize};

use crate::utils::Fraction;
use super::Color;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Beaker {
    pub amount: Fraction,
    pub fill: Option<Color>
}