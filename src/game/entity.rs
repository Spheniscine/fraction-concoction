use super::Color;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Entity {
    Dispenser { pub color: Color },
    Blender,
    Beaker { pub index: usize },
    Dropper { pub index: usize },
    Trash
}