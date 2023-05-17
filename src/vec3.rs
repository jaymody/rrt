use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
        )
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }
}

// implement common operations on vectors that we'd like:
//
// - negation
// - vector addition, scalar addition
// - vector subtraction, scalar subtraction
// - vector multiplication, scalar multiplication
// - vector division, scalar division
//
// except for negation (Neg), we use the implement_op macro to implement these
// operations since otherwise it can get quite repetitive
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

macro_rules! implement_op {
    ($op:ident, $op_trait:ident) => {
        impl $op_trait<Vec3> for Vec3 {
            type Output = Vec3;
            fn $op(self, v: Vec3) -> Vec3 {
                Vec3::new(self.x.$op(v.x), self.y.$op(v.y), self.z.$op(v.z))
            }
        }
        impl $op_trait<f64> for Vec3 {
            type Output = Vec3;
            fn $op(self, f: f64) -> Vec3 {
                Vec3::new(self.x.$op(f), self.y.$op(f), self.z.$op(f))
            }
        }
        impl $op_trait<Vec3> for f64 {
            type Output = Vec3;
            fn $op(self, v: Vec3) -> Vec3 {
                Vec3::new(self.$op(v.x), self.$op(v.y), self.$op(v.z))
            }
        }
    };
}

implement_op!(add, Add);
implement_op!(sub, Sub);
implement_op!(div, Div);
implement_op!(mul, Mul);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_is_close(a: f64, b: f64) {
        const TOLERANCE: f64 = 1e-6;
        assert!(a - b < TOLERANCE, "lhs={} rhs={} diff={}", a, b, a - b);
    }

    fn assert_is_close_vec(u: Vec3, v: Vec3) {
        assert_is_close(u.x, v.x);
        assert_is_close(u.y, v.y);
        assert_is_close(u.z, v.z);
    }

    fn setup() -> (Vec3, Vec3) {
        (Vec3::new(1., 2., 3.), Vec3::new(2., 5., -4.))
    }

    #[test]
    fn test_length_squared() {
        let (u, v) = setup();
        assert_is_close(u.length_squared(), 14.);
        assert_is_close(v.length_squared(), 45.);
    }

    #[test]
    fn test_length() {
        let (u, v) = setup();
        assert_is_close(u.length(), 3.7416573868);
        assert_is_close(v.length(), 6.7082039325);
    }

    #[test]
    fn test_unit_vector() {
        let (u, v) = setup();
        assert_is_close_vec(
            u.unit_vector(),
            Vec3::new(0.2672612419, 0.5345224838, 0.8017837257),
        );
        assert_is_close_vec(
            v.unit_vector(),
            Vec3::new(0.298142397, 0.7453559925, -0.596284794),
        );
        assert_is_close(u.unit_vector().length(), 1.0);
        assert_is_close(v.unit_vector().length(), 1.0);
    }

    #[test]
    fn test_dot() {
        let (u, v) = setup();
        assert_eq!(u.dot(v), 0.);
        assert_eq!(v.dot(u), 0.);
        assert_is_close(u.dot(u), u.length_squared());
        assert_is_close(v.dot(v), v.length_squared());
    }

    #[test]
    fn test_cross() {
        let (u, v) = setup();
        assert_is_close_vec(u.cross(v), Vec3::new(-23., 10., 1.));
        assert_is_close_vec(v.cross(u), Vec3::new(23., -10., -1.));
    }

    #[test]
    fn test_neg() {
        let (u, v) = setup();
        assert_is_close_vec(u.neg(), Vec3::new(-1., -2., -3.));
        assert_is_close_vec(v.neg(), Vec3::new(-2., -5., 4.));
    }

    #[test]
    fn test_add() {
        let (u, v) = setup();
        assert_is_close_vec(u + v, Vec3::new(3., 7., -1.));
        assert_is_close_vec(v + u, Vec3::new(3., 7., -1.));
        assert_is_close_vec(u + 1., Vec3::new(2., 3., 4.));
        assert_is_close_vec(1. + u, Vec3::new(2., 3., 4.));
    }

    #[test]
    fn test_sub() {
        let (u, v) = setup();
        assert_is_close_vec(u - v, Vec3::new(-1., -3., 7.));
        assert_is_close_vec(v - u, Vec3::new(1., 3., -7.));
        assert_is_close_vec(u - 1., Vec3::new(0., 1., 2.));
        assert_is_close_vec(1. - u, Vec3::new(0., -1., -2.));
    }

    #[test]
    fn test_mul() {
        let (u, v) = setup();
        assert_is_close_vec(u * v, Vec3::new(2., 10., -12.));
        assert_is_close_vec(v * u, Vec3::new(2., 10., -12.));
        assert_is_close_vec(u * 2., Vec3::new(2., 4., 6.));
        assert_is_close_vec(2. * u, Vec3::new(2., 4., 6.));
    }

    #[test]
    fn test_div() {
        let (u, v) = setup();
        assert_is_close_vec(u / v, Vec3::new(0.5, 0.4, -0.75));
        assert_is_close_vec(v / u, Vec3::new(2., 2.5, -1.3333333333));
        assert_is_close_vec(u / 2., Vec3::new(0.5, 1., 1.5));
        assert_is_close_vec(2. / u, Vec3::new(2., 1., 0.6666666667));
        assert_is_close_vec(0. / v, Vec3::new(0., 0., 0.));
    }
}
