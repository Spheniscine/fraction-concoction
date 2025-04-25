use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumCount, EnumIter, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

impl Difficulty {
    pub fn next_up(&self) -> Option<Self> {
        match self {
            Difficulty::Easy => Some(Difficulty::Medium),
            Difficulty::Medium => Some(Difficulty::Hard),
            Difficulty::Hard => None,
        }
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        };
        f.write_str(s)?;
        Ok(())
    }
}

impl FromStr for Difficulty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "easy" => Ok(Difficulty::Easy),
            "medium" => Ok(Difficulty::Medium),
            "hard" => Ok(Difficulty::Hard),
            _ => Err(())
        }
    }
}