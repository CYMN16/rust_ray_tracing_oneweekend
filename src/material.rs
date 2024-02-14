#![allow(unused_assignments)]
use crate::{hittable::*, random_f64, Color, Ray, Vec3};
pub trait Material: Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal.clone() + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        *scattered = Ray::new(rec.p.clone(), scatter_direction.clone());
        *attenuation = self.albedo.clone();
        true
    }
}
pub struct Metal {
    albedo: Color,
    f: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Self {
        Self {
            albedo,
            f: if f < 1. { f } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r_in.direction.unit(), &rec.normal);

        *scattered = Ray::new(
            rec.p.clone(),
            reflected + self.f * Vec3::random_unit_vector(),
        );
        *attenuation = self.albedo.clone();

        return scattered.direction.dot(&rec.normal) > 0.;
    }
}
pub struct Dielectric {
    ir: f64, //index of refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.unit();

        let cos_theta = (-&unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let mut direction = Vec3::default();

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_f64() {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        let refracted = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);

        *scattered = Ray::new(rec.p.clone(), direction);
        return true;
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    //Shlick approximation for reflectance
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cosine).powi(5);
}
