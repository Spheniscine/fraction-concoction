use super::{Beaker, Difficulty, Dropper, Entity, Recipe, NUM_BEAKERS, NUM_DROPPERS};

#[derive(Debug, Clone)]
pub struct GameState {
    difficulty: Difficulty,
    recipe: Recipe,
    beakers: [Option<Beaker>; NUM_BEAKERS],
    droppers: [Dropper; NUM_DROPPERS],
    selected: Option<Entity>,
}