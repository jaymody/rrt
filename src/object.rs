use crate::{material::Material, shape::Shape};

/// An object is just a combination of a material and a shape.
pub struct Object {
    /// The objects shape.
    shape: Box<dyn Shape>,

    /// The objects material.
    material: Box<dyn Material>,
}
