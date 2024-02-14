use crate::hittable::*;
use crate::hittable_list::*;
use crate::image::*;
use crate::interval::*;
use crate::ray::*;
use crate::sphere::*;
use crate::utility::*;
use crate::vec3::*;
use crate::Lambertian;
use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_lr: Vec3,
    pixel_delta_ud: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.,
            lookfrom: Point3::new(0., 0., -1.),
            lookat: Point3::new(0., 0., 0.),
            vup: Vec3::new(0., 1., 0.),
            image_height: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_lr: Default::default(),
            pixel_delta_ud: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
        }
    }
}

impl Camera {
    pub fn initialize(&mut self) {
        //image
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.center = self.lookfrom.clone();

        let focal_length = (&self.lookfrom - &self.lookat).length();

        let theta = deg_to_rad(self.vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focal_length;

        let viewport_width = viewport_height * self.image_width as f64 / self.image_height as f64; //self.aspect_ratio;

        self.w = (&self.lookfrom - &self.lookat).unit();
        self.u = self.vup.cross(&self.w).unit();
        self.v = self.w.cross(&self.u);

        let viewport_lr = viewport_width * &self.u;
        let viewport_ud = viewport_height * -&self.v;

        self.pixel_delta_lr = &viewport_lr / self.image_width as f64;
        self.pixel_delta_ud = &viewport_ud / self.image_height as f64;

        let viewport_upper_left =
            &self.center - (focal_length * &self.w) - viewport_lr / 2. - viewport_ud / 2.; //Vec3::new(0., 0., focal_length) - &viewport_lr / 2. - &viewport_ud / 2.;

        self.pixel00_loc =
            viewport_upper_left + 0.5 * (&self.pixel_delta_lr + &self.pixel_delta_ud);
    }

    pub fn ray_color(r: &Ray, depth: usize, world: &dyn Hittable) -> Color {
        let rec: &mut HitRecord = &mut HitRecord {
            p: Vec3::default(),
            normal: Vec3::default(),
            mat: Arc::new(Lambertian::new(Color::default())),
            t: 0.,
            front_face: false,
        };

        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        if world.hit(r, Interval::new_with_init(0.001, INFINITY), rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();

            if rec.mat.scatter(r, rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            } else {
                return Color::default();
            }
        }

        let unit_direction = r.direction.unit();

        let a = 0.5 * (unit_direction.y + 1.);

        (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
    }

    pub fn render(&mut self, world: &dyn Hittable) -> Image {
        Self::initialize(self);
        let mut image = Image::new(self.image_height, self.image_width);

        for row in 0..self.image_height {
            for col in 0..self.image_width {
                let mut pixel_color = Color::new(0., 0., 0.);
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(col, row);
                    pixel_color = pixel_color + Self::ray_color(&r, self.max_depth, world);
                }

                image.pixels[row][col] = pixel_color.to_pixel_with_spp(self.samples_per_pixel);
            }
        }
        image
    }

    pub fn parallel_render(&mut self, world: &dyn Hittable) -> Image {
        Self::initialize(self);
        let image = Image::new(self.image_height, self.image_width);
        let mut processed_image = Image::new(self.image_height, self.image_width);

        processed_image
            .pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(x, row)| {
                row.par_iter_mut().enumerate().for_each(|(y, pixel)| {
                    // *pixel = Pixel { r: 0, g: 0, b: 0 };
                    let pixel_color = (0..self.samples_per_pixel)
                        .into_par_iter()
                        .map(|_sample| {
                            // set_device(0);
                            let r = self.get_ray(y, x);

                            Self::ray_color(&r, self.max_depth, world)
                        })
                        .sum::<Color>();
                    *pixel = pixel_color.to_pixel_with_spp(self.samples_per_pixel)
                });
            });
        processed_image
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center = &self.pixel00_loc
            + (i as f64 * &self.pixel_delta_lr)
            + (j as f64 * &self.pixel_delta_ud);

        let pixel_sample = &pixel_center + self.pixel_sample_square();
        let ray_origin = self.center.clone();

        let ray_direction = &pixel_sample - &ray_origin;

        let ray = Ray::new(ray_origin, ray_direction);

        ray
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_f64();
        let py = -0.5 + random_f64();
        return (px * &self.pixel_delta_lr) + (py * &self.pixel_delta_ud);
    }
}
