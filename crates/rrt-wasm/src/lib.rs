use js_sys::Uint8ClampedArray;
use rrt::{
    color::Color, engine::Engine, material::Lambertian, object::Object, scene::Scene,
    shape::Sphere, vec3::Vec3,
};
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn render(width: u32, height: u32, num_samples: u32, max_bounces: u32) -> Uint8ClampedArray {
    // set the panic hook to log panics to the js console
    console_error_panic_hook::set_once();

    // create the scene and it's objects
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

    // render the scene
    let engine = Engine::new()
        .scene(scene)
        .width(width as usize)
        .height(height as usize)
        .num_samples(num_samples as usize)
        .max_bounces(max_bounces as usize);
    let pixels = engine.render().pixels;

    // store the
    let arr = Uint8ClampedArray::new_with_length(width * height * 4);
    for (i, color) in pixels.iter().enumerate() {
        let idx = (i * 4) as u32;
        let (r, g, b) = color.to_u8();
        arr.set_index(idx + 0, r);
        arr.set_index(idx + 1, g);
        arr.set_index(idx + 2, b);
        arr.set_index(idx + 3, 255); //alpha channel should be fully opaque
    }
    arr
}
