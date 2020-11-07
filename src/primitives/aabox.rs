use super::{XyPlane, XzPlane, YzPlane};
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::material::Material;
use crate::math::{Aabb, Point3, Ray};
use std::sync::Arc;

pub struct AaBox {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl AaBox {
    pub fn new(min: Point3, max: Point3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(XyPlane::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            max.z(),
            material.clone(),
        ));
        sides.add(XyPlane::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            min.z(),
            material.clone(),
        ));

        sides.add(XzPlane::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            max.y(),
            material.clone(),
        ));
        sides.add(XzPlane::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            min.y(),
            material.clone(),
        ));

        sides.add(YzPlane::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            max.x(),
            material.clone(),
        ));
        sides.add(YzPlane::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            min.x(),
            material.clone(),
        ));

        Self { min, max, sides }
    }
}

impl Hittable for AaBox {
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.min, self.max))
    }
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }
}
