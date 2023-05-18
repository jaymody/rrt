use crate::{color::Color, hitrecord::HitRecord, ray::Ray};

/// A material determines two things given a hit record:
///
///   1) what is the outgoing array (e.g. if the material is a mirror
///        we perfectly reflect w.r.t. the normal)
///   2) the attenuation of light (e.g. how much of each red light, blue
///        light, and green light is absorbed)
///
pub trait Material {
    fn scatter(&self, hit: HitRecord) -> (Ray, Color);
}
