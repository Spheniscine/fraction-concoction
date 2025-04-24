use arrayvec::ArrayVec;
use rand::{rng, seq::{IndexedRandom, SliceRandom}, Rng};
use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator, VariantArray};

use crate::{components::LocalStorage, utils::Fraction};

use super::{random_name, Audio, Beaker, Color, Difficulty, Dropper, Entity, Feedback, FeedbackImpl, Ingredient, Recipe, SettingsState, NUM_BEAKERS, NUM_DROPPERS, NUM_INGREDIENTS, PRIME_DENOMS};

#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    pub difficulty: Difficulty,
    pub num_won_at_difficulty: usize,
    pub recipe: Recipe,
    pub beakers: [Option<Beaker>; NUM_BEAKERS],
    pub droppers: [Dropper; NUM_DROPPERS],
    pub selected: Option<Entity>,
    pub feedback: FeedbackImpl,
    pub keep_dropper_selection: bool,
    pub show_settings: bool,
}

impl GameState {
    pub fn generate(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.selected = None;
        self.recipe.name = random_name();
        self.beakers = [
            Some(Beaker { amount: Fraction::zero(), fill: None }); NUM_BEAKERS
        ];
        let mut rng = rng();
        match difficulty {
            Difficulty::Easy => {
                let prime1 = *PRIME_DENOMS.choose(&mut rng).unwrap();
                let prime2 = loop {
                    let x = *PRIME_DENOMS.choose(&mut rng).unwrap();
                    if x != prime1 { break x; }
                };

                let mut recipe_amounts = ArrayVec::<Fraction, NUM_INGREDIENTS>::new();
                let mut dropper_amounts = ArrayVec::<Fraction, NUM_DROPPERS>::new();

                for p in [prime1, prime2, prime2] {
                    let mut a = rng.random_range(1..p);
                    let mut b = rng.random_range(1..p);

                    if a > b { std::mem::swap(&mut a, &mut b); }

                    recipe_amounts.push(Fraction::new(b, p));
                    dropper_amounts.push(Fraction::new(a, p));
                    dropper_amounts.push(Fraction::new(b - a, p));
                }

                recipe_amounts.shuffle(&mut rng);
                dropper_amounts.shuffle(&mut rng);

                let mut colors = Color::iter().collect::<ArrayVec<Color, {Color::COUNT}>>();
                colors.partial_shuffle(&mut rng, NUM_INGREDIENTS);

                for i in 0..NUM_INGREDIENTS {
                    self.recipe.ingredients[i] = Ingredient {
                        amount: recipe_amounts[i],
                        color: colors[i],
                        done: false,
                    }
                }

                for i in 0..NUM_DROPPERS {
                    self.droppers[i] = Dropper { capacity: dropper_amounts[i], fill: None  }
                }
            },
            Difficulty::Medium => todo!(),
            Difficulty::Hard => todo!(),
        }
        LocalStorage.save_game_state(&self);
    }

    pub fn advance(&mut self) {
        if self.is_won() {
            self.num_won_at_difficulty += 1;
            self.recipe.index += 1;
            self.generate(self.difficulty);
        }
    } 

    /// generate fixed values for testing
    pub fn new_test() -> Self {
        Self {
            difficulty: Difficulty::Easy,
            num_won_at_difficulty: 0,
            recipe: Recipe { index: 0, name: String::from("Aqua Fortis"), ingredients: [
                Ingredient {
                    amount: Fraction::new(1, 2),
                    color: Color::Green,
                    done: false,
                },
                Ingredient {
                    amount: Fraction::new(7, 11),
                    color: Color::Blue,
                    done: false,
                },
                Ingredient {
                    amount: Fraction::new(5, 7),
                    color: Color::Yellow,
                    done: false,
                },
            ] },
            beakers: [
                Some(Beaker { amount: Fraction::zero(), fill: None }); NUM_BEAKERS
            ],
            droppers: [
                Dropper {
                    capacity: Fraction::new(1, 2),
                    fill: None,
                },
                Dropper {
                    capacity: Fraction::new(1, 11),
                    fill: None,
                },
                Dropper {
                    capacity: Fraction::new(2, 11),
                    fill: None,
                },
                Dropper {
                    capacity: Fraction::new(1, 7),
                    fill: None,
                },
                Dropper {
                    capacity: Fraction::new(6, 7),
                    fill: None,
                },
                Dropper {
                    capacity: Fraction::new(4, 11),
                    fill: None,
                },
            ],
            selected: None, 
            // selected: Some(Entity::Beaker { index: 1 }),
            feedback: FeedbackImpl { audio_state: true },
            keep_dropper_selection: false,
            show_settings: false,
        }
    }


    pub fn click_entity(&mut self, entity: Entity) {
        (||{
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
                                    if !self.keep_dropper_selection { self.selected = None; }
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
                                if !self.keep_dropper_selection { self.selected = None; }
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
                            if beaker_index == other_index {
                                self.feedback.play_audio(Audio::Clink);
                                self.selected = None;
                            } else {
                                if self.beakers[other_index].is_some_and(|b| b.fill.is_some()) {
                                    self.feedback.play_audio(Audio::Clink);
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
        })();
        LocalStorage.save_game_state(&self);
    }

    pub fn is_won(&self) -> bool {
        self.recipe.ingredients.iter().all(|i| i.done)
    }

    pub fn new_settings_state(&self) -> SettingsState {
        SettingsState {
            difficulty: self.difficulty,
            keep_dropper_selection: self.keep_dropper_selection,
            audio_state: self.feedback.get_audio_state(),
        }
    }

    pub fn apply_settings(&mut self, settings: &SettingsState) {
        // todo: apply difficulty
        self.keep_dropper_selection = settings.keep_dropper_selection;
        self.feedback.set_audio_state(settings.audio_state);
        LocalStorage.save_game_state(&self);
    }
}