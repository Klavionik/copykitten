use crate::event::Event;
use crate::platform::Listener;
use log::debug;
use objc2::ffi::NSInteger;
use objc2_app_kit::NSPasteboard;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct MacOS {
    last_change_count: Option<NSInteger>,
}

impl MacOS {
    pub fn new() -> Self {
        Self {
            last_change_count: None,
        }
    }
}

impl Listener for MacOS {
    fn next_event(&mut self) -> Option<Event> {
        let pasteboard = NSPasteboard::generalPasteboard();
        debug!("Acquired pasteboard");
        debug!("Last change count: {:?}", self.last_change_count);

        loop {
            let change_count = pasteboard.changeCount();
            debug!("Fetched change count {change_count}");

            if let Some(last_change_count) = self.last_change_count {
                if change_count != last_change_count {
                    self.last_change_count = Some(change_count);

                    debug!("Updated change count from {last_change_count} to {change_count}");
                    debug!("There was a change, emit an event");
                    return Some(Event::change());
                }
            } else {
                debug!("No previous count, store {change_count}");
                self.last_change_count = Some(change_count)
            }

            debug!("Sleep...");
            sleep(Duration::from_millis(250));
        }
    }
}
