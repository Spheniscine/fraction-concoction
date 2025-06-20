use arrayvec::ArrayVec;
use rand::{rng, seq::{IndexedRandom, SliceRandom}, Rng};
use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator, VariantArray};

use crate::{components::LocalStorage, utils::{CommonNumExt, Fraction}};

use super::{random_name, Audio, Beaker, Color, Difficulty, Dropper, Entity, Feedback, FeedbackImpl, Ingredient, Recipe, SettingsState, ADAPTIVE_ADVANCE_SCORE, NUM_BEAKERS, NUM_COLORS, NUM_DROPPERS, NUM_INGREDIENTS, PRIME_DENOMS};

fn yes() -> bool {
    true
}

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

    #[serde(default = "yes")]
    pub adaptive_difficulty: bool,
}

impl GameState {
    pub fn generate(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.selected = None;
        self.recipe.name = random_name();
        self.beakers = [
            Some(Beaker { amount: Fraction::zero(), fill: None }); NUM_BEAKERS
        ];
        let rng = &mut rng();
        match difficulty {
            Difficulty::Easy => {
                let prime1 = *PRIME_DENOMS.choose(rng).unwrap();
                let prime2 = loop {
                    let x = *PRIME_DENOMS.choose(rng).unwrap();
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

                self.generate_from_amounts(rng, recipe_amounts.into_inner().unwrap(), dropper_amounts.into_inner().unwrap());
            },
            Difficulty::Medium => {
                let mut recipe_amounts = ArrayVec::<Fraction, NUM_INGREDIENTS>::new();
                let mut dropper_amounts = ArrayVec::<Fraction, NUM_DROPPERS>::new();

                let numer = rng.random_range(1..=2);
                let denom = rng.random_range(4..=29);
                let whole = rng.random_range(1..=3);
                recipe_amounts.push(Fraction::new(whole, 1) + Fraction::new(numer, denom));
                dropper_amounts.push(Fraction::one());
                dropper_amounts.push(Fraction::new(1, denom));

                for it in 0..2 {
                    loop {
                        let denom = rng.random_range(6..=29);
                        let divisors = (1..denom).filter(|&i| denom % i == 0).collect::<Vec<_>>();
                        let small = *divisors.choose(rng).unwrap();

                        let big_candidates = (small + 1 .. denom).filter(|&i| i.gcd(denom) == 1).collect::<Vec<_>>();
                        let big = *big_candidates.choose(rng).unwrap();

                        let small_frac = Fraction::new(small, denom);
                        let big_frac = Fraction::new(big, denom);
                        let diff = big_frac - small_frac;
                        if diff.denominator() != denom { continue; }

                        if it == 0 {
                            recipe_amounts.push(big_frac);
                            dropper_amounts.push(small_frac);
                            dropper_amounts.push(diff);
                        } else {
                            recipe_amounts.push(diff);
                            dropper_amounts.push(small_frac);
                            dropper_amounts.push(big_frac);
                        }
                        break;
                    }
                }

                self.generate_from_amounts(rng, recipe_amounts.into_inner().unwrap(), dropper_amounts.into_inner().unwrap());
            },
            Difficulty::Hard => {
                let mut recipe_amounts = ArrayVec::<Fraction, NUM_INGREDIENTS>::new();
                let mut dropper_amounts = ArrayVec::<Fraction, NUM_DROPPERS>::new();

                let denom = rng.random_range(4..=29);
                let mut small = rng.random_range(1..denom);
                let mut big = loop {
                    let x = rng.random_range(1..denom);
                    if x != small { break x; }
                };
                if small > big { std::mem::swap(&mut small, &mut big); }
                let small_frac = Fraction::new(small, denom);
                let big_frac = Fraction::new(big, denom);

                let frac = if rng.random() { 
                    dropper_amounts.push(small_frac);
                    dropper_amounts.push(big_frac - small_frac);
                    big_frac
                } else {
                    dropper_amounts.push(small_frac);
                    dropper_amounts.push(big_frac);
                    big_frac - small_frac
                };

                let whole = rng.random_range(1..=3);
                dropper_amounts.push(Fraction::new(whole, 1));
                recipe_amounts.push(Fraction::new(whole * rng.random_range(1..=3), 1) + frac);
                // three droppers and one ingredient accounted for by now

                let denom = rng.random_range(6..=30);
                let mut candidates = (1..denom).collect::<Vec<_>>();
                let nums = candidates.partial_shuffle(rng, NUM_DROPPERS - 3).0;

                let small_i = (0..nums.len()).min_by_key(|&i| nums[i]).unwrap();
                nums.swap(small_i, 0);

                recipe_amounts.push(Fraction::new(nums[1], denom));
                recipe_amounts.push(Fraction::new(nums[2] - nums[0], denom));
                dropper_amounts.push(Fraction::new(nums[0], denom));
                dropper_amounts.push(Fraction::new(nums[1] - nums[0], denom));
                dropper_amounts.push(Fraction::new(nums[2], denom));

                self.generate_from_amounts(rng, recipe_amounts.into_inner().unwrap(), dropper_amounts.into_inner().unwrap());
            },

            Difficulty::Insane => {
                let mut recipe_amounts = ArrayVec::<Fraction, NUM_INGREDIENTS>::new();
                let mut dropper_amounts = ArrayVec::<Fraction, NUM_DROPPERS>::new();

                for t in 1..=2 {
                    let denom = rng.random_range(6..=30);

                    let mut candidates = (1..denom).collect::<Vec<_>>();
                    let nums: &[i64] = candidates.partial_shuffle(rng, 3).0;

                    for &num in nums {
                        dropper_amounts.push(Fraction::new(num, denom));
                    }

                    for _ in 0..t {
                        loop {
                            let mut sum = 0i64;
                            for &num in nums {
                                sum += num * rng.random_range(-3 ..= 3);
                            }
                            if sum == 0 { continue; }
                            recipe_amounts.push(Fraction::new(sum.abs(), denom));
                            break;
                        }
                    }
                }

                self.generate_from_amounts(rng, recipe_amounts.into_inner().unwrap(), dropper_amounts.into_inner().unwrap());
            },
        }
        LocalStorage.save_game_state(&self);
    }

    fn generate_from_amounts(&mut self, rng: &mut impl Rng, 
        mut recipe_amounts: [Fraction; NUM_INGREDIENTS], 
        mut dropper_amounts: [Fraction; NUM_DROPPERS]) 
    {
        recipe_amounts.shuffle(rng);
        dropper_amounts.shuffle(rng);

        let mut colors: [Color; NUM_COLORS] = Color::VARIANTS.try_into().unwrap();
        colors.partial_shuffle(rng, NUM_INGREDIENTS);

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
    }

    pub fn advance(&mut self) {
        if self.is_won() {
            self.num_won_at_difficulty += 1;
            if self.adaptive_difficulty && self.num_won_at_difficulty >= ADAPTIVE_ADVANCE_SCORE {
                if let Some(d) = self.difficulty.next_up() {
                    self.difficulty = d;
                    self.num_won_at_difficulty = 0;
                }
            }
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
            feedback: FeedbackImpl { audio_state: 1., prev_audio_state: 1. },
            keep_dropper_selection: false,
            show_settings: false,
            adaptive_difficulty: true,
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

    pub fn toggle_audio(&mut self) {
        self.feedback.toggle_audio();
        LocalStorage.save_game_state(&self);
    }

    pub fn is_won(&self) -> bool {
        self.recipe.ingredients.iter().all(|i| i.done)
    }

    pub fn do_show_settings(&mut self) {
        self.advance(); // make sure not stuck at won state
        self.show_settings = true;
    }

    pub fn new_settings_state(&self) -> SettingsState {
        SettingsState {
            difficulty: self.difficulty,
            keep_dropper_selection: self.keep_dropper_selection,
            audio_state: (self.feedback.get_audio_state() * 100.).round() as i32,
            adaptive_difficulty: self.adaptive_difficulty,
            reset_level: false,
        }
    }

    pub fn apply_settings(&mut self, settings: &SettingsState) {
        self.keep_dropper_selection = settings.keep_dropper_selection;
        self.feedback.set_audio_state(settings.audio_state as f64 / 100.);
        self.adaptive_difficulty = settings.adaptive_difficulty;

        if self.difficulty != settings.difficulty || settings.reset_level {
            self.difficulty = settings.difficulty;
            self.num_won_at_difficulty = 0;
            self.generate(self.difficulty);
        }
        LocalStorage.save_game_state(&self);
    }
}