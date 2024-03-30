use crate::INF;

#[derive(Debug, Default, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

// pub static EMPTY: Interval = Interval::new(INF, -INF);

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn new_from_intervals(a: Interval, b: Interval) -> Self {
        Self {
            min: f64::min(a.min, b.min),
            max: f64::max(a.max, b.max),
        }
    }

    pub fn empty() -> Self {
        Self {
            min: INF,
            max: -INF,
        }
    }
    pub fn universe() -> Self {
        Self {
            min: -INF,
            max: INF,
        }
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}
