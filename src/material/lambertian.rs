use super::{Material, ScatterRecord};
use crate::hittable::HitRecord;
use crate::math::{Onb, Ray, Vec3};
use crate::pdf::CosinePdf;
use crate::texture::{SolidColor, Texture};
use std::sync::Arc;

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::from(albedo)),
        }
    }
}

impl From<Arc<dyn Texture>> for Lambertian {
    fn from(texture: Arc<dyn Texture>) -> Self {
        Self { albedo: texture }
    }
}

#[allow(unused_variables)]
impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord, scatter: &mut ScatterRecord) -> bool {
        scatter.attenuation = self.albedo.sample(hit_rec.u, hit_rec.v, &hit_rec.point);
        scatter.pdf = Some(Arc::new(CosinePdf::new(&hit_rec.normal)));
        return true;
    }

    fn scattering_pdf(&self, ray_in: &Ray, hit_rec: &HitRecord, scattered_ray: &Ray) -> f64 {
        let cosine = hit_rec.normal.dot(&scattered_ray.direction().unit());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }
}
