use crate::{color::Color, hitrecord::HitRecord, object::Object, ray::Ray};

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

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn hit_closest_object(&self, ray: Ray) -> Option<(Ray, Color)> {
        let mut record = HitRecord::new();
        let mut closest_obj = None;
        for object in &self.objects {
            if object.shape.intersect(ray, &mut record) {
                closest_obj = Some(object);
            }
        }
        closest_obj.map(|obj| obj.material.scatter(record))
    }
}
