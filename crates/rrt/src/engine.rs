use indicatif::ParallelProgressIterator;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::*;

use crate::{
    camera::Camera, color::Color, io::Buffer, ray::Ray, scene::Scene, utils::random_double,
};

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
        const WIDTH: usize = 800;
        const HEIGHT: usize = 450;

        Engine {
            scene: Scene::new(),
            camera: Camera::new(WIDTH, HEIGHT),
            width: WIDTH,
            height: HEIGHT,
            num_samples: 100,
            max_bounces: 50,
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

    /// Render without the use of indicatif or rayon, so it's compatible with
    /// web assembly.
    pub fn render(&self) -> Buffer {
        let pixels = (0..self.height)
            .flat_map(|i| {
                (0..self.width).map(move |j| {
                    (0..self.num_samples)
                        .map(|_| self.trace_ray(self.get_ray_from_ij(i, j), 0))
                        .reduce(|acc, e| acc + e)
                        .unwrap()
                        / self.num_samples as f64
                })
            })
            .collect();
        Buffer::new(pixels, self.width, self.height)
    }

    pub fn render_parallel(&self) -> Buffer {
        let pixels = (0..self.height)
            .into_par_iter()
            // .progress()
            .flat_map(|i| {
                (0..self.width).into_par_iter().map(move |j| {
                    (0..self.num_samples)
                        .into_par_iter()
                        .map(|_| self.trace_ray(self.get_ray_from_ij(i, j), 0))
                        .reduce(|| Color::BLACK, |acc, e| acc + e)
                        / self.num_samples as f64
                })
            })
            .collect();
        Buffer::new(pixels, self.width, self.height)
    }

    fn get_ray_from_ij(&self, i: usize, j: usize) -> Ray {
        // Normalize i and j to the range [-1, 1] and get the ray pointing
        // at the given pixel.
        let get_a = |i| ((i as f64 + random_double()) / (self.height - 1) as f64) * 2.0 - 1.0;
        let get_b = |j| ((j as f64 + random_double()) / (self.width - 1) as f64) * 2.0 - 1.0;
        self.camera.get_ray(get_a(i), get_b(j))
    }

    fn trace_ray(&self, ray: Ray, num_bounces: usize) -> Color {
        // If we've exceeded the max number of bounces, return no light.
        if num_bounces > self.max_bounces {
            return Color::BLACK;
        }

        match self.scene.hit_closest_object(ray) {
            // If we hit something, trace the outgoing ray and multiply
            // it's color by the attenuation of the current hit.
            Some((outgoing_ray, attenuation)) => {
                attenuation * self.trace_ray(outgoing_ray, num_bounces + 1)
            }
            // If we haven't hit anything, return the light from the background.
            None => self.background_light(ray),
        }
    }

    fn background_light(&self, ray: Ray) -> Color {
        // We get background light from our scene via a sky that is blue
        // and a void below that is white (and a nice gradient for everything in
        // between).
        //
        // We can achieve this by taking the y component of the ray direction,
        // normalizing it to a value between 0 and 1 and use that to blend the
        // white and sky blue. To normalize the y component to 0 and 1, we take
        // the y component of the associated unit vector of the ray direction.
        let t = (ray.direction.unit_vector().y + 1.0) / 2.0;
        Color::WHITE * (1.0 - t) + Color::SKY_BLUE * t
    }
}
