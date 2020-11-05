use super::Material;
use crate::hittable::HitRecord;
use crate::math::{Color, Point3, Ray, Vec3};
use crate::texture::{SolidColor, Texture};
use std::sync::Arc;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(color: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::from(color)),
        }
    }
}

impl From<Arc<dyn Texture>> for DiffuseLight {
    fn from(texture: Arc<dyn Texture>) -> Self {
        Self { emit: texture }
    }
}

impl Material for DiffuseLight {
    #[allow(unused_variables)]
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.sample(u, v, p)
    }
}
