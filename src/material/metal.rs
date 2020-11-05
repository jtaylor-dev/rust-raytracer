use super::Material;
use crate::hittable::HitRecord;
use crate::math::{Ray, Vec3};

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
            ray_in.time(),
        );
        *attenuation = self.albedo;

        scattered_ray.direction().dot(&hit_rec.normal) > 0.0
    }
}
