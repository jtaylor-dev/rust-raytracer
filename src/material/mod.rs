//! Types for sampling light reflected from different surfaces
#![allow(dead_code)]

use crate::hittable::HitRecord;
use crate::math::{Color, Point3, Ray, Vec3};

/// Scene trait for sampling reflected light.
pub trait Material: Sync + Send {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool;

    #[allow(unused_variables)]
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::default()
    }
}

mod dielectric;
mod diffuse;
mod isotropic;
mod lambertian;
mod metal;

pub use dielectric::*;
pub use diffuse::*;
pub use isotropic::*;
pub use lambertian::*;
pub use metal::*;
