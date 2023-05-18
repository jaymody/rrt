use std::ops::Mul;

use crate::{camera::Camera, color::Color, io::Buffer, scene::Scene};

/// The engine handles actually rendering an image given all the necessary
/// information. The algorithm here is:
///
///   1) The camera casts rays out into the scene (num_samples rays per pixels).
///   2) If a ray hits an object in our scene, we compute the attenuation (how
///      light is absorbed by the object) and the direction of the outgoing array.
///   3) We then repeat step 2) for a ray that hits an object until the ray no
///      longer hits the object or until we have reached max_bounces steps.
///   4) If a ray ends before reaching max_bounces, we compute the color based
///      on our background which is providing us ambient light (blue for the
///      above sky white for the void below). We then go in reverse multiplying
///      the light color by the attenuation of each object that it hit, until
///      we get back to our camera, which determines the pixel's color. If the
///      ray reached max_bounces, we say the color of the ray is just black (no
///      light).
///   5) Since there are num_sample rays per pixel, we take the average of them
///       to determine the final color of a pixel.
///
pub struct Engine {
    /// The scene to render.
    pub scene: Scene,

    /// The camera.
    pub camera: Camera,

    /// Image width.
    pub width: usize,

    /// Image height.
    pub height: usize,

    /// Number of rays to samples per pixel.
    pub num_samples: usize,

    /// Max number of bounces for a given ray.
    pub max_bounces: usize,
}

impl Engine {
    pub fn new() -> Self {
        // TODO: for now, we are hard coding these values but later we'll want
        // to let the user determine these values
        const WIDTH: usize = 400;
        const HEIGHT: usize = 300;

        Engine {
            scene: Scene::new(),
            camera: Camera::new(WIDTH, HEIGHT),
            width: WIDTH,
            height: HEIGHT,
            num_samples: 16,
            max_bounces: 10,
        }
    }

    pub fn scene(mut self, scene: Scene) -> Self {
        self.scene = scene;
        self
    }

    pub fn camera(mut self, camera: Camera) -> Self {
        self.camera = camera;
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    pub fn num_samples(mut self, num_samples: usize) -> Self {
        self.num_samples = num_samples;
        self
    }

    pub fn max_bounces(mut self, max_bounces: usize) -> Self {
        self.max_bounces = max_bounces;
        self
    }

    pub fn render(&self) -> Buffer {
        let pixels = (0..self.height)
            .flat_map(|i| (0..self.width).map(move |j| self.render_pixel(i, j)))
            .collect();
        Buffer::new(pixels, self.width, self.height)
    }

    fn render_pixel(&self, i: usize, j: usize) -> Color {
        // Normalize i and j to the range [-1, 1].
        let a = (i as f64 / (self.height - 1) as f64) * 2.0 - 1.0;
        let b = (j as f64 / (self.width - 1) as f64) * 2.0 - 1.0;
        let ray = self.camera.get_ray(a, b);

        // By default, we want the effect that any ray that is pointing up into
        // the sky is blue, and pointing down into the ground (void) is white,
        // and have a nice gradient for everything in between.
        //
        // We can achieve this by taking the y component of the ray direction,
        // normalizing it to a value between 0 and 1 and use that to blend the
        // white and sky blue. To normalize the y component to 0 and 1, we take
        // the y component of the associated unit vector of the ray direction.
        //
        // Color the sky blue and the void white
        let t = (ray.direction.unit_vector().y + 1.0) / 2.0;
        let color = Color::WHITE.mul(1.0 - t) + Color::SKY_BLUE.mul(t);
        color
    }
}
