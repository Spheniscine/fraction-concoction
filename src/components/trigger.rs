use std::time::Duration;
use web_time::Instant;

use dioxus::prelude::*;

pub type Trigger<T> = Signal<Option<(Instant, T)>>;
pub type ReadTrigger<T> = ReadSignal<Option<(Instant, T)>, UnsyncStorage>;

pub fn use_trigger<T>() -> Trigger<T> {
    use_signal(|| None)
}

#[extension(pub trait TriggerExt)]
impl <T> Trigger<T> {
    fn trigger(&mut self, message: T) {
        *self.write() = Some((Instant::now(), message));
    }
}

#[extension(pub trait ReadTriggerExt)]
impl <T> ReadTrigger<T> {
    fn check<R>(&self, duration: Duration, action: impl FnOnce(&T) -> R) -> Option<R> {
        self.read().as_ref().and_then(|(time, message)| {
            if Instant::now() - *time <= duration {
                Some(action(message))
            } else {None}
        })
    }
}