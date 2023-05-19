use crate::{ray::Ray, vec3::Vec3};

/// Stores information about a hit between a ray and some object.
pub struct HitRecord {
    /// The time of the hit.
    pub t: f64,

    /// The point of contact between the object and ray.
    pub p: Vec3,

    /// The normal vector at the point of the hit.
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: f64::INFINITY,
            p: Vec3::ZERO,
            normal: Vec3::ZERO,
        }
    }

    pub fn correct_normal_direction(&mut self, ray: Ray) {
        if self.normal.dot(ray.direction) > 0.0 {
            self.normal = -self.normal;
        }
    }
}
