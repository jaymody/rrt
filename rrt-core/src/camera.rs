use crate::{ray::Ray, vec3::Vec3};

/// The camera determines how and where we look at the rendered scene.
/// Reference: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays.html
pub struct Camera {
    /// The location of the camera center (i.e. the "eye")
    eye: Vec3,

    /// The direction the camera is facing (direction, up, and right form an orthogonal basis).
    /// The magnitude of this vector is equal to the focal length.
    direction: Vec3,

    /// The direction of up (direction, up, and right form an orthogonal basis).
    up: Vec3,

    /// The direction of right (direction, up, and right form an orthogonal basis).
    right: Vec3,

    /// Field of view (in degrees).
    fov: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(Vec3::ZERO, -Vec3::Z, Vec3::Y, 90.0)
    }
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, view_up: Vec3, fov: f64) -> Camera {
        assert!(fov > 0.0 && fov < 180.0);
        let eye = look_from;
        let direction = (look_at - look_from).unit_vector();
        let right = direction.cross(view_up).unit_vector();
        let up = right.cross(direction);
        Camera {
            eye,
            direction,
            up,
            right,
            fov,
        }
    }

    /// Gets the ray coming from the eye of the camera to the viewport at
    /// position a-b from left-to-right then up-to-down. The inputs a and b must
    /// be normalized to the range [-1, 1]. As such (-1, -1) would be the top
    /// left corner of the viewport, (-1, 1) would be the top right corner,
    /// (1, -1) would be the bottom-left corner, (1, 1) would be the bottom
    /// right corner, and (0, 0) would be the center.
    pub fn cast_ray(&self, a: f64, b: f64) -> Ray {
        let scale = (self.fov.to_radians() / 2.0).tan();
        Ray::new(
            self.eye,
            self.direction + b * self.right * scale - a * self.up * scale,
        )
    }
}
