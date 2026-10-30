pub mod platform;

use chrono::{DateTime, Utc};

pub trait Platform {
    fn listen(&self) -> impl Iterator<Item = Event>;
}

pub struct Event {
    pub timestamp: DateTime<Utc>,
    pub kind: EventKind,
}

impl Event {
    pub fn change() -> Self {
        Self {
            timestamp: Utc::now(),
            kind: EventKind::Change,
        }
    }
}

pub enum EventKind {
    Change,
}

pub struct Watcher<T> {
    platform: T,
    callback: Box<dyn Fn(&Event) + Send + 'static>,
}

impl<T: Platform> Watcher<T> {
    pub fn new<F>(callback: F, platform: T) -> Self
    where
        F: Fn(&Event) + Send + 'static,
        T: Platform,
    {
        Self {
            platform,
            callback: Box::new(callback),
        }
    }

    pub fn run(&self) {
        for event in self.platform.listen() {
            (self.callback)(&event)
        }
    }
}

pub fn watch(callback: impl Fn(&Event) + Send + 'static, platform: impl Platform) {
    let watcher = Watcher::new(callback, platform);
    watcher.run()
}
