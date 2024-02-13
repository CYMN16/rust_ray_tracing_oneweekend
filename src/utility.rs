use crate::interval::*;
use rand::{self, thread_rng, Rng};
use std::f64::{consts::PI as f64_pi, MAX};

pub const INFINITY: f64 = MAX;
pub const PI: f64 = f64_pi;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.
}

//random f64 between 0 and 1
pub fn random_f64() -> f64 {
    let mut rng = thread_rng();
    rng.gen::<f64>()
}

pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    min + random_f64() * (max - min)
}
