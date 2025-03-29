use crate::utils::Fraction;

use super::{random_name, Beaker, Difficulty, Dropper, Entity, Ingredient, Recipe, Color, NUM_BEAKERS, NUM_DROPPERS};

#[derive(Debug, Clone)]
pub struct GameState {
    difficulty: Difficulty,
    recipe: Recipe,
    beakers: [Option<Beaker>; NUM_BEAKERS],
    droppers: [Dropper; NUM_DROPPERS],
    selected: Option<Entity>,
}

impl GameState {
    /// temporary, generate fixed values for testing
    pub fn new_test() -> Self {
        Self {
            difficulty: Difficulty::Easy,
            recipe: Recipe { index: 0, name: random_name(), ingredients: [
                Ingredient {
                    amount: Fraction::new(12, 23),
                    color: Color::Cyan,
                    done: false,
                },
                Ingredient {
                    amount: Fraction::new(8, 11),
                    color: Color::Red,
                    done: false,
                },
                Ingredient {
                    amount: Fraction::new(11, 23),
                    color: Color::Yellow,
                    done: false,
                },
            ] },
            beakers: [
                Some(Beaker { amount: Fraction::zero(), fill: None }); 3
            ],
            droppers: [
                Dropper {
                    capacity: Fraction::new(2, 23),
                    fill: None,
                },
                Dropper {
                    capacity: Fraction::new(9, 23),
                    fill: None,
                },
                Dropper {
                    capacity: Fraction::new(2, 23),
                    fill: None,
                },
                Dropper {
                    capacity: Fraction::new(10, 23),
                    fill: None,
                },
                Dropper {
                    capacity: Fraction::new(6, 11),
                    fill: None,
                },
                Dropper {
                    capacity: Fraction::new(2, 11),
                    fill: None,
                },
            ],
            selected: None,
        }
    }
}