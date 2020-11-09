use crate::hittable::*;
use crate::math::{Onb, Point3, Vec3};
use rand::{thread_rng, Rng};
use std::sync::Arc;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}
impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: Onb::new(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = direction.unit().dot(&self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&Vec3::random_cosine_direction())
    }
}

pub struct HittablePdf {
    hittable: Arc<dyn Hittable>,
    o: Point3,
}

impl HittablePdf {
    pub fn new(hittable: Arc<dyn Hittable>, o: Point3) -> Self {
        Self { hittable, o }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.hittable.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.hittable.random(&self.o)
    }
}

pub struct MixturePdf {
    pdfs: [Arc<dyn Pdf>; 2],
}

impl MixturePdf {
    pub fn new(pdf0: Arc<dyn Pdf>, pdf1: Arc<dyn Pdf>) -> Self {
        Self { pdfs: [pdf0, pdf1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.pdfs[0].value(direction) + 0.5 * self.pdfs[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if thread_rng().gen::<f64>() < 0.5 {
            self.pdfs[0].generate()
        } else {
            self.pdfs[1].generate()
        }
    }
}
