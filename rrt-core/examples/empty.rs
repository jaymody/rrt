use rrt_core::{engine, scene::Scene};

fn main() {
    let scene = Scene::new();
    std::fs::write(
        "./output.ppm",
        engine::render(&scene, 800, 450, 10, 5).to_ppm(),
    )
    .unwrap();
}
