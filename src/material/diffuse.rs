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
    fn emitted(&self, ray: &Ray, hit_rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !hit_rec.front_face {
            Color::default()
        } else {
            self.emit.sample(u, v, p)
        }
    }
}
