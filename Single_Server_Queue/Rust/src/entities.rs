use crate::engine::SimulationEngine;
use crate::event::{Event, EventType};
use crate::statistics::Statistics;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Server {
    inv_mu: f64, // reciprocal of service rate mu (multiplication is faster than division)
    queue: VecDeque<f64>, // Queue of customer arrival times
    busy: bool,
    service_start_time: f64,
    stats: Rc<RefCell<Statistics>>,
}

impl Server {
    pub fn new(mu: f64, stats: Rc<RefCell<Statistics>>) -> Self {
        Self {
            inv_mu: 1.0 / mu,
            queue: VecDeque::new(),
            busy: false,
            service_start_time: 0.0,
            stats,
        }
    }

    #[inline]
    pub fn receive_customer(&mut self, engine: &mut SimulationEngine) {
        let now = engine.now();

        self.queue.push_back(now);

        self.stats
            .borrow_mut()
            .record_queue_change(now, self.queue.len());

        if !self.busy {
            self.start_service(engine);
        }
    }

    #[inline]
    fn start_service(&mut self, engine: &mut SimulationEngine) {
        if self.queue.is_empty() {
            return;
        }

        let now = engine.now();
        let arrival_time = self.queue.pop_front().unwrap();
        let wait_time = now - arrival_time;

        let mut stats = self.stats.borrow_mut();
        stats.record_queue_change(now, self.queue.len());
        stats.record_service_start(wait_time);
        drop(stats);

        self.busy = true;
        self.service_start_time = now;

        // Generate service time from exponential distribution
        // Using pre-computed reciprocal for faster multiplication
        let service_time = -fastrand::f64().ln() * self.inv_mu;

        engine.schedule(Event::new(now + service_time, EventType::Departure));
    }

    #[inline]
    pub fn handle_departure(&mut self, engine: &mut SimulationEngine) {
        let now = engine.now();
        let service_duration = now - self.service_start_time;

        self.busy = false;
        self.stats.borrow_mut().record_service_end(service_duration);

        if !self.queue.is_empty() {
            self.start_service(engine);
        }
    }
}

pub struct Client {
    inv_lambda: f64,
    server: Rc<RefCell<Server>>,
}

impl Client {
    pub fn new(lambda: f64, server: Rc<RefCell<Server>>) -> Self {
        Self {
            inv_lambda: 1.0 / lambda,
            server,
        }
    }

    #[inline]
    pub fn handle_generate(&mut self, engine: &mut SimulationEngine) {
        self.server.borrow_mut().receive_customer(engine);

        let inter_arrival_time = -fastrand::f64().ln() * self.inv_lambda;
        let next_time = engine.now() + inter_arrival_time;
        engine.schedule(Event::new(next_time, EventType::Arrival));
    }
}
