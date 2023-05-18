use std::ops::{Deref, DerefMut};

use crate::{color::Color, vec3::Vec3};

pub struct Buffer {
    arr: Vec<Color>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let arr = vec![Vec3(0., 0., 0.); width * height];
        Buffer { arr, width, height }
    }

    pub fn to_ppm(&self) -> String {
        let mut s = String::new();
        s.push_str("P3\n");
        s.push_str(&format!("{} {}\n", self.width, self.height));
        s.push_str("255\n");
        for color in &self.arr {
            let r = (color.0 * 255.99).clamp(0., 255.99) as u8;
            let g = (color.1 * 255.99).clamp(0., 255.99) as u8;
            let b = (color.2 * 255.99).clamp(0., 255.99) as u8;
            s.push_str(&format!("{} {} {}\n", r, g, b));
        }
        s
    }
}

impl Deref for Buffer {
    type Target = [Color];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.arr.as_ptr(), self.arr.len()) }
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.arr.as_mut_ptr(), self.arr.len()) }
    }
}
