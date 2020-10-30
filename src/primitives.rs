//! Types that implement [`Hittable`](crate::hittable::Hittable)

use crate::material::Material;
use crate::math::{self, Ray};
use crate::{
    hittable::{HitRecord, Hittable},
    math::{Point3, Vec3},
};
use std::sync::Arc;

pub struct Sphere {
    sphere: math::Sphere,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            sphere: math::Sphere::new(center, radius),
            material,
        }
    }
}

impl Hittable for Sphere {
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

pub struct MovingSphere {
    center_0: Point3,
    time_0: f64,
    center_1: Point3,
    time_1: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center_0: Point3,
        center_1: Point3,
        radius: f64,
        time_0: f64,
        time_1: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center_0,
            time_0,
            center_1,
            time_1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center_0
            + ((time - self.time_0) / (self.time_1 - self.time_0)) * (self.center_1 - self.center_0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let radius = self.radius;

        let center_to_origin = ray.origin() - self.center(ray.time());
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
                let outward_normal = (hit_rec.point - self.center(ray.time())) / radius;
                hit_rec.set_face_normal(ray, &outward_normal);
                hit_rec.material = Some(self.material.clone());
                return Some(hit_rec);
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let mut hit_rec = HitRecord::new();
                hit_rec.t = temp;
                hit_rec.point = ray.at(hit_rec.t);
                let outward_normal = (hit_rec.point - self.center(ray.time())) / radius;
                hit_rec.set_face_normal(ray, &outward_normal);
                hit_rec.material = Some(self.material.clone());
                return Some(hit_rec);
            }
        }
        return None;
    }
}
