use crate::math::{Point3, Sphere, Vec3};

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    /// Returns a `Point3` a distance `t` along the `Ray`.
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    /// Returns true if the `Ray` intersects the `Sphere` at least once.
    pub fn intersects_sphere(&self, s: &Sphere) -> f64 {
        let center_to_origin = self.origin - s.center();
        let a = self.direction.length_squared();
        let half_b = center_to_origin.dot(&self.direction);
        let c = center_to_origin.length_squared() - s.radius() * s.radius();
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}
