use crate::material::{Isotropic, Material};
use crate::math::Ray;
use crate::{
    hittable::{HitRecord, Hittable},
    math::{Aabb, Color, Vec3},
    texture::*,
};
use rand::{thread_rng, Rng};
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_fn: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn from_texture(
        hittable: Arc<dyn Hittable>,
        density: f64,
        albedo: Arc<dyn Texture>,
    ) -> Self {
        Self {
            boundary: hittable,
            neg_inv_density: -1.0 / density,
            phase_fn: Arc::new(Isotropic::from(albedo)),
        }
    }

    pub fn from_color(hittable: Arc<dyn Hittable>, density: f64, color: Color) -> Self {
        Self {
            boundary: hittable,
            neg_inv_density: -1.0 / density,
            phase_fn: Arc::new(Isotropic::new(color)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        use std::f64::{INFINITY, NEG_INFINITY};
        let hit_0 = self.boundary.hit(ray, NEG_INFINITY, INFINITY);
        if hit_0.is_none() {
            return None;
        }

        let hit_1 = self
            .boundary
            .hit(ray, hit_0.as_ref().unwrap().t + 0.0001, INFINITY);
        if hit_1.is_none() {
            return None;
        }

        let mut hit_0 = hit_0.unwrap();
        let mut hit_1 = hit_1.unwrap();

        if hit_0.t < t_min {
            hit_0.t = t_min;
        }
        if hit_1.t > t_max {
            hit_1.t = t_max;
        }

        if hit_0.t >= hit_1.t {
            return None;
        }

        if hit_0.t < 0.0 {
            hit_0.t = 0.0;
        }

        let ray_len = ray.direction().length();
        let distance_inside = (hit_1.t - hit_0.t) * ray_len;
        let hit_dist = self.neg_inv_density * thread_rng().gen::<f64>().ln();

        if hit_dist > distance_inside {
            return None;
        }

        let mut hit_rec = HitRecord::new();
        hit_rec.t = hit_0.t + hit_dist / ray_len;
        hit_rec.point = ray.at(hit_rec.t);
        hit_rec.normal = Vec3::new(1.0, 0.0, 0.0);
        hit_rec.front_face = true;
        hit_rec.material = Some(self.phase_fn.clone());

        Some(hit_rec)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(t0, t1)
    }
}
