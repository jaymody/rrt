use crate::{ray::Ray, vec3::Vec3};

/// Stores information about a hit between a ray and some object.
pub struct HitRecord {
    /// The incoming ray.
    pub ray: Ray,

    /// Point of impact.
    pub p: Vec3,

    /// Normal vector between the ray and tangential plane at the point of impact.
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(ray: Ray, p: Vec3, normal: Vec3) -> Self {
        HitRecord { ray, p, normal }
    }
}
