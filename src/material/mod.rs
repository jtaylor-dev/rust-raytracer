//! Types for sampling light reflected from different surfaces
#![allow(dead_code)]

use crate::hittable::HitRecord;
use crate::math::{Color, Point3, Ray};
use crate::pdf::Pdf;

/// Scene trait for sampling reflected light.
#[allow(unused_variables)]
pub trait Material: Sync + Send {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord, scatter: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, ray_in: &Ray, hit_rec: &HitRecord, scattered_ray: &Ray) -> f64 {
        0.0
    }

    #[allow(unused_variables)]
    fn emitted(&self, ray_in: &Ray, hit_rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        Color::default()
    }
}

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub attenuation: Color,
    pub pdf: Option<std::sync::Arc<dyn Pdf>>,
}

impl ScatterRecord {
    pub fn new() -> Self {
        Self {
            specular_ray: None,
            attenuation: Color::default(),
            pdf: None,
        }
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
