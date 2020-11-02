use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::math::{Aabb, Ray};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::sync::Arc;
pub struct BvhNode {
    left: Option<Box<BvhNode>>,
    right: Option<Box<BvhNode>>,
    hittable: Option<Arc<dyn Hittable>>,
    aabb: Aabb,
}

impl BvhNode {
    pub fn from_list(list: &HittableList, t0: f64, t1: f64) -> Self {
        Self::new(list.objects(), 0, list.objects().len(), t0, t1)
    }

    pub fn new(
        src_objects: &Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        t0: f64,
        t1: f64,
    ) -> Self {
        Self::new_impl(&mut src_objects.clone(), start, end, t0, t1)
    }

    fn new_impl(
        objects: &mut [std::sync::Arc<dyn Hittable>],
        start: usize,
        end: usize,
        t0: f64,
        t1: f64,
    ) -> Self {
        let object_span = end - start;

        let left: Box<BvhNode>;
        let right: Box<BvhNode>;
        if object_span == 1 {
            let hittable = objects[start].clone();
            return Self {
                left: None,
                right: None,
                aabb: hittable.bounding_box(t0, t1).unwrap(),
                hittable: Some(hittable),
            };
        } else {
            sort_span(&mut objects[start..end]);

            if object_span == 2 {
                left = Box::new(BvhNode {
                    left: None,
                    right: None,
                    hittable: Some(objects[start].clone()),
                    aabb: objects[start].bounding_box(t0, t1).unwrap(),
                });
                right = Box::new(BvhNode {
                    left: None,
                    right: None,
                    hittable: Some(objects[start + 1].clone()),
                    aabb: objects[start + 1].bounding_box(t0, t1).unwrap(),
                });
            } else {
                let mid = start + object_span / 2;

                left = Box::new(Self::new_impl(objects, start, mid, t0, t1));
                right = Box::new(Self::new_impl(objects, mid, end, t0, t1));
            }
        }

        let box_left = left.aabb;
        let box_right = right.aabb;

        Self {
            left: Some(left),
            right: Some(right),
            hittable: None,
            aabb: Aabb::surrounding_box(&box_left, &box_right),
        }
    }

    pub fn left(&self) -> &Option<Box<BvhNode>> {
        &self.left
    }

    pub fn right(&self) -> &Option<Box<BvhNode>> {
        &self.right
    }

    pub fn len(&self) -> usize {
        let left = if let Some(ref node) = self.left {
            node.len()
        } else {
            0
        };
        let right = if let Some(ref node) = self.right {
            node.len()
        } else {
            0
        };

        left + right + if self.hittable.is_some() { 1 } else { 0 }
    }
}

fn sort_span(span: &mut [Arc<dyn Hittable>]) {
    match thread_rng().gen_range(0, 3) {
        0 => span.sort_by(box_compare_x),
        1 => span.sort_by(box_compare_y),
        2 => span.sort_by(box_compare_z),
        _ => panic!(),
    };
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0).unwrap();
    let box_b = b.bounding_box(0.0, 0.0).unwrap();

    box_a.min()[axis].partial_cmp(&box_b.min()[axis]).unwrap()
}
fn box_compare_x(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}
fn box_compare_y(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}
fn box_compare_z(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        if self.hittable.is_some() {
            return self.hittable.as_ref().unwrap().hit(ray, t_min, t_max);
        }

        let hit_left = if let Some(ref left) = self.left {
            left.hit(ray, t_min, t_max)
        } else {
            None
        };
        let hit_right = if let Some(ref right) = self.right {
            right.hit(ray, t_min, t_max)
        } else {
            None
        };

        if hit_left.is_some() && hit_right.is_some() {
            return if hit_left.as_ref().unwrap().t < hit_right.as_ref().unwrap().t {
                hit_left
            } else {
                hit_right
            };
        } else {
            hit_left.or(hit_right)
        }
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        Some(self.aabb)
    }
}

impl std::fmt::Display for BvhNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BvhNode {{ left: {}, right: {}, hittable:{}, aabb: {:?}, len: {} }} ",
            if self.left.is_some() { "Some" } else { "None" },
            if self.right.is_some() { "Some" } else { "None" },
            if self.hittable.is_some() {
                "Some"
            } else {
                "None"
            },
            self.aabb,
            self.len()
        )
    }
}
