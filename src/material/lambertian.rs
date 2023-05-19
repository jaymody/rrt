use crate::{color::Color, hitrecord::HitRecord, ray::Ray, utils::random_in_unit_sphere};

use super::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, record: HitRecord) -> (Ray, Color) {
        let outgoing_ray = Ray::new(record.p, record.normal + random_in_unit_sphere());
        let attenuation = self.albedo;
        (outgoing_ray, attenuation)
    }
}
