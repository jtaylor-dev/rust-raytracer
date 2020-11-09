use super::Material;
use crate::hittable::HitRecord;
use crate::math::{Color, Ray, Vec3};
use crate::texture::{SolidColor, Texture};
use std::sync::Arc;

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::from(albedo)),
        }
    }
}

impl From<Arc<dyn Texture>> for Isotropic {
    fn from(texture: Arc<dyn Texture>) -> Self {
        Self { albedo: texture }
    }
}

impl Material for Isotropic {
    //fn scatter(
    //    &self,
    //    ray_in: &Ray,
    //    hit_rec: &HitRecord,
    //    albedo: &mut Vec3,
    //    scattered_ray: &mut Ray,
    //    pdf: &mut f64,
    //) -> bool {
    //    *scattered_ray = Ray::new(hit_rec.point, Vec3::random_in_unit_sphere(), ray_in.time());
    //    *albedo = self.albedo.sample(hit_rec.u, hit_rec.v, &hit_rec.point);
    //    return true;
    //}
}
