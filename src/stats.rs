#[derive(Copy, Clone, Debug)]
pub struct VarianceState {
    pub n: usize,
    pub m: f64,
    pub ssd: f64,
}

impl VarianceState {
    fn new() -> VarianceState {
        VarianceState {
            n: 0,
            m: 0.0,
            ssd: 0.0,
        }
    }

    // fn reset(&mut self) {
    //     *self = VarianceState::new();
    // }

    fn update(&mut self, value: u64) -> f64 {
        self.n += 1;
        let new_mean = self.m + (value as f64 - self.m) / (self.n as f64);
        let new_ssd =
            self.ssd + (value as f64 - self.m) * (value as f64 - new_mean);
        // if we have only seen one value, the variance is undefined, so we return 0.0
        let result = if self.n <= 1 {
            0.0
        } else {
            new_ssd / ((self.n - 1) as f64)
        };
        self.m = new_mean;
        self.ssd = new_ssd;
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Stats {
    pub min: u64,
    pub max: u64,
    pub variance: f64,
    pub variance_state: VarianceState,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            min: u32::MAX as u64,
            max: u32::MIN as u64,
            variance: 0.0,
            variance_state: VarianceState::new(),
        }
    }

    // pub fn reset(&mut self) {
    //     *self = Stats::new();
    // }

    pub fn update(&mut self, value: u64) {
        if value > self.max {
            self.max = value;
        }
        if value < self.min {
            self.min = value;
        }
        if self.min > self.max {
            self.min = self.max
        }
        if self.max < self.min {
            self.max = self.min
        }
        self.variance = self.variance_state.update(value);
    }

    pub fn get_min(&self) -> u64 {
        self.min
    }

    pub fn get_max(&self) -> u64 {
        self.max
    }

    pub fn get_variance(&self) -> f64 {
        self.variance
    }
}
