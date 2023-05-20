use core::num;

use js_sys::{Uint8ClampedArray, WebAssembly};
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::*;
use rrt::{color::Color, engine::Engine};
use wasm_bindgen::prelude::*;

use crate::utils::{default_scene, log};

#[wasm_bindgen]
pub struct Image {
    arr: Vec<u8>,
    buf: Vec<Color>,
    width: u32,
    height: u32,
    max_bounces: u32,
    total_samples: u32,
    engine: Engine,
}

#[wasm_bindgen]
impl Image {
    pub fn new(width: u32, height: u32, max_bounces: u32) -> Self {
        Image {
            arr: vec![255; (width * height * 4) as usize],
            buf: vec![Color::BLACK; (width * height) as usize],
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

    pub fn render(&mut self, num_samples: u32) {
        self.total_samples += num_samples;

        self.engine.num_samples = num_samples as usize;
        let pixels = self.engine.render().pixels;

        for (i, color) in pixels.into_iter().enumerate() {
            // TODO: we need to multiply color by num_samples since render
            // takes the average for us, we should probably have a separate
            // rendering function to return an iterator that returns the raw sum
            // as well as being an iterator means we don't need to collect to
            // a vector first
            self.buf[i] = self.buf[i] + color * num_samples as f64;
            let (r, g, b) = (self.buf[i] / self.total_samples as f64).to_u8();
            self.arr[i * 4 + 0] = r;
            self.arr[i * 4 + 1] = g;
            self.arr[i * 4 + 2] = b;
        }
    }

    pub fn arr_len(&self) -> usize {
        self.arr.len()
    }

    pub fn get_image_so_far(&self) -> Uint8ClampedArray {
        // a bit of a hack
        // see: https://github.com/rustwasm/wasm-bindgen/blob/85f72c912577fca98d9c23ef486405cd43770813/examples/raytrace-parallel/src/lib.rs#L141-L157
        let ptr = self.get_ptr();
        let mem = wasm_bindgen::memory().unchecked_into::<WebAssembly::Memory>();
        Uint8ClampedArray::new(&mem.buffer())
            .slice(ptr as u32, (ptr as usize + self.arr_len()) as u32)
    }
}
