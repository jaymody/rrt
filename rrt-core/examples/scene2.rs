use rrt_core::{
    color::Color, engine, material::Lambertian, object::Object, scene::Scene, shape::Sphere,
    vec3::Vec3,
};

fn main() {
    let mut scene = Scene::new();
    let sphere = Object::new(
        Box::new(Sphere::new(0.5, Vec3::new(0., 0., -1.))),
        Box::new(Lambertian::new(Color::RED)),
    );
    let ground = Object::new(
        Box::new(Sphere::new(100.0, Vec3::new(0.0, -100.5, -1.0))),
        Box::new(Lambertian::new(Color::GREEN)),
    );
    scene.add_object(sphere);
    scene.add_object(ground);

    std::fs::write(
        "./output.ppm",
        engine::render(&scene, 800, 450, 250, 50).to_ppm(),
    )
    .unwrap();
}
