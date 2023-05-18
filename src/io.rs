use std::ops::{Deref, DerefMut};

use crate::color::Color;

/// Stores the rendered image and provides utility methods to convert it to
/// various output formats.
pub struct Buffer {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(arr: Vec<Color>, width: usize, height: usize) -> Self {
        assert!(arr.len() == width * height);
        Buffer {
            pixels: arr,
            width,
            height,
        }
    }

    pub fn to_ppm(&self) -> String {
        let mut s = String::new();
        s.push_str("P3\n");
        s.push_str(&format!("{} {}\n", self.width, self.height));
        s.push_str("255\n");
        for color in &self.pixels {
            let (r, g, b) = color.to_u8();
            s.push_str(&format!("{} {} {}\n", r, g, b));
        }
        s
    }
}

impl Deref for Buffer {
    type Target = [Color];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.pixels.as_ptr(), self.pixels.len()) }
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.pixels.as_mut_ptr(), self.pixels.len()) }
    }
}
