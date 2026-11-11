pub mod event;
mod platform;

use event::Event;
use log::debug;
pub use platform::Platform;
use std::thread;
use std::thread::JoinHandle;

pub struct Watcher {
    callback: Box<dyn FnMut(&Event) + Send + 'static>,
    platform: Platform,
}

// TODO: Potential option: ignore operations made by this process.
// TODO: Potential option: deduplicate by the sequence ID.
impl Watcher {
    pub fn new(callback: impl FnMut(&Event) + Send + 'static, platform: Platform) -> Self {
        Self {
            callback: Box::new(callback),
            platform,
        }
    }
    pub fn run(&mut self) {
        // TODO: Handle error path.
        self.platform.setup().unwrap();
        debug!("Run clipboard watcher using {:?}", self.platform);

        for event in self.platform.listen() {
            (self.callback)(&event)
        }

        debug!("Stop clipboard watcher");
    }

    pub fn spawn(mut self) -> Result<JoinHandle<()>, anyhow::Error> {
        Ok(thread::Builder::new()
            .name("copykitten-watcher".to_string())
            .spawn(move || {
                self.run();
            })?)
    }
}

pub fn spawn(cb: impl FnMut(&Event) + Send + 'static) -> Result<JoinHandle<()>, anyhow::Error> {
    let platform = Platform::new()?;
    let watcher = Watcher::new(cb, platform);

    watcher.spawn()
}
