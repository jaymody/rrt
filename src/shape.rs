use crate::{hitrecord::HitRecord, ray::Ray};

/// A shape defines the math that determines if a given a ray hits it.
pub trait Shape {
    fn intersect(&self, ray: Ray) -> Option<HitRecord>;
}
