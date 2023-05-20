use rayon::prelude::IntoParallelIterator;
use rayon::prelude::*;

use crate::{
    camera::Camera, color::Color, io::Buffer, ray::Ray, scene::Scene, utils::random_double,
};

/// Renders a scene. The algorithm for rendering a scene works roughly as such:
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
/// # Arguments
///
/// * `scene` - The scene to render.
/// * `width` - Image width.
/// * `height` - Image height.
/// * `num_samples` - Number of rays to samples per pixel.
/// * `max_bounces` - Max number of bounces for a given ray.
pub fn render(
    scene: &Scene,
    width: usize,
    height: usize,
    num_samples: usize,
    max_bounces: usize,
) -> Buffer {
    let camera = Camera::new(width, height);

    let rand_and_norm = |x, size| ((x as f64 + random_double()) / (size - 1) as f64) * 2.0 - 1.0;
    let cast_ray = |i, j| camera.cast_ray(rand_and_norm(i, height), rand_and_norm(j, width));

    let pixels = (0..height)
        .into_par_iter()
        // .progress()
        .flat_map(|i| {
            (0..width).into_par_iter().map(move |j| {
                (0..num_samples)
                    .into_par_iter()
                    .map(|_| trace_ray(cast_ray(i, j), max_bounces + 1, scene))
                    .reduce(|| Color::BLACK, |acc, e| acc + e)
                    / num_samples as f64
            })
        })
        .collect();
    Buffer::new(pixels, width, height)
}

fn trace_ray(ray: Ray, bounces_left: usize, scene: &Scene) -> Color {
    // If we've exceeded the max number of bounces, return no light.
    if bounces_left == 0 {
        return Color::BLACK;
    }

    match scene.hit_closest_object(ray) {
        // If we hit something, trace the outgoing ray and multiply
        // it's color by the attenuation of the current hit.
        Some((outgoing_ray, attenuation)) => {
            attenuation * trace_ray(outgoing_ray, bounces_left - 1, scene)
        }
        // If we haven't hit anything, return the light from the background.
        None => scene.get_environment_light(ray),
    }
}
