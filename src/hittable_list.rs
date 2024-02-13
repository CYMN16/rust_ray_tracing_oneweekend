use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::vec3::*;
use std::rc::*;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

unsafe impl Sync for HittableList{

}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let temp_rec: &mut HitRecord = &mut HitRecord {
            p: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(
                r,
                Interval::new_with_init(ray_t.min, closest_so_far),
                temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

impl HittableList {
    pub fn new(&self) {}

    pub fn new_with_init(&self, object: Rc<dyn Hittable>) {}

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}
