//! Scene object types that implement [`Hittable`](crate::hittable::Hittable)

mod aabox;
mod plane;
mod sphere;

pub use aabox::*;
pub use plane::*;
pub use sphere::*;
