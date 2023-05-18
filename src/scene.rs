use crate::object::Object;

/// A scene is just a list of objects.
pub struct Scene {
    /// The objects in the scene.
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }
}
