use rrt::{Engine, Scene};

fn main() {
    let engine = Engine::new();
    let scene = Scene::new();
    let buffer = engine.render(&scene);
    std::fs::write("./output.ppm", buffer.to_ppm()).unwrap();
}
