use std::{fmt::*, iter::Sum, ops::*};
#[derive(Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn unit(&self) -> Self {
        self.clone() / self.length()
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

//add

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Self) -> Self::Output {
        &self + rhs
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

//sub
impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Self) -> Self::Output {
        &self - rhs
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

//mul

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}
impl Mul<&Vec3> for i64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(
            self as f64 * rhs.x,
            self as f64 * rhs.y,
            self as f64 * rhs.z,
        )
    }
}
impl Mul<Vec3> for i64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}

//div

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}
impl Div<i64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i64) -> Self::Output {
        Vec3::new(
            self.x / rhs as f64,
            self.y / rhs as f64,
            self.z / rhs as f64,
        )
    }
}
impl Div<i64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i64) -> Self::Output {
        self / rhs as f64
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        -1 * self
    }
}
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        -1 * self
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Vec3::default();
        for vec in iter {
            sum = sum + vec;
        }
        sum
    }
}
