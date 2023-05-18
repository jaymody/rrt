use crate::{camera::Camera, io::Buffer, scene::Scene, vec3::Vec3};

pub struct Engine {
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
            camera: Camera {},
            width: 800,
            height: 600,
            num_samples: 16,
            max_bounces: 10,
        }
    }

    pub fn render(&self, scene: &Scene) -> Buffer {
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
