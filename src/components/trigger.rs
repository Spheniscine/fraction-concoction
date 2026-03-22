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
        while let Some(_) = rx.next().await {
            if signal.read().is_empty() { continue; }
            async_std::task::sleep(duration).await;
            if signal.read().last().is_some_and(|evt: &TriggerEvent<T>| evt.time.elapsed() >= duration) {
                signal.write().clear();
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