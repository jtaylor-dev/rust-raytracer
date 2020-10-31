use crate::math::{Point3, Ray};

#[derive(Debug, Default, Copy, Clone)]
pub struct Aabb {
    min: Point3,
    max: Point3,
}

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Aabb {
        Self { min, max }
    }

    pub fn min(&self) -> Point3 {
        self.min
    }

    pub fn max(&self) -> Point3 {
        self.max
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let (r_origin, r_direction) = (ray.origin(), ray.direction());
        for a in 0..3 {
            let t0 = ((self.min[a] - r_origin[a]) / r_direction[a])
                .min((self.max[a] - r_origin[a]) / r_direction[a]);
            let t1 = ((self.min[a] - r_origin[a]) / r_direction[a])
                .max((self.max[a] - r_origin[a]) / r_direction[a]);
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box_0: &Aabb, box_1: &Aabb) -> Aabb {
        let small = Point3::new(
            box_0.min().x().min(box_1.min().x()),
            box_0.min().y().min(box_1.min().y()),
            box_0.min().z().min(box_1.min().z()),
        );
        let big = Point3::new(
            box_0.max().x().max(box_1.max().x()),
            box_0.max().y().max(box_1.max().y()),
            box_0.max().z().max(box_1.max().z()),
        );
        Aabb::new(small, big)
    }
}
