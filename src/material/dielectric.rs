use super::{Material, ScatterRecord};
use crate::hittable::HitRecord;
use crate::math::{Ray, Vec3};
use rand::{thread_rng, Rng};

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
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord, scatter: &mut ScatterRecord) -> bool {
        scatter.pdf = None;
        scatter.attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_rec.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = ray_in.direction().unit();
        let cos_theta = (1.0 as f64).min((-unit_direction).dot(&hit_rec.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > thread_rng().gen() {
            direction = unit_direction.reflect(&hit_rec.normal);
        } else {
            direction = unit_direction.refract(&hit_rec.normal, refraction_ratio);
        }

        scatter.specular_ray = Some(Ray::new(hit_rec.point, direction, ray_in.time()));
        return true;
    }
}
