#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Entity {
    Dispenser { color: Color },
    Blender,
    Beaker { index: usize },
    Dropper { index: usize },
    Trash
}