use rrt::engine::Engine;
use wasm_bindgen::prelude::*;

use crate::utils::{default_scene, log};

#[wasm_bindgen]
pub struct Image {
    arr: Vec<u8>,
    width: u32,
    height: u32,
    max_bounces: u32,
    total_samples: u32,
    engine: Engine,
}

#[wasm_bindgen]
impl Image {
    pub fn new(width: u32, height: u32, max_bounces: u32) -> Self {
        log("Logging form inside rust! Creating image ...");
        Image {
            arr: vec![0; (width * height * 4) as usize],
            width: width,
            height: height,
            max_bounces: max_bounces,
            total_samples: 0,
            engine: Engine::new()
                .scene(default_scene())
                .width(width as usize)
                .height(height as usize)
                .max_bounces(max_bounces as usize),
        }
    }

    pub fn get_ptr(&self) -> *const u8 {
        self.arr.as_ptr()
    }

    pub fn Image(&mut self, num_samples: u32) {
        self.total_samples += 50;
        self.arr.fill(self.total_samples.clamp(0, 255) as u8);
        // let pixels = (0..self.height)
        //     .into_par_iter()
        //     .flat_map(|i| {
        //         (0..self.width).into_par_iter().map(move |j| {
        //             (0..self.num_samples)
        //                 .into_par_iter()
        //                 .map(|_| self.trace_ray(self.get_ray_from_ij(i, j), 0))
        //                 .reduce(|| Color::BLACK, |acc, e| acc + e)
        //                 / self.num_samples as f64
        //         })
        //     })
        //     .collect();
    }

    pub fn arr_len(&self) -> usize {
        self.arr.len()
    }
}
