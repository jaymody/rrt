use js_sys::{Uint8ClampedArray, WebAssembly};
use rrt_core::{
    camera::Camera, color::Color, engine, material::Lambertian, object::Object, scene::Scene,
    shape::Sphere, vec3::Vec3,
};
use wasm_bindgen::prelude::*;

pub fn default_scene() -> Scene {
    let mut scene = Scene::new();
    let sphere_left = Object::new(
        Box::new(Sphere::new(0.10, Vec3::new(-0.35, 0., 0.))),
        Box::new(Lambertian::new(Color::RED)),
    );
    let sphere_middle = Object::new(
        Box::new(Sphere::new(0.25, Vec3::new(0., 0., 0.))),
        Box::new(Lambertian::new(Color::GREEN)),
    );
    let sphere_right = Object::new(
        Box::new(Sphere::new(0.10, Vec3::new(0.35, 0., 0.))),
        Box::new(Lambertian::new(Color::BLUE)),
    );
    let ground = Object::new(
        Box::new(Sphere::new(100.0, Vec3::new(0.0, -100.25, 0.0))),
        Box::new(Lambertian::new(Color::WHITE * 0.5)),
    );
    scene.add_object(sphere_left);
    scene.add_object(sphere_middle);
    scene.add_object(sphere_right);
    scene.add_object(ground);
    scene
}

#[wasm_bindgen]
pub struct Image {
    /// RGBA array containing the actual pixel values we'd like to show.
    arr: Vec<u8>,

    /// Array containing the raw color calculations per pixel (flattened row-wise).
    buf: Vec<Color>,

    /// Width of the rendered image.
    width: usize,

    /// Height of the rendered image.
    height: usize,

    /// Keeps track of the total number of samples we have thus far drawn.
    total_samples: usize,

    /// The scene. The "center" of the scene should be the origin (which is
    /// what the camera will rotate around).
    scene: Scene,

    /// The camera.
    camera: Camera,

    /// Distance from camera to origin.
    camera_distance: f64,
}

#[wasm_bindgen]
impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            arr: vec![255; width * height * 4],
            buf: vec![Color::BLACK; width * height],
            width: width,
            height: height,
            total_samples: 0,
            scene: default_scene(),
            camera: Camera::default(),
            camera_distance: 1.0,
        }
    }

    pub fn set_camera(&mut self, x_rot: f64, y_rot: f64, fov: f64) {
        let mut direction = Vec3::Z;
        direction = direction.rotate_about_x_axis(x_rot);
        direction = direction.rotate_about_y_axis(y_rot);
        direction = direction.unit_vector() * self.camera_distance;
        self.camera = Camera::new(-direction, Vec3::ZERO, Vec3::Y, fov);
    }

    pub fn get_ptr(&self) -> *const u8 {
        self.arr.as_ptr()
    }

    pub fn render(&mut self, num_samples: usize, max_bounces: usize) {
        self.total_samples += num_samples;

        let pixels = engine::render(
            &self.scene,
            &self.camera,
            self.width,
            self.height,
            num_samples,
            max_bounces,
        )
        .pixels;

        for (i, color) in pixels.into_iter().enumerate() {
            // TODO: we need to multiply color by num_samples since render
            // takes the average for us, we should probably have a separate
            // rendering function to return an iterator that returns the raw sum
            // as well as being an iterator means we don't need to collect to
            // a vector first
            self.buf[i] = self.buf[i] + color * num_samples as f64;
            let (r, g, b) = (self.buf[i] / self.total_samples as f64).to_u8();
            self.arr[i * 4 + 0] = r;
            self.arr[i * 4 + 1] = g;
            self.arr[i * 4 + 2] = b;
        }
    }

    pub fn clear(&mut self) {
        self.buf.fill(Color::BLACK);
        self.total_samples = 0;
    }

    pub fn get_image_so_far(&self) -> Uint8ClampedArray {
        // a bit of a hack
        // see: https://github.com/rustwasm/wasm-bindgen/blob/85f72c912577fca98d9c23ef486405cd43770813/examples/raytrace-parallel/src/lib.rs#L141-L157
        let ptr = self.get_ptr();
        let mem = wasm_bindgen::memory().unchecked_into::<WebAssembly::Memory>();
        Uint8ClampedArray::new(&mem.buffer())
            .slice(ptr as u32, (ptr as usize + self.arr.len()) as u32)
    }
}
