use crate::rtweekend::INFINITY;

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(a: f64, b: f64) -> Self {
        Self { min: a, max: b }
    }
    pub fn interval_new(a: Interval, b: Interval) -> Self {
        Self {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
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
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
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
