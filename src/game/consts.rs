use super::Color;
use strum::EnumCount;

pub const NUM_COLORS: usize = Color::COUNT;
pub const NUM_INGREDIENTS: usize = 3;
pub const NUM_BEAKERS: usize = NUM_INGREDIENTS;
pub const NUM_DROPPERS: usize = 6;

pub const ADAPTIVE_ADVANCE_SCORE: usize = 3;

pub const NEUTRAL_HTML_COLOR: &str = "#ccc";
pub const NEUTRAL_CONTRAST_COLOR: &str = "#000";

pub const PRIME_DENOMS: &[i64] = &[5, 7, 11, 13, 17, 19, 23, 29];