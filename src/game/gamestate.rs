use crate::utils::Fraction;

use super::{random_name, Audio, Beaker, Color, Difficulty, Dropper, Entity, Feedback, FeedbackImpl, Ingredient, Recipe, NUM_BEAKERS, NUM_DROPPERS, NUM_INGREDIENTS};

#[derive(Clone)]
pub struct GameState {
    pub difficulty: Difficulty,
    pub recipe: Recipe,
    pub beakers: [Option<Beaker>; NUM_BEAKERS],
    pub droppers: [Dropper; NUM_DROPPERS],
    pub selected: Option<Entity>,
    pub feedback: FeedbackImpl,
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
            // selected: Some(Entity::Beaker { index: 1 }),
            feedback: FeedbackImpl
        }
    }


    pub fn click_entity(&mut self, entity: Entity) {
        match self.selected {
            None => {
                match entity {
                    Entity::Dropper { index } => {
                        self.feedback.play_audio(Audio::Clink);
                        self.selected = Some(entity);
                    }
                    Entity::Beaker { index } => {
                        if self.beakers[index].is_some_and(|b| b.fill.is_some()) {
                            self.feedback.play_audio(Audio::Clink);
                            self.selected = Some(entity);
                        }
                    }
                    _ => {}
                }
            }

            Some(Entity::Dropper { index: dropper_index }) => {
                match entity {
                    Entity::Dispenser { color } => {
                        if self.droppers[dropper_index].fill.is_none() {
                            self.feedback.play_audio(Audio::Pour1);
                            self.droppers[dropper_index].fill = Some(color);
                        }
                    }
                    Entity::Beaker { index: beaker_index } => {
                        let Some(beaker) = self.beakers[beaker_index] else {return};
                        let dropper = self.droppers[dropper_index];
                        if let Some(color) = dropper.fill {
                            if beaker.fill.is_none_or(|c| c == color) {
                                self.feedback.play_audio(Audio::Pour2);
                                self.droppers[dropper_index].fill = None;
                                let beaker = self.beakers[beaker_index].as_mut().unwrap();
                                beaker.amount += dropper.capacity;
                                beaker.fill = Some(color);
                            } else {
                                self.feedback.play_audio(Audio::Error);
                            }
                        } else {
                            if let Some(color) = beaker.fill {
                                if beaker.amount >= dropper.capacity {
                                    self.feedback.play_audio(Audio::Pour1);
                                    self.droppers[dropper_index].fill = Some(color);
                                    let beaker = self.beakers[beaker_index].as_mut().unwrap();
                                    beaker.amount -= dropper.capacity;
                                    if beaker.amount == Fraction::zero() {
                                        beaker.fill = None;
                                    }
                                } else {
                                    self.feedback.play_audio(Audio::Error);
                                }
                            }
                        }
                    }
                    Entity::Trash => {
                        if self.droppers[dropper_index].fill.is_some() {
                            self.feedback.play_audio(Audio::Drain);
                            self.droppers[dropper_index].fill = None;
                        }
                    }
                    Entity::Dropper { index: other_index } => {
                        self.feedback.play_audio(Audio::Clink);
                        if dropper_index == other_index {
                            self.selected = None;
                        } else {
                            self.selected = Some(entity);
                        }
                    }
                    _ => {}
                }
            }

            Some(Entity::Beaker { index: beaker_index }) => {
                let Some(beaker) = self.beakers[beaker_index] else {
                    self.selected = None;
                    return
                };
                match entity {
                    Entity::Beaker { index: other_index } => {
                        self.feedback.play_audio(Audio::Clink);
                        if beaker_index == other_index {
                            self.selected = None;
                        } else {
                            if self.beakers[other_index].is_some_and(|b| b.fill.is_some()) {
                                self.selected = Some(entity);
                            }
                        }
                    }
                    // Entity::Dropper { index } => {
                    //     self.selected = Some(entity);
                    // }
                    Entity::Trash => {
                        if beaker.fill.is_some() {
                            self.feedback.play_audio(Audio::Drain);
                            let beaker = self.beakers[beaker_index].as_mut().unwrap();
                            beaker.amount = Fraction::zero();
                            beaker.fill = None;
                            self.selected = None;
                        }
                    }
                    Entity::Blender => {
                        if let Some(i) = (0..NUM_INGREDIENTS).find(|&i| {
                            let ingredient = self.recipe.ingredients[i];
                            !ingredient.done && ingredient.amount == beaker.amount && Some(ingredient.color) == beaker.fill
                        }) {
                            self.feedback.play_audio(Audio::Pour2);
                            self.recipe.ingredients[i].done = true;
                            self.beakers[beaker_index] = None;
                            self.selected = None;

                            if self.is_won() {
                                self.feedback.play_audio(Audio::Blend);
                            }
                        } else {
                            self.feedback.play_audio(Audio::Error);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn is_won(&self) -> bool {
        self.recipe.ingredients.iter().all(|i| i.done)
    }
}