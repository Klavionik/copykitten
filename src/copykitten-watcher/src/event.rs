use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    Change,
}
