use crate::hittable::*;
use crate::hittable_list::*;
use crate::image::*;
use crate::interval::*;
use crate::ray::*;
use crate::sphere::*;
use crate::utility::*;
use crate::vec3::*;
use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_lr: Vec3,
    pixel_delta_ud: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            image_height: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_lr: Default::default(),
            pixel_delta_ud: Default::default(),
        }
    }
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize, samples_per_pixel: usize) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_lr: Default::default(),
            pixel_delta_ud: Default::default(),
        }
    }

    pub fn initialize(&mut self) {
        //image
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;

        // //world
        // let mut world: HittableList = HittableList{ objects: vec![] };

        // world.add(Rc::new(Sphere::new(Point3::new(0.,0.,-1.), 0.5)));
        // world.add(Rc::new(Sphere::new(Point3::new(0.,-100.5,-1.), 100.)));

        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * self.aspect_ratio;
        self.center = Point3::new(0., 0., 0.);

        let viewport_lr = Vec3::new(viewport_width, 0., 0.);
        let viewport_ud = Vec3::new(0., -viewport_height, 0.);

        self.pixel_delta_lr = &viewport_lr / self.image_width as f64;
        self.pixel_delta_ud = &viewport_ud / self.image_height as f64;

        let viewport_upper_left =
            &self.center - Vec3::new(0., 0., focal_length) - &viewport_lr / 2. - &viewport_ud / 2.;

        self.pixel00_loc =
            viewport_upper_left + 0.5 * (&self.pixel_delta_lr + &self.pixel_delta_ud);
    }

    pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let rec: &mut HitRecord = &mut HitRecord {
            p: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
            front_face: false,
        };

        if world.hit(r, Interval::new_with_init(0., INFINITY), rec) {
            return 0.5 * (rec.normal.clone() + Color::new(0.5, 0.7, 1.0));
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
                    pixel_color = pixel_color + Self::ray_color(&r, world);
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
                                let r = self.t_get_ray(y, x);

                                Self::ray_color(&r, world)
                            })
                            .sum::<Color>();
                        *pixel = pixel_color.to_pixel_with_spp(self.samples_per_pixel)
                });
            });

        // let row: Vec<Vec<Pixel>> = image.pixels
        //     .par_iter()
        //     .enumerate()
        //     .for_each(|(x, row)| -> Vec<Pixel> {
        //         let col = row
        //             .par_iter()
        //             .enumerate()
        //             .for_each(|(y, mut pixel)| -> Pixel {
        //                 let pixel_color = (0..self.samples_per_pixel)
        //                     .into_par_iter()
        //                     .map(|_sample| {
        //                         let r = self.t_get_ray(y, x);

        //                         Self::ray_color(&r, world)
        //                     })
        //                     .sum::<Color>();
        //                 pixel_color.to_pixel_with_spp(self.samples_per_pixel)
        //             }).collect()
        //     }).collect();

        processed_image
    }
    fn t_get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center = &self.pixel00_loc
            + (i as f64 * &self.pixel_delta_lr)
            + (j as f64 * &self.pixel_delta_ud);

        let pixel_sample = &pixel_center + self.t_pixel_sample_square();
        let ray_origin = self.center.clone();

        let ray_direction = &pixel_sample - &ray_origin;

        let ray = Ray::new(ray_origin, ray_direction);

        ray
    }

    fn t_pixel_sample_square(&self) -> Vec3 {
        let mut rng = thread_rng();
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();
        return (px * &self.pixel_delta_lr) + (py * &self.pixel_delta_ud);
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