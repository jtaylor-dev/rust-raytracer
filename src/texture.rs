use crate::math::{Color, Point3};
use std::sync::Arc;

pub trait Texture: Sync + Send {
    fn sample(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> Self {
        Self { value: c }
    }
}

impl Texture for SolidColor {
    #[allow(unused_variables)]
    fn sample(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.value
    }
}

impl From<Color> for SolidColor {
    fn from(c: Color) -> Self {
        Self {
            value: Color::new(c[0], c[1], c[2]),
        }
    }
}

pub struct CheckerPattern {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerPattern {
    pub fn from_textures(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self { even, odd }
    }

    pub fn from_colors(even: Color, odd: Color) -> Self {
        Self {
            even: Arc::new(SolidColor::from(even)),
            odd: Arc::new(SolidColor::from(odd)),
        }
    }
}

impl Texture for CheckerPattern {
    fn sample(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.sample(u, v, p)
        } else {
            self.even.sample(u, v, p)
        }
    }
}
