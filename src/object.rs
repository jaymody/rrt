use crate::{material::Material, shape::Shape};

/// An object is just a combination of a material and a shape.
pub struct Object {
    /// The objects shape.
    pub shape: Box<dyn Shape>,

    /// The objects material.
    pub material: Box<dyn Material>,
}

impl Object {
    pub fn new(shape: Box<dyn Shape>, material: Box<dyn Material>) -> Self {
        Object { shape, material }
    }
}
