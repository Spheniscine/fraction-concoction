use dioxus::{document, prelude::*};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumCount, EnumIter, Hash, Eq, PartialEq)]
pub enum Audio {
    Clink, Pour1, Pour2, Drain, Blend, Error
}

impl Audio {
    pub fn asset(self) -> Asset {
        match self {
            Audio::Clink => asset!("/assets/audio/clink.ogg"),
            Audio::Pour1 => asset!("/assets/audio/pour1.ogg"),
            Audio::Pour2 => asset!("/assets/audio/pour2.ogg"),
            Audio::Drain => asset!("/assets/audio/drain.ogg"),
            Audio::Blend => asset!("/assets/audio/blend.ogg"),
            Audio::Error => asset!("/assets/audio/error.ogg"),
        }
    }
}

// for feedback with a time duration, e.g. audio, animation

pub trait Feedback {
    fn play_audio(&self, audio: Audio);
    fn get_audio_state(&self) -> bool;
    fn set_audio_state(&mut self, value: bool);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedbackImpl {
    pub audio_state: bool
}

impl Feedback for FeedbackImpl {
    fn play_audio(&self, audio: Audio) {
        if self.get_audio_state() {
            let script = format!("new Audio('{}').play();", audio.asset());
            document::eval(&script);
        }
    }
    
    fn get_audio_state(&self) -> bool {
        self.audio_state
    }
    
    fn set_audio_state(&mut self, value: bool) {
        self.audio_state = value;
    }
}