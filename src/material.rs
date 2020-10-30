//! Types for sampling light reflected from different surfaces

#![allow(dead_code)]

use crate::hittable::HitRecord;
use crate::math::{Ray, Vec3};
use num::Float;
use rand::{thread_rng, Rng};

/// Scene trait for sampling reflected light.
pub trait Material: Sync + Send {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

#[allow(unused_variables)]
impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        let scatter_direction = hit_rec.normal + Vec3::random_unit_vector();
        *scattered_ray = Ray::new(hit_rec.point, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        let reflected = ray_in.direction().unit().reflect(&hit_rec.normal);
        *scattered_ray = Ray::new(
            hit_rec.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        *attenuation = self.albedo;

        scattered_ray.direction().dot(&hit_rec.normal) > 0.0
    }
}

pub struct Dielectric {
    ior: f64,
}

impl Dielectric {
    pub fn new(ior: f64) -> Self {
        Self { ior }
    }
    fn reflectance(cosine: f64, ior: f64) -> f64 {
        let mut r0 = (1.0 - ior) / (1.0 + ior);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_rec.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = ray_in.direction().unit();
        let cos_theta = 1.0.min((-unit_direction).dot(&hit_rec.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > thread_rng().gen() {
            direction = unit_direction.reflect(&hit_rec.normal);
        } else {
            direction = unit_direction.refract(&hit_rec.normal, refraction_ratio);
        }

        *scattered_ray = Ray::new(hit_rec.point, direction);
        return true;
    }
}
