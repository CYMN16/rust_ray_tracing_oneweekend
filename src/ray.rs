use crate::image::*;
use crate::vec3::*;
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, scale: f64) -> Point3 {
        self.origin.clone() + scale * &self.direction
    }
}
