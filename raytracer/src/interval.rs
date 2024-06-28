use crate::rtweekend::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(a: f64, b: f64) -> Self {
        Self { min: a, max: b }
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn empty() -> Self {
        Self::new(INFINITY, -INFINITY)
    }
    pub fn universe() -> Self {
        Self::new(-INFINITY, INFINITY)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::new(INFINITY, -INFINITY)
    }
}
