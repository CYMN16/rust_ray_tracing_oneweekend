use crate::utility::*;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub static EMPTY: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};
pub static UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};

impl Interval {
    pub fn new() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub fn new_with_init(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        } else {
            return x;
        }
    }
}
