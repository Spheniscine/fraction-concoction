use std::time::Duration;
use rand::RngCore;
use web_time::Instant;

use dioxus::prelude::*;

pub type Trigger<T> = Signal<Vec<TriggerEvent<T>>>;

pub struct TriggerEvent<T> {
    pub time: Instant,
    pub id: u64,
    pub message: T,
}

pub fn use_trigger<T>() -> Trigger<T> {
    use_signal(|| vec![])
}

#[extension(pub trait TriggerExt)]
impl <T> Trigger<T> {
    fn trigger(&mut self, message: T) {
        self.write().push(TriggerEvent {
            time: Instant::now(),
            id: rand::rng().next_u64(),
            message
        });
    }

    fn retain_recent(&mut self, duration: Duration) {
        self.write().retain(|ev| {
            ev.time.elapsed() <= duration
        });
    }
}