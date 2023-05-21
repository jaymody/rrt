use rrt_core::{camera::Camera, engine, scene::Scene};

fn main() {
    let scene = Scene::new();
    let camera = Camera::default();
    std::fs::write(
        "./output.ppm",
        engine::render(&scene, &camera, 800, 450, 10, 5).to_ppm(),
    )
    .unwrap();
}
