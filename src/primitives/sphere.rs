use crate::material::Material;
use crate::math::Ray;
use crate::{
    hittable::{HitRecord, Hittable},
    math::{Aabb, Onb, Point3, Vec3},
};
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        use std::f64::consts::PI;

        let neg_p = -*p;
        let theta = neg_p.y().acos();
        let phi = neg_p.z().atan2(p.x()) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center_to_origin = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = center_to_origin.dot(&ray.direction());
        let c = center_to_origin.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let hit_p = ray.at(root);
        let outward_normal = (hit_p - self.center) / self.radius;
        let (u, v) = Sphere::get_sphere_uv(&outward_normal);
        let mut hit_rec = HitRecord {
            t: root,
            point: hit_p,
            normal: Vec3::default(),
            front_face: false,
            material: Some(self.material.clone()),
            u,
            v,
        };
        hit_rec.set_face_normal(ray, &outward_normal);

        Some(hit_rec)
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        if let Some(_) = self.hit(&Ray::new(*o, *v, 0.0), 0.001, std::f64::INFINITY) {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center - *o).length_squared()).sqrt();
            let solid_angle = 2.0 * std::f64::consts::PI * (1.0 - cos_theta_max);

            1.0 / solid_angle
        } else {
            0.0
        }
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let direction: Vec3 = self.center - *o;
        let dist_squared = direction.length_squared();
        let uvw = Onb::new(&direction);
        uvw.local_vec(&Vec3::random_to_sphere(self.radius, dist_squared))
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
        let center_to_origin = ray.origin() - self.center(ray.time());
        let a = ray.direction().length_squared();
        let half_b = center_to_origin.dot(&ray.direction());
        let c = center_to_origin.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let hit_p = ray.at(root);
        let outward_normal = (hit_p - self.center(ray.time())) / self.radius;
        let (u, v) = Sphere::get_sphere_uv(&outward_normal);
        let mut hit_rec = HitRecord {
            t: root,
            point: hit_p,
            normal: Vec3::default(),
            front_face: false,
            material: Some(self.material.clone()),
            u,
            v,
        };
        hit_rec.set_face_normal(ray, &outward_normal);

        Some(hit_rec)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        let box_0 = Aabb::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );

        let box_1 = Aabb::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(Aabb::surrounding_box(&box_0, &box_1))
    }
}
