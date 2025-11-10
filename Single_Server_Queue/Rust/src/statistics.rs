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
        self.last_event_time = time;
        self.last_queue_length = queue_length;
    }

    #[inline]
    pub fn record_service_start(&mut self, wait_time: f64) {
        self.total_wait_time += wait_time;
    }

    #[inline]
    pub fn record_service_end(&mut self, service_duration: f64) {
        self.served_customers += 1;
        self.total_busy_time += service_duration;
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
}
