use crate::{vec3::*, Interval};
use std::{
    fmt::*,
    thread::{self, JoinHandle},
};
#[derive(Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Image {
    pub pixels: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn new(height: usize, width: usize) -> Self {
        let mut pixels = Vec::with_capacity(height);

        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Pixel::default())
            }
            pixels.push(row);
        }

        Self { pixels }
    }

    pub fn new_with_init(
        height: usize,
        width: usize,
        init: impl Fn(usize, usize) -> Pixel,
    ) -> Self {
        let mut image = Self::new(height, width);

        for row in 0..height {
            for col in 0..width {
                image.pixels[row][col] = init(row, col);
            }
        }

        image
    }

    pub fn height(&self) -> usize {
        self.pixels.len()
    }
    pub fn width(&self) -> usize {
        self.pixels[0].len()
    }
}

#[derive(Debug)]
pub struct PPM<'a, T>(pub &'a T);

impl Display for PPM<'_, Pixel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>3} {:>3} {:>3}", self.0.r, self.0.g, self.0.b)
    }
}

impl Debug for PPM<'_, Pixel> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self)
    }
}

impl Display for PPM<'_, Image> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.0.width(), self.0.height())?;
        writeln!(f, "255")?;

        for row in 0..self.0.height() {
            for col in 0..self.0.width() {
                writeln!(f, "{}", PPM(&self.0.pixels[row][col]))?;
            }
        }
        Ok(())
    }
}

impl Debug for PPM<'_, Image> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self)
    }
}

pub type Color = Vec3;

impl Color {
    pub fn to_pixel(&self) -> Pixel {
        let factor = 255.999;
        let scaled = factor * self;

        Pixel {
            r: scaled.x as u8,
            g: scaled.y as u8,
            b: scaled.z as u8,
        }
    }

    pub fn to_pixel_with_spp(&self, samples_per_pixel: usize) -> Pixel {
        let intensity = Interval::new_with_init(0., 0.999);

        let scale = 1. / samples_per_pixel as f64;
        let scaled = scale * self;

        fn linear_to_gamma(linear_component: f64) -> f64 {
            linear_component.sqrt()
        }
        let scaled = scaled.map(linear_to_gamma);
        Pixel {
            r: (256. * intensity.clamp(scaled.x)) as u8,
            g: (256. * intensity.clamp(scaled.y)) as u8,
            b: (256. * intensity.clamp(scaled.z)) as u8,
        }
    }
}
