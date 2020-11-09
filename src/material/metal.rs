use super::{Material, ScatterRecord};
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
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord, scatter: &mut ScatterRecord) -> bool {
        let reflected = ray_in.direction().unit().reflect(&hit_rec.normal);
        scatter.specular_ray = Some(Ray::new(
            hit_rec.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            ray_in.time(),
        ));
        scatter.attenuation = self.albedo;
        scatter.pdf = None;
        return true;
    }
}
