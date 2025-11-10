#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Arrival,
    Departure,
}

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub time: f64,
    pub event_type: EventType,
}

impl Event {
    #[inline]
    pub fn new(time: f64, event_type: EventType) -> Self {
        Self { time, event_type }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .time
            .partial_cmp(&self.time)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}
