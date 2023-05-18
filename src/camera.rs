use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    // The location of the camera center (i.e. the "eye")
    eye: Vec3,

    // The direction the camera is facing (direction, up, and right form an orthogonal basis).
    // The magnitude of this vector is equal to the focal length.
    direction: Vec3,

    // The direction of up (direction, up, and right form an orthogonal basis).
    // The magnitude of this vector is equal to 1/2 the viewport height.
    up: Vec3,

    // The direction of right (direction, up, and right form an orthogonal basis).
    // The magnitude of this vector is equal to 1/2 the viewport width.
    right: Vec3,
}

impl Camera {
    pub fn new(width: usize, height: usize) -> Self {
        // As convention, the y-axis is up, the x-axis is left-right
        // and the z axis is the depth where usually, we are looking into
        // the negative area of the z-axis.
        //
        // TODO: For now, we'll just hardcode some values for the
        // camera, of course, respecting the aspect ratio of the
        // image we're gonna create.
        let aspect_ratio = width as f64 / height as f64;

        let viewport_height = 1.0;
        let viewport_width = aspect_ratio;
        let focal_length = 1.0;

        let eye = Vec3::ZERO;
        let direction = Vec3::Z * focal_length;
        let up = Vec3::Y * viewport_height / 2.0;
        let right = Vec3::X * viewport_width / 2.0;

        Camera {
            eye,
            direction,
            up,
            right,
        }
    }

    /// Gets the ray coming from the eye of the camera to the viewport at
    /// position a-b from left-to-right then up-to-down. The inputs a and b must
    /// be normalized to the range [-1, 1]. As such (-1, -1) would be the top
    /// left corner of the viewport, (-1, 1) would be the top right corner,
    /// (1, -1) would be the bottom-left corner, (1, 1) would be the bottom
    /// right corner, and (0, 0) would be the center.
    pub fn get_ray(&self, a: f64, b: f64) -> Ray {
        Ray::new(self.eye, self.direction + b * self.right + a * -self.up)
    }
}
