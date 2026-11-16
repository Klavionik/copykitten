use crate::Event;
use std::fmt::Debug;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(windows)]
mod windows;
#[cfg(all(unix, not(target_os = "macos")))]
mod x11;

#[derive(Debug)]
pub enum Platform {
    #[cfg(windows)]
    Windows(windows::Windows),
    #[cfg(all(unix, not(target_os = "macos")))]
    X11(x11::X11),
    #[cfg(target_os = "macos")]
    MacOS(macos::MacOS),
}

impl Platform {
    #[cfg(all(unix, not(target_os = "macos")))]
    pub fn new() -> Result<Self, anyhow::Error> {
        Ok(Self::X11(x11::X11::new()))
    }
    #[cfg(windows)]
    pub fn new() -> Result<Self, anyhow::Error> {
        Ok(Self::Windows(windows::Windows::new()))
    }
    #[cfg(target_os = "macos")]
    pub fn new() -> Result<Self, anyhow::Error> {
        Ok(Self::MacOS(macos::MacOS::new()))
    }

    // TODO: Seems more like a private method.
    pub fn next_event(&mut self) -> Option<Event> {
        match self {
            #[cfg(all(unix, not(target_os = "macos")))]
            Self::X11(p) => p.next_event(),
            #[cfg(windows)]
            Self::Windows(p) => p.next_event(),
            #[cfg(target_os = "macos")]
            Self::MacOS(p) => p.next_event(),
        }
    }

    pub fn listen(&mut self) -> impl Iterator<Item = Event> + '_ {
        std::iter::from_fn(move || self.next_event())
    }

    pub fn setup(&mut self) -> Result<(), anyhow::Error> {
        match self {
            #[cfg(all(unix, not(target_os = "macos")))]
            Self::X11(_) => Ok(()),
            #[cfg(windows)]
            Self::Windows(p) => p.setup(),
            #[cfg(target_os = "macos")]
            Self::MacOS(_) => Ok(()),
        }
    }
}

trait Listener: Debug {
    fn next_event(&mut self) -> Option<Event>;
}
