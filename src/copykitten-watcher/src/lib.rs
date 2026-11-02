pub mod platform;

use chrono::{DateTime, Utc};
use log::debug;
use std::fmt::Debug;
use std::thread::{self, JoinHandle};

pub trait Platform: Debug {
    fn listen(&self) -> impl Iterator<Item = Event>;
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum EventKind {
    Change,
}

pub struct Watcher<F, P> {
    platform: P,
    callback: F,
}

impl<F: FnMut(&Event), P: Platform> Watcher<F, P> {
    pub fn new(callback: F, platform: P) -> Self {
        Self { platform, callback }
    }

    pub fn run(&mut self) {
        debug!("Run clipboard watcher using {:?}", self.platform);

        for event in self.platform.listen() {
            (self.callback)(&event)
        }

        debug!("Stop clipboard watcher");
    }

    pub fn spawn(mut self) -> JoinHandle<()>
    where
        F: FnMut(&Event) + Send + 'static,
        P: Platform + Send + 'static,
    {
        thread::Builder::new()
            .name("copykitten-watcher".into())
            .spawn(move || self.run())
            .expect("Failed to spawn clipboard watcher")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct FakePlatform {
        events: RefCell<Vec<Event>>,
    }

    impl Platform for FakePlatform {
        fn listen(&self) -> impl Iterator<Item = Event> {
            std::iter::from_fn(move || self.events.borrow_mut().pop())
        }
    }

    fn fake_platform(fake_events: &[Event]) -> FakePlatform {
        FakePlatform {
            events: RefCell::new(fake_events.to_vec()),
        }
    }

    #[test]
    fn invokes_callback_on_received_events() {
        let first = Event::change();
        let second = Event::change();

        let mut expected_events = vec![second, first];
        let mut inbox = vec![];
        let platform = fake_platform(&expected_events);

        let cb = |e: &Event| inbox.push(e.timestamp);

        Watcher::new(cb, platform).run();

        assert_eq!(inbox.len(), 2);

        for ts in inbox {
            assert_eq!(expected_events.pop().unwrap().timestamp, ts)
        }
    }
}
