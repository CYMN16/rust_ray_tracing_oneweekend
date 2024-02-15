#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
pub mod camera;
pub mod display;
pub mod hittable;
pub mod hittable_list;
pub mod image;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utility;
pub mod vec3;

use camera::*;
use display::*;
use hittable::*;
use hittable_list::*;
use im::vector;
use image::*;
use interval::*;
use material::*;
use ray::*;
use sphere::*;
use utility::*;
use vec3::*;

use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::Arc,
    time::Instant,
};

fn main() -> std::io::Result<()> {
    // set_device(0);
    // let mut buffer = BufWriter::new(File::create("sample.ppm")?);

    // let img = simple_scene();
    let mut cam = Camera::default(); //= Camera::new(16./9., 400);
    cam.aspect_ratio = 16. / 9.;
    cam.image_width = 800;
    cam.samples_per_pixel = 5;
    cam.max_depth = 50;

    cam.vfov = 20.;
    cam.lookfrom = Point3::new(-2., 2., 1.);
    cam.lookat = Point3::new(0., 0., -1.);
    cam.vup = Vec3::new(0., 1., 0.);
    let mut world: HittableList = HittableList { objects: vector![] };

    // let r = (PI / 4.).cos();

    // let material_left = Arc::new(Lambertian::new(Color::new(0., 0., 1.)));
    // let material_right = Arc::new(Lambertian::new(Color::new(1., 0., 0.)));

    // world.add(Arc::new(Sphere::new(
    //     Point3::new(-r, 0., -1.),
    //     r,
    //     material_left,
    // )));
    // world.add(Arc::new(Sphere::new(
    //     Point3::new(r, 0., -1.),
    //     r,
    //     material_right,
    // )));

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.4,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right,
    )));

    // display
    // let time_start = Instant::now();
    
    display_image(&mut cam, &world);

    // let img = cam.parallel_render(&world);

    // display_image(&mut cam, &world);

    // let time_end = Instant::now();


    // write!(buffer, "{}", PPM(&img))?;
    // buffer.flush()?;
    // println!("Successfully generated PPM image!");
    // println!("Time start: {:?}", time_start);
    // println!("Time end: {:?}", time_end);
    // println!("Time elapsed: {:?}", time_end - time_start);

    Ok(())
}

#[cfg(test)]
mod test_random {
    use super::*;

    #[test]
    fn test_hit() {
        let mut cam = Camera::default(); //= Camera::new(16./9., 400);
        cam.aspect_ratio = 16. / 9.;
        cam.image_width = 400;
        cam.samples_per_pixel = 100;
        cam.max_depth = 50;

        let material_center = Arc::new(Dielectric::new(1.5));
        let ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0.2, 0.2, 0.2));
        let rec: &mut HitRecord = &mut HitRecord {
            p: Vec3::default(),
            normal: Vec3::default(),
            mat: Arc::new(Lambertian::new(Color::default())),
            t: 0.,
            front_face: false,
        };
        let mut attenuation = Color::default();
        let mut scattered = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
        material_center.scatter(&ray, &rec, &mut attenuation, &mut scattered);
        // let res = cam.parallel_render(world);
        k9::snapshot!(attenuation, "(1, 1, 1)");
        k9::snapshot!(
            scattered,
            "Origin: (0, 0, 0), Direction (0.5773502691896257, 0.5773502691896257, 0.5773502691896257)"
        );
    }

    #[test]
    fn test_cam() {
        let mut cam = Camera::default();

        cam.aspect_ratio = 16. / 9.;
        cam.image_width = 400;
        cam.samples_per_pixel = 10;
        cam.max_depth = 50;

        cam.vfov = 90.;
        cam.lookfrom = Point3::new(-2., 2., 1.);
        cam.lookat = Point3::new(0., 0., -1.);
        cam.vup = Vec3::new(0., 1., 0.);

        // cam.initialize();
        k9::snapshot!(
            cam,
            "
Camera {
    aspect_ratio: 1.7777777777777777,
    image_width: 400,
    samples_per_pixel: 10,
    max_depth: 50,
    vfov: 90.0,
    lookfrom: (-2, 2, 1),
    lookat: (0, 0, -1),
    vup: (0, 1, 0),
    image_height: 0,
    center: (0, 0, 0),
    pixel00_loc: (0, 0, 0),
    pixel_delta_lr: (0, 0, 0),
    pixel_delta_ud: (0, 0, 0),
    u: (0, 0, 0),
    v: (0, 0, 0),
    w: (0, 0, 0),
}
"
        );
    }
}
#[cfg(test)]
mod test_hittable {
    use super::*;

