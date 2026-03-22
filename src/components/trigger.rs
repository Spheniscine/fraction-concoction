use std::time::Duration;
use async_std::stream::StreamExt;
use web_time::Instant;

use dioxus::prelude::*;

pub type TriggerSignal<T> = Signal<Vec<TriggerEvent<T>>>;

#[derive(Clone)]
pub struct Trigger<T: 'static> {
    pub signal: Signal<Vec<TriggerEvent<T>>>,
    pub coroutine: Coroutine<()>,
}

pub struct TriggerEvent<T> {
    pub time: Instant,
    pub message: T,
}

pub fn use_trigger<T>(duration: Duration) -> Trigger<T> {
    let mut signal = use_signal(|| vec![]);
    let coroutine = use_coroutine(move |mut rx| async move {
        // This coroutine tries to cleanup the event list when `duration` has passed since the last event.
        while let Some(_) = rx.next().await {
            loop {
                let Some(remaining) = signal.read().last().map(|evt: &TriggerEvent<T>| {
                    duration.saturating_sub(evt.time.elapsed())
                }) else { break };
                if remaining.is_zero() {
                    signal.write().clear();
                } else {
                    async_std::task::sleep(remaining).await;
                }
            }
        }
    });
    Trigger {
        signal, coroutine
    }
}

impl <T: 'static> Trigger<T> {
    pub fn trigger(&mut self, message: T) {
        self.signal.write().push(TriggerEvent { 
            time: Instant::now(), message 
        });
        self.coroutine.send(());
    }
}