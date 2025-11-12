pub struct Statistics {
    /// Sum of all customer wait times
    total_wait_time: f64,

    /// Number of customers who have been served
    served_customers: u64,

    /// Total time the server has been busy
    total_busy_time: f64,

    /// Timestamp of the last queue length change
    last_event_time: f64,

    /// Area under the queue length curve (for average calculation)
    area_under_q: f64,

    /// Last recorded queue length
    last_queue_length: usize,

    /// Current server busy state (0 or 1)
    server_busy: bool,

    /// Area under the customers-in-system curve
    area_under_customers: f64,

    /// Last recorded customers in system
    last_customers_in_system: usize,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            total_wait_time: 0.0,
            served_customers: 0,
            total_busy_time: 0.0,
            last_event_time: 0.0,
            area_under_q: 0.0,
            last_queue_length: 0,
            server_busy: false,
            area_under_customers: 0.0,
            last_customers_in_system: 0,
        }
    }

    /// Record a change in queue length
    ///
    /// This updates the "area under the curve" for queue length.
    /// By tracking the integral of queue length over time, we can compute
    /// the time-weighted average queue length.
    #[inline]
    pub fn record_queue_change(&mut self, time: f64, queue_length: usize) {
        let time_delta = time - self.last_event_time;
        self.area_under_q += self.last_queue_length as f64 * time_delta;
        self.area_under_customers += self.last_customers_in_system as f64 * time_delta;

        self.last_event_time = time;
        self.last_queue_length = queue_length;

        // Update customers in system (queue + server if busy)
        self.last_customers_in_system = queue_length + if self.server_busy { 1 } else { 0 };
    }

    #[inline]
    pub fn record_service_start(&mut self, time: f64, wait_time: f64) {
        // Update areas before changing state
        let time_delta = time - self.last_event_time;
        self.area_under_q += self.last_queue_length as f64 * time_delta;
        self.area_under_customers += self.last_customers_in_system as f64 * time_delta;

        self.total_wait_time += wait_time;
        self.server_busy = true;
        self.last_event_time = time;

        // Update last_customers_in_system since server became busy
        self.last_customers_in_system = self.last_queue_length + 1;
    }

    #[inline]
    pub fn record_service_end(&mut self, time: f64, service_duration: f64) {
        // Update areas before changing state
        let time_delta = time - self.last_event_time;
        self.area_under_q += self.last_queue_length as f64 * time_delta;
        self.area_under_customers += self.last_customers_in_system as f64 * time_delta;

        self.served_customers += 1;
        self.total_busy_time += service_duration;
        self.server_busy = false;
        self.last_event_time = time;

        // Update last_customers_in_system since server became idle
        self.last_customers_in_system = self.last_queue_length;
    }

    pub fn average_wait_time(&self) -> f64 {
        if self.served_customers == 0 {
            0.0
        } else {
            self.total_wait_time / self.served_customers as f64
        }
    }

    pub fn average_queue_length(&self, total_time: f64) -> f64 {
        if total_time == 0.0 {
            0.0
        } else {
            self.area_under_q / total_time
        }
    }

    pub fn utilization(&self, total_time: f64) -> f64 {
        if total_time == 0.0 {
            0.0
        } else {
            self.total_busy_time / total_time
        }
    }

    pub fn served_customers(&self) -> u64 {
        self.served_customers
    }

    pub fn current_queue_length(&self) -> usize {
        self.last_queue_length
    }

    pub fn instantaneous_utilization(&self, current_time: f64) -> f64 {
        if current_time == 0.0 {
            0.0
        } else {
            self.total_busy_time / current_time
        }
    }

    pub fn current_customers_in_system(&self) -> usize {
        self.last_customers_in_system
    }

    pub fn average_customers_in_system(&self, total_time: f64) -> f64 {
        if total_time == 0.0 {
            0.0
        } else {
            self.area_under_customers / total_time
        }
    }

    pub fn throughput(&self, total_time: f64) -> f64 {
        if total_time == 0.0 {
            0.0
        } else {
            self.served_customers as f64 / total_time
        }
    }
}
