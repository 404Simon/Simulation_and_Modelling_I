#[derive(Debug, Clone)]
pub struct TimeSeries<T> {
    data: Vec<(f64, T)>, // (time, value)
    sample_interval: f64,
    next_sample_time: f64,
}

impl<T: Clone> TimeSeries<T> {
    pub fn new(sample_interval: f64, max_samples: usize) -> Self {
        Self {
            data: Vec::with_capacity(max_samples),
            sample_interval,
            next_sample_time: 0.0,
        }
    }

    #[inline]
    pub fn should_sample(&self, current_time: f64) -> bool {
        current_time >= self.next_sample_time
    }

    #[inline]
    pub fn sample(&mut self, current_time: f64, value: T) -> bool {
        if self.should_sample(current_time) {
            self.data.push((current_time, value));
            self.next_sample_time += self.sample_interval;
            true
        } else {
            false
        }
    }

    pub fn data(&self) -> &[(f64, T)] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[derive(Clone)]
pub struct SimulationTimeSeries {
    pub queue_length: TimeSeries<usize>,
    pub mean_wait_time: TimeSeries<f64>,
    pub utilization: TimeSeries<f64>,
    pub customers_served: TimeSeries<u64>,
    pub customers_in_system: TimeSeries<usize>,
    pub throughput: TimeSeries<f64>,
}

impl SimulationTimeSeries {
    pub fn new(sample_interval: f64, max_samples: usize) -> Self {
        Self {
            queue_length: TimeSeries::new(sample_interval, max_samples),
            mean_wait_time: TimeSeries::new(sample_interval, max_samples),
            utilization: TimeSeries::new(sample_interval, max_samples),
            customers_served: TimeSeries::new(sample_interval, max_samples),
            customers_in_system: TimeSeries::new(sample_interval, max_samples),
            throughput: TimeSeries::new(sample_interval, max_samples),
        }
    }

    /// only need to check one
    #[inline]
    pub fn should_sample(&self, current_time: f64) -> bool {
        self.queue_length.should_sample(current_time)
    }
}
