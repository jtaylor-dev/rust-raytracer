//! Scene-level abstractions for objects that can intersect with raycasts

use crate::bvh::BvhNode;
use crate::material::Material;
use crate::math::{Aabb, Point3, Ray, Vec3};
use std::sync::Arc;

/// Maintains a record of a ray intersection with a [`Hittable`] object.
#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Point3::default(),
            normal: Vec3::default(),
            t: 0.,
            u: 0.,
            v: 0.,
            front_face: false,
            material: None,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

/// Scene trait for intersecting with rays.
pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb>;
}
/// Stores a list of hittable scene objects.
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Arc::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn objects(&self) -> &Vec<Arc<dyn Hittable>> {
        &self.objects
    }
}

impl From<BvhNode> for HittableList {
    fn from(bvh: BvhNode) -> Self {
        Self {
            objects: vec![Arc::new(bvh)],
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit = Some(temp_rec);
            }
        }

        return hit;
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        if self.objects().is_empty() {
            return None;
        }

        let mut output_box = Aabb::default();
        let mut first_box = true;

        for object in self.objects.iter() {
            if let Some(temp_box) = object.bounding_box(t0, t1) {
                output_box = if first_box {
                    temp_box
                } else {
                    Aabb::surrounding_box(&output_box, &temp_box)
                };
                first_box = false;
            } else {
                return None;
            }
        }

        Some(output_box)
    }
}

pub struct Translate {
    hittable: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(hittable: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self { hittable, offset }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        if let Some(aabb) = self.hittable.bounding_box(t0, t1) {
            Some(Aabb::new(
                aabb.min() + self.offset,
                aabb.max() + self.offset,
            ))
        } else {
            None
        }
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());

        if let Some(mut hit_rec) = self.hittable.hit(&moved_ray, t_min, t_max) {
            hit_rec.point += self.offset;
            let outward_normal = hit_rec.normal;
            hit_rec.set_face_normal(&moved_ray, &outward_normal);
            Some(hit_rec)
        } else {
            None
        }
    }
}

pub struct RotateY {
    hittable: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    aabb: Option<Aabb>,
}

impl RotateY {
    pub fn new(hittable: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let aabb = hittable.bounding_box(0.0, 1.0).unwrap();

        use std::f64::{INFINITY, NEG_INFINITY};
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * aabb.max().x() + (1.0 - i as f64) * aabb.min().x();
                    let y = j as f64 * aabb.max().y() + (1.0 - j as f64) * aabb.min().y();
                    let z = k as f64 * aabb.max().z() + (1.0 - k as f64) * aabb.min().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            hittable,
            cos_theta,
            sin_theta,
            aabb: Some(Aabb::new(min, max)),
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        self.aabb
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin[0] = self.cos_theta * ray.origin()[0] - self.sin_theta * ray.origin()[2];
        origin[2] = self.sin_theta * ray.origin()[0] + self.cos_theta * ray.origin()[2];

        direction[0] = self.cos_theta * ray.direction()[0] - self.sin_theta * ray.direction()[2];
        direction[2] = self.sin_theta * ray.direction()[0] + self.cos_theta * ray.direction()[2];

        let rotated_ray = Ray::new(origin, direction, ray.time());

        if let Some(mut hit_rec) = self.hittable.hit(&rotated_ray, t_min, t_max) {
            let mut p = hit_rec.point;
            let mut n = hit_rec.normal;

            p[0] = self.cos_theta * hit_rec.point[0] + self.sin_theta * hit_rec.point[2];
            p[2] = -self.sin_theta * hit_rec.point[0] + self.cos_theta * hit_rec.point[2];

            n[0] = self.cos_theta * hit_rec.normal[0] + self.sin_theta * hit_rec.normal[2];
            n[2] = -self.sin_theta * hit_rec.normal[0] + self.cos_theta * hit_rec.normal[2];

            hit_rec.point = p;
            hit_rec.set_face_normal(&rotated_ray, &n);

            Some(hit_rec)
        } else {
            None
        }
    }
}
