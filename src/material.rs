use crate::{color::Color, hitrecord::HitRecord, ray::Ray, vec3::Vec3};

/// A material determines two things given a hit record:
///
///   1) what is the outgoing array (e.g. if the material is a mirror
///        we perfectly reflect w.r.t. the normal)
///   2) the attenuation of light (e.g. how much of each red light, blue
///        light, and green light is absorbed)
///
pub trait Material {
    fn scatter(&self, record: HitRecord) -> (Ray, Color);
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

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
