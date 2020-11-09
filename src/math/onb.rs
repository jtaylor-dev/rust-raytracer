use super::Vec3;
use std::ops::Index;

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new(n: &Vec3) -> Self {
        let mut axis = [Vec3::default(); 3];
        axis[2] = n.unit();
        let a = if axis[2].x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        axis[1] = axis[2].cross(&a).unit();
        axis[0] = axis[2].cross(&axis[1]);
        Self { axis }
    }

    pub fn u(&self) -> Vec3 {
        self[0]
    }

    pub fn v(&self) -> Vec3 {
        self[1]
    }

    pub fn w(&self) -> Vec3 {
        self[2]
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u() + b * self.v() + c * self.w()
    }

    pub fn local_vec(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }
}

impl Index<usize> for Onb {
    type Output = Vec3;
    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}
