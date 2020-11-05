use crate::material::Material;
use crate::math::Ray;
use crate::{
    hittable::{HitRecord, Hittable},
    math::{Aabb, Point3, Vec3},
};
use std::sync::Arc;

pub struct XyPlane {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Arc<dyn Material>,
}

impl XyPlane {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl Hittable for XyPlane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().z()) / ray.direction().z();

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let y = ray.origin().y() + t * ray.direction().y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let mut hit_rec = HitRecord::new();

        hit_rec.u = (x - self.x0) / (self.x1 - self.x0);
        hit_rec.v = (y - self.y0) / (self.y1 - self.y0);
        hit_rec.t = t;

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        hit_rec.set_face_normal(ray, &outward_normal);

        hit_rec.material = Some(self.material.clone());
        hit_rec.point = ray.at(t);

        Some(hit_rec)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        let offset = 0.0001;
        Some(Aabb::new(
            Point3::new(self.x0, self.y0, self.k - offset),
            Point3::new(self.x1, self.y1, self.k + offset),
        ))
    }
}

pub struct XzPlane {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<dyn Material>,
}

impl XzPlane {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hittable for XzPlane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().y()) / ray.direction().y();

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let z = ray.origin().z() + t * ray.direction().z();

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let mut hit_rec = HitRecord::new();

        hit_rec.u = (x - self.x0) / (self.x1 - self.x0);
        hit_rec.v = (z - self.z0) / (self.z1 - self.z0);
        hit_rec.t = t;

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        hit_rec.set_face_normal(ray, &outward_normal);

        hit_rec.material = Some(self.material.clone());
        hit_rec.point = ray.at(t);

        Some(hit_rec)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        let offset = 0.0001;
        Some(Aabb::new(
            Point3::new(self.x0, self.k - offset, self.z0),
            Point3::new(self.x1, self.k + offset, self.z1),
        ))
    }
}

pub struct YzPlane {
    z0: f64,
    z1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Arc<dyn Material>,
}

impl YzPlane {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hittable for YzPlane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().x()) / ray.direction().x();

        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin().y() + t * ray.direction().y();
        let z = ray.origin().z() + t * ray.direction().z();

        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let mut hit_rec = HitRecord::new();

        hit_rec.u = (y - self.y0) / (self.y1 - self.y0);
        hit_rec.v = (z - self.z0) / (self.z1 - self.z0);
        hit_rec.t = t;

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        hit_rec.set_face_normal(ray, &outward_normal);

        hit_rec.material = Some(self.material.clone());
        hit_rec.point = ray.at(t);

        Some(hit_rec)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        let offset = 0.0001;
        Some(Aabb::new(
            Point3::new(self.k - offset, self.y0, self.z0),
            Point3::new(self.k + offset, self.y1, self.z1),
        ))
    }
}
