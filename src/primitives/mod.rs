//! Scene object types that implement [`Hittable`](crate::hittable::Hittable)

pub mod plane;
mod sphere;

pub use plane::*;
pub use sphere::*;
