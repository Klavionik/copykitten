pub mod event;
mod platform;

use event::Event;
use log::debug;
pub use platform::Platform;
use std::thread::{self, JoinHandle};

pub struct Watcher<F> {
    callback: F,
    platform: Platform,
}

// TODO: Potential option: ignore operations made by this process.
// TODO: Potential option: deduplicate by the sequence ID.
impl<F: FnMut(&Event)> Watcher<F> {
    pub fn new(callback: F, platform: Platform) -> Self {
        Self { callback, platform }
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
    {
        thread::Builder::new()
            .name("copykitten-watcher".into())
            .spawn(move || self.run())
            .expect("Failed to spawn clipboard watcher")
    }
}
