#![allow(dead_code)]
use rand::{thread_rng, Rng};
use std::fmt;
use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    data: [f64; 3],
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(v1: f64, v2: f64, v3: f64) -> Self {
        Self { data: [v1, v2, v3] }
    }

    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn length(&self) -> f64 {
        let length_squared: f64 = self.length_squared();
        length_squared.sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.data[0] * rhs.data[0] + self.data[1] * rhs.data[1] + self.data[2] * rhs.data[2]
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            data: [
                self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
                self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
                self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
            ],
        }
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        let v = *self;
        let n = *normal;
        v - 2.0 * v.dot(&n) * n
    }

    pub fn refract(&self, normal: &Self, ior: f64) -> Self {
        let uv = *self;
        let n = *normal;
        let cos_theta = -uv.dot(normal);
        let r_perp: Vec3 = ior * (uv + cos_theta * n);
        let r_parallel = -((1.0 - r_perp.length_squared()).abs().sqrt()) * n;
        r_perp + r_parallel
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();
        Self::new(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in_range(-1., 1.);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_hemisphere(normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            // in the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = thread_rng();

        loop {
            let p = Self::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        let a: f64 = thread_rng().gen_range(0.0, 2.0 * std::f64::consts::PI);
        let z: f64 = thread_rng().gen_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Self::new(r * a.cos(), r * a.sin(), z)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            data: [0.0, 0.0, 0.0],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            data: [-self.data[0], -self.data[1], -self.data[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.data[index]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            data: [
                self.data[0] - rhs.data[0],
                self.data[1] - rhs.data[1],
                self.data[2] - rhs.data[2],
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            data: [
                self.data[0] * rhs.data[0],
                self.data[1] * rhs.data[1],
                self.data[2] * rhs.data[2],
            ],
        }
    }
}

impl Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self {
            data: [
                self.data[0] * rhs as f64,
                self.data[1] * rhs as f64,
                self.data[2] * rhs as f64,
            ],
        }
    }
}

impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<i32> for Vec3 {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Self::Output {
        self * (1f64 / rhs as f64)
    }
}

impl DivAssign<i32> for Vec3 {
    fn div_assign(&mut self, rhs: i32) {
        *self *= 1f64 / rhs as f64;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            data: [self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1f64 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1f64 / rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.data[0], self.data[1], self.data[2])
    }
}
