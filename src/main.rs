#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod image;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod utility;
pub mod vec3;

use camera::*;
use hittable::*;
use hittable_list::*;
use image::*;
use interval::*;
use ray::*;
use sphere::*;
use utility::*;
use vec3::*;
use rand;

use std::{
    fs::File,
    io::{BufWriter, Write},
    rc::*,
    time::Instant,
};

fn main() -> std::io::Result<()> {
    // set_device(0);
    let mut buffer = BufWriter::new(File::create("sample.ppm")?);

    // let img = simple_scene();
    let mut cam = Camera::default(); //= Camera::new(16./9., 400);
    cam.aspect_ratio = 16. / 9.;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;

    let mut world: HittableList = HittableList { objects: vec![] };

    world.add(Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let time_start = Instant::now();

    let img = cam.parallel_render(&world);

    let time_end = Instant::now();

    write!(buffer, "{}", PPM(&img))?;
    buffer.flush()?;
    println!("Successfully generated PPM image!");
    println!("Time start: {:?}", time_start);
    println!("Time end: {:?}", time_end);
    println!("Time elapsed: {:?}", time_end - time_start);

    Ok(())
}

#[cfg(test)]
mod test_random {
    use super::*;

    #[test]
    fn test_hit() {
        // let h1 = Hittable::new();
        // let r1 = rand::random::<f64>();
        // k9::snapshot!(r1, "0.5618833664477753");
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