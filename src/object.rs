use crate::{material::Material, shape::Shape};

pub struct Object {
    material: Material,
    shape: Box<dyn Shape>,
}
