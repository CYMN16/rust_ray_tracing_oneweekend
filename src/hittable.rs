use crate::material::*;
use crate::ray::*;
use crate::vec3::*;
use crate::Interval;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        //outward_normal is assumed to be unit length
        self.front_face = r.direction.dot(&outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            (-outward_normal).clone()
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        false
    }
}
