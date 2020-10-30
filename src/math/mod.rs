//! A collection of useful math types.
//!
//! These types are purely mathematical, and do not implement high-level scene abstractions such as [`Hittable`](crate::hittable::Hittable).

mod vec3;
pub use vec3::*;

mod ray;
pub use ray::*;

mod sphere;
pub use sphere::*;
