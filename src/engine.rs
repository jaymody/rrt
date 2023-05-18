use crate::{camera::Camera, io::Buffer, scene::Scene, vec3::Vec3};

pub struct Engine {
    // the scene to render
    pub scene: Scene,

    // camera
    pub camera: Camera,

    // image width
    pub width: usize,

    // image height
    pub height: usize,

    // number of samples per pixel
    pub num_samples: usize,

    // max number of bounces for a given ray
    pub max_bounces: usize,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            scene: Scene::new(),
            camera: Camera {},
            width: 800,
            height: 600,
            num_samples: 16,
            max_bounces: 10,
        }
    }

    pub fn scene(mut self, scene: Scene) -> Self {
        self.scene = scene;
        self
    }

    pub fn camera(mut self, camera: Camera) -> Self {
        self.camera = camera;
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    pub fn num_samples(mut self, num_samples: usize) -> Self {
        self.num_samples = num_samples;
        self
    }

    pub fn max_bounces(mut self, max_bounces: usize) -> Self {
        self.max_bounces = max_bounces;
        self
    }

    pub fn render(&self) -> Buffer {
        let mut buffer = Buffer::new(self.width, self.height);
        for i in 0..self.height {
            for j in 0..self.width {
                buffer[i * self.width + j] = Vec3(
                    i as f64 / self.height as f64,
                    j as f64 / self.width as f64,
                    0.,
                );
            }
        }
        buffer
    }
}
