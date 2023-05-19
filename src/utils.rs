use crate::vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
