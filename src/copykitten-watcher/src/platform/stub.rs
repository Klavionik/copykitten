use crate::{Event, Platform};

pub struct Stub;

impl Platform for Stub {
    fn listen(&self) -> impl Iterator<Item = Event> {
        std::iter::from_fn(|| Some(Event::change()))
    }
}
