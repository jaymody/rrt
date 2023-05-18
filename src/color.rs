use std::ops::{Add, Deref, DerefMut, Div, Mul, Neg, Sub};

use crate::{implement_op, vec3::Vec3};

/// Color and Vec3 are kept as different structs to help differentiate it's
/// usage from a 3D vector/point in space (here, the xyz elements are rgb),
/// despite them implementing very similar functionality (such as scalar/vector
/// addition, etc ...). This allows us to strictly type check it in our code (
/// we can't accidentally input a Vec3 as a color) and allow us to define
/// constants/methods specific to colors.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
    };
    pub const BLACK: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
    };
    pub const RED: Color = Color {
        r: 1.,
        g: 0.,
        b: 0.,
    };
    pub const GREEN: Color = Color {
        r: 0.,
        g: 1.,
        b: 0.,
    };
    pub const BLUE: Color = Color {
        r: 0.,
        g: 0.,
        b: 1.,
    };
    pub const SKY_BLUE: Color = Color {
        r: 0.5,
        g: 0.7,
        b: 1.0,
    };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn from_vec3(vec: Vec3) -> Self {
        Color {
            r: vec.x,
            g: vec.y,
            b: vec.z,
        }
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.99).clamp(0., 255.99) as u8,
            (self.g * 255.99).clamp(0., 255.99) as u8,
            (self.b * 255.99).clamp(0., 255.99) as u8,
        )
    }
}

impl Neg for Color {
    type Output = Color;
    fn neg(self) -> Color {
        Color::new(-self.r, -self.g, -self.b)
    }
}

implement_op!(Color, add, Add, r, g, b);
implement_op!(Color, sub, Sub, r, g, b);
implement_op!(Color, div, Div, r, g, b);
implement_op!(Color, mul, Mul, r, g, b);
