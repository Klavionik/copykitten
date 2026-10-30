pub mod platform;

use log::debug;
use std::fmt::Debug;

use chrono::{DateTime, Utc};

pub trait Platform: Debug {
    fn listen(&self) -> impl Iterator<Item = Event>;
}

#[derive(Debug)]
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

#[derive(Debug)]
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
        debug!("Run clipboard watcher using {:?}", self.platform);

        for event in self.platform.listen() {
            (self.callback)(&event)
        }

        debug!("Stop clipboard watcher");
    }
}

pub fn watch(callback: impl Fn(&Event) + Send + 'static, platform: impl Platform) {
    Watcher::new(callback, platform).run()
}
