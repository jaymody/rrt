use js_sys::Promise;
use rrt::{
    color::Color, material::Lambertian, object::Object, scene::Scene, shape::Sphere, vec3::Vec3,
};

use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn initialize(num_threads: usize, set_panic_hook: bool) -> Promise {
    if set_panic_hook {
        console_error_panic_hook::set_once();
    }
    wasm_bindgen_rayon::init_thread_pool(num_threads)
}

pub fn default_scene() -> Scene {
    let mut scene = Scene::new();
    let sphere = Object::new(
        Box::new(Sphere::new(0.5, Vec3::new(0., 0., -1.))),
        Box::new(Lambertian::new(Color::WHITE * 0.5)),
    );
    let ground = Object::new(
        Box::new(Sphere::new(100.0, Vec3::new(0.0, -100.5, -1.0))),
        Box::new(Lambertian::new(Color::WHITE * 0.5)),
    );
    scene.add_object(sphere);
    scene.add_object(ground);
    scene
}

pub fn f64_to_u8(num: f64) -> u8 {
    // TODO: move this to rrt crate so we don't repeat logic
    (256. * num.sqrt().clamp(0., 0.999)) as u8
}
