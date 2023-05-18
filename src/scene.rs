use crate::object::Object;

pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }
}
