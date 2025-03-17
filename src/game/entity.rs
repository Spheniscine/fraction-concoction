pub enum Entity {
    Dispenser { color: Color },
    Blender,
    Beaker { index: usize },
    Dropper { index: usize },
    Trash
}