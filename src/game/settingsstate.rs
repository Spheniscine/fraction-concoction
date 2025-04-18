use super::Difficulty;

#[derive(Clone)]
pub struct SettingsState {
    pub difficulty: Difficulty,
    pub audio_state: bool,
}