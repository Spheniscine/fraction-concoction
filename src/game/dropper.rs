use serde::{Deserialize, Serialize};

use crate::utils::Fraction;
use super::Color;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Dropper {
    pub capacity: Fraction,
    pub fill: Option<Color>
}