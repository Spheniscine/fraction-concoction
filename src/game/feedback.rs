use dioxus::{document, prelude::*};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use strum_macros::{EnumCount, EnumIter};
use web_sys::HtmlAudioElement;

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
    fn get_audio_state(&self) -> f64;
    fn set_audio_state(&mut self, value: f64);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedbackImpl {
    #[serde(deserialize_with = "deserialize_audio_state")]
    pub audio_state: f64
}

fn deserialize_audio_state<'de, D>(deserializer: D) -> Result<f64, D::Error> where D: Deserializer<'de> {
    struct MyVisitor;
    impl<'de> Visitor<'de> for MyVisitor {
        type Value = f64;
    
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "value between 0.0 and 1.0, supports booleans for back-compatibility")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error, {
            Ok(v as u8 as f64)
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error, {
            Ok(v)
        }
    }

    deserializer.deserialize_any(MyVisitor)
}

impl Feedback for FeedbackImpl {
    fn play_audio(&self, audio: Audio) {
        HtmlAudioElement::new_with_src(&audio.asset().to_string())
            .map(|e| {
                e.set_volume(self.audio_state);
                e.play()
            });
    }
    
    fn get_audio_state(&self) -> f64 {
        self.audio_state
    }
    
    fn set_audio_state(&mut self, value: f64) {
        self.audio_state = value;
    }
}