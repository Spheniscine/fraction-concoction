use serde::{Deserialize, Serialize};

use super::Difficulty;

#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub difficulty: Difficulty,
    pub keep_dropper_selection: bool,
    pub audio_state: bool,
    pub adaptive_difficulty: bool,
    pub reset_level: bool,
}