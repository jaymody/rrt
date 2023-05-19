use crate::vec3::Vec3;

/// A ray represents a line in space.
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    /// The origin point of the ray.
    pub origin: Vec3,

    /// The direction of the ray.
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    /// Returns the point along the line drawn by the ray according to the
    /// parametric equation `origin + t * direction`.
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