    #[test]
    fn test_hit() {
        // let h1 = Hittable::new();
        let h1 = 2;
        k9::snapshot!(h1, "2");
    }
}

#[cfg(test)]
mod test_vec {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec3::new(2., 3., 5.);
        let v2 = Vec3::new(2., 6., 9.);

        let v11 = &v1 + &v2;
        k9::snapshot!(v11, "(4, 9, 14)");

        let v12 = &v1 + Vec3::new(3., 4., 5.);

        k9::snapshot!(v12, "(5, 7, 10)");

        let v13 = Vec3::new(1., 2., 3.) + Vec3::new(2., 3., 3.);
        k9::snapshot!(v13, "(3, 5, 6)");
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(2., 3., 5.);
        let v2 = Vec3::new(2., 6., 9.);

        let v11 = &v1 - &v2;
        k9::snapshot!(v11, "(0, -3, -4)");

        let v12 = &v1 - Vec3::new(3., 4., 5.);

        k9::snapshot!(v12, "(-1, -1, 0)");

        let v13 = Vec3::new(1., 2., 3.) - Vec3::new(2., 3., 3.);
        k9::snapshot!(v13, "(-1, -1, 0)");
    }
    #[test]
    fn test_mul_f() {
        let v1 = Vec3::new(2., 3., 5.);
        let v11 = 2.1 * &v1;
        k9::snapshot!(v11, "(4.2, 6.300000000000001, 10.5)");
        let v12 = 3.1 * v1;
        k9::snapshot!(v12, "(6.2, 9.3, 15.5)");
    }

    #[test]
    fn test_mul_i() {
        let v1 = Vec3::new(2., 3., 5.);
        let v11 = 2 * &v1;
        k9::snapshot!(v11, "(4, 6, 10)");
        let v12 = 3 * v1;
        k9::snapshot!(v12, "(6, 9, 15)");
    }
    #[test]
    fn test_div_f() {
        let v1 = Vec3::new(2., 3., 5.);
        let v11 = &v1 / 2.;
        k9::snapshot!(v11, "(1, 1.5, 2.5)");
        let v12 = v1 / 3.;
        k9::snapshot!(v12, "(0.6666666666666666, 1, 1.6666666666666667)");
    }

    #[test]
    fn test_div_i() {
        let v1 = Vec3::new(2., 3., 5.);
        let v11 = &v1 / 2;
        k9::snapshot!(v11, "(1, 1.5, 2.5)");
        let v12 = v1 / 3;
        k9::snapshot!(v12, "(0.6666666666666666, 1, 1.6666666666666667)");
    }

    #[test]
    fn test_neg() {
        let v1 = Vec3::new(4., 3., 1.);
        k9::snapshot!(-&v1, "(-4, -3, -1)");

        let v2 = Vec3::new(2., 3., 8.);
        k9::snapshot!(-v2, "(-2, -3, -8)");
    }

    #[test]
    fn test_aux_funcs() {
        let v1 = Vec3::new(4., 3., 1.);
        k9::snapshot!(v1.length(), "5.0990195135927845");

        let v2 = Vec3::new(4., 7., 3.);
        k9::snapshot!(v1.dot(&v2), "40.0");
        k9::snapshot!(v1.cross(&v2), "(2, -8, 16)");

        let v4 = Vec3::new(1., 1., 0.);
        let v5 = Vec3::new(0., 1., 0.);
        k9::snapshot!(Vec3::refract(&v4, &v5, 1.5), "(1.5, -1.118033988749895, 0)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let pixel = Pixel { r: 1, g: 2, b: 3 };
        k9::snapshot!(PPM(&pixel), "  1   2   3");
    }

    #[test]
    fn test2() {
        // let img = Image::new(2, 3);
        let img = Image::new_with_init(2, 3, |row, col| Pixel {
            r: row as u8,
            g: col as u8,
            b: 20,
        });
        k9::snapshot!(
            PPM(&img),
            "
P3
3 2
255
  0   0  20
  0   1  20
  0   2  20
  1   0  20
  1   1  20
  1   2  20

"
        );
    }
}
