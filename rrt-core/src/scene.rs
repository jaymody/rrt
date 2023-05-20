use crate::{color::Color, hitrecord::HitRecord, object::Object, ray::Ray};

/// A scene is just a list of objects and an environment that determines the
/// ambient background lighting (if any).
pub struct Scene {
    /// The objects in the scene.
    objects: Vec<Object>,

    /// The background.
    environment: fn(ray: Ray) -> Color,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            environment: sky_environment,
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

    pub fn get_environment_light(&self, ray: Ray) -> Color {
        (self.environment)(ray)
    }
}

fn sky_environment(ray: Ray) -> Color {
    let t = (ray.direction.unit_vector().y + 1.0) / 2.0;
    Color::WHITE * (1.0 - t) + Color::SKY_BLUE * t
}
