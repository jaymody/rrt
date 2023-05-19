use rrt::{
    color::Color, engine::Engine, material::Lambertian, object::Object, scene::Scene,
    shape::Sphere, vec3::Vec3,
};

fn main() {
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

    let engine = Engine::new().scene(scene);

    std::fs::write("./output.ppm", engine.render().to_ppm()).unwrap();
}
