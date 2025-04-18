use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumCount, EnumIter, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}