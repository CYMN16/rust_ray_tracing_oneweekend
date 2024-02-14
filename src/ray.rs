use crate::image::*;
use crate::vec3::*;
use std::fmt::*;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Default::default(),
            direction: Default::default(),
        }
    }
}

impl Debug for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Origin: {:?}, Direction {:?}",
            self.origin, self.direction
        )
    }
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, scale: f64) -> Point3 {
        self.origin.clone() + scale * &self.direction
    }
}
