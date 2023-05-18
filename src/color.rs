use std::ops::{Deref, DerefMut};

use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    vec: Vec3,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { vec: Vec3(r, g, b) }
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        (
            (self.0 * 255.99).clamp(0., 255.99) as u8,
            (self.1 * 255.99).clamp(0., 255.99) as u8,
            (self.2 * 255.99).clamp(0., 255.99) as u8,
        )
    }
}

// implementing Deref and DerefMut allow us to use color as if it were a Vec3
// (that is we can do color1.dot(color2) and color1 + color) while also being
// able to implement it's own methods and be kept as a separate type
impl Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}
