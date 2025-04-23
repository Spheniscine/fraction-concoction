use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumCount, EnumIter, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}