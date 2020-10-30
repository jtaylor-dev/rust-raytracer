//! Types that implement [`Hittable`](crate::hittable::Hittable)

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::{Ray, Sphere};
use std::sync::Arc;

pub struct SpherePrimitive {
    sphere: Sphere,
    material: Arc<dyn Material>,
}

impl SpherePrimitive {
    pub fn new(sphere: Sphere, material: &Arc<dyn Material>) -> Self {
        Self {
            sphere,
            material: material.clone(),
        }
    }
}

impl Hittable for SpherePrimitive {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.sphere.center();
        let radius = self.sphere.radius();

        let center_to_origin = ray.origin() - center;
        let a = ray.direction().length_squared();
        let half_b = center_to_origin.dot(&ray.direction());
        let c = center_to_origin.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let mut hit_rec = HitRecord::new();
                hit_rec.t = temp;
                hit_rec.point = ray.at(hit_rec.t);
                let outward_normal = (hit_rec.point - center) / radius;
                hit_rec.set_face_normal(ray, &outward_normal);
                hit_rec.material = Some(self.material.clone());
                return Some(hit_rec);
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let mut hit_rec = HitRecord::new();
                hit_rec.t = temp;
                hit_rec.point = ray.at(hit_rec.t);
                let outward_normal = (hit_rec.point - center) / radius;
                hit_rec.set_face_normal(ray, &outward_normal);
                hit_rec.material = Some(self.material.clone());
                return Some(hit_rec);
            }
        }
        return None;
    }
}
