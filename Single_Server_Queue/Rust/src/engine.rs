use crate::event::Event;

pub struct SimulationEngine {
    next_arrival: Option<Event>,
    next_departure: Option<Event>,
    now: f64,
}

impl SimulationEngine {
    pub fn new() -> Self {
        Self {
            next_arrival: None,
            next_departure: None,
            now: 0.0,
        }
    }

    #[inline]
    pub fn schedule(&mut self, event: Event) {
        match event.event_type {
            crate::event::EventType::Arrival => {
                self.next_arrival = Some(event);
            }
            crate::event::EventType::Departure => {
                self.next_departure = Some(event);
            }
        }
    }

    #[inline]
    pub fn now(&self) -> f64 {
        self.now
    }

    #[inline]
    pub fn has_next_event(&self) -> bool {
        self.next_arrival.is_some() || self.next_departure.is_some()
    }

    #[inline]
    pub fn peek_next_time(&self) -> f64 {
        match (&self.next_arrival, &self.next_departure) {
            (Some(arr), Some(dep)) => arr.time.min(dep.time),
            (Some(arr), None) => arr.time,
            (None, Some(dep)) => dep.time,
            (None, None) => f64::INFINITY,
        }
    }

    /// Process a single event
    ///
    /// This returns the event so the caller can dispatch it to the right entity.
    /// This design keeps the engine decoupled from entity logic.
    #[inline]
    pub fn run_step(&mut self) -> Option<Event> {
        // Find which event happens next
        let event = match (&self.next_arrival, &self.next_departure) {
            (Some(arr), Some(dep)) => {
                if arr.time <= dep.time {
                    self.next_arrival.take()
                } else {
                    self.next_departure.take()
                }
            }
            (Some(_arr), None) => self.next_arrival.take(),
            (None, Some(_dep)) => self.next_departure.take(),
            (None, None) => None,
        };

        if let Some(ref e) = event {
            self.now = e.time;
        }

        event
    }

    #[inline]
    pub fn queue_size(&self) -> usize {
        let mut count = 0;
        if self.next_arrival.is_some() {
            count += 1;
        }
        if self.next_departure.is_some() {
            count += 1;
        }
        count
    }
}
