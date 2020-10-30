use crate::math::{Point3, Ray};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center: center,
            radius: radius,
        }
    }

    pub fn intersects_ray(&self, ray: &Ray) -> f64 {
        ray.intersects_sphere(self)
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}
impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Point3::default(),
            radius: 1.0,
        }
    }
}
