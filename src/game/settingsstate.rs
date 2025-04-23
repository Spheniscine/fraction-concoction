use serde::{Deserialize, Serialize};

use super::Difficulty;

#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub difficulty: Difficulty,
    pub audio_state: bool,
}