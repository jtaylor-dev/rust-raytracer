use super::Material;
use crate::hittable::HitRecord;
use crate::math::{Ray, Vec3};
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
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        let scatter_direction = hit_rec.normal + Vec3::random_unit_vector();
        *scattered_ray = Ray::new(hit_rec.point, scatter_direction, ray_in.time());
        *attenuation = self.albedo.sample(hit_rec.u, hit_rec.v, &hit_rec.point);
        return true;
    }
}
