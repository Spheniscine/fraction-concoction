use serde::{Deserialize, Serialize};

use super::Color;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Entity {
    Dispenser { color: Color },
    Blender,
    Beaker { index: usize },
    Dropper { index: usize },
    Trash
}