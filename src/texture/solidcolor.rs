use super::Texture;
use crate::math::{Color, Point3};
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
