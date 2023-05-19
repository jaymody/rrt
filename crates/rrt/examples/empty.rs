use rrt::engine::Engine;

fn main() {
    let engine = Engine::new();
    std::fs::write("./output.ppm", engine.render().to_ppm()).unwrap();
}
