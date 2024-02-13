use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;
use crate::Interval;
use std::sync::Arc;
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    material: Arc<dyn Material>,
}
unsafe impl Sync for Sphere {}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}
// , material: Rc<Material>
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = &r.origin - &self.center;

        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - &self.radius * &self.radius;

        let discriminant = half_b * half_b - a * c;
        let sqrtd = discriminant.sqrt();
        if discriminant < 0. {
            return false;
        }

        let root = (-half_b - sqrtd) / a;

        if !ray_t.surrounds(root) {
            let root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.mat = self.material.clone();
        let outward_normal = (&rec.p - &self.center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);

        return true;
    }
}
