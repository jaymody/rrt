mod sphere;

pub use sphere::Sphere;

use crate::{hitrecord::HitRecord, ray::Ray};

const T_MIN: f64 = 0.001;

/// A shape is as a mathematical model for which we can compute the intersection
/// with an array.
pub trait Shape: Sync {
    fn intersect(&self, ray: Ray, record: &mut HitRecord) -> bool;
}
