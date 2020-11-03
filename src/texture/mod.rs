use crate::math::{Color, Point3};
pub trait Texture: Sync + Send {
    fn sample(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub mod checker;
pub use checker::*;

pub mod solidcolor;
pub use solidcolor::*;

pub mod perlin;
pub use perlin::*;
