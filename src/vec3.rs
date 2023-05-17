use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3(
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
        )
    }

    pub fn length_squared(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
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
        Vec3(-self.0, -self.1, -self.2)
    }
}

macro_rules! implement_op {
    ($op:ident, $op_trait:ident) => {
        impl $op_trait<Vec3> for Vec3 {
            type Output = Vec3;
            fn $op(self, v: Vec3) -> Vec3 {
                Vec3(self.0.$op(v.0), self.1.$op(v.1), self.2.$op(v.2))
            }
        }
        impl $op_trait<f64> for Vec3 {
            type Output = Vec3;
            fn $op(self, f: f64) -> Vec3 {
                Vec3(self.0.$op(f), self.1.$op(f), self.2.$op(f))
            }
        }
        impl $op_trait<Vec3> for f64 {
            type Output = Vec3;
            fn $op(self, v: Vec3) -> Vec3 {
                Vec3(self.$op(v.0), self.$op(v.1), self.$op(v.2))
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
        assert_is_close(u.0, v.0);
        assert_is_close(u.1, v.1);
        assert_is_close(u.2, v.2);
    }

    fn setup() -> (Vec3, Vec3) {
        (Vec3(1., 2., 3.), Vec3(2., 5., -4.))
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
            Vec3(0.2672612419, 0.5345224838, 0.8017837257),
        );
        assert_is_close_vec(
            v.unit_vector(),
            Vec3(0.298142397, 0.7453559925, -0.596284794),
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
        assert_is_close_vec(u.cross(v), Vec3(-23., 10., 1.));
        assert_is_close_vec(v.cross(u), Vec3(23., -10., -1.));
    }

    #[test]
    fn test_neg() {
        let (u, v) = setup();
        assert_is_close_vec(u.neg(), Vec3(-1., -2., -3.));
        assert_is_close_vec(v.neg(), Vec3(-2., -5., 4.));
    }

    #[test]
    fn test_add() {
        let (u, v) = setup();
        assert_is_close_vec(u + v, Vec3(3., 7., -1.));
        assert_is_close_vec(v + u, Vec3(3., 7., -1.));
        assert_is_close_vec(u + 1., Vec3(2., 3., 4.));
        assert_is_close_vec(1. + u, Vec3(2., 3., 4.));
    }

    #[test]
    fn test_sub() {
        let (u, v) = setup();
        assert_is_close_vec(u - v, Vec3(-1., -3., 7.));
        assert_is_close_vec(v - u, Vec3(1., 3., -7.));
        assert_is_close_vec(u - 1., Vec3(0., 1., 2.));
        assert_is_close_vec(1. - u, Vec3(0., -1., -2.));
    }

    #[test]
    fn test_mul() {
        let (u, v) = setup();
        assert_is_close_vec(u * v, Vec3(2., 10., -12.));
        assert_is_close_vec(v * u, Vec3(2., 10., -12.));
        assert_is_close_vec(u * 2., Vec3(2., 4., 6.));
        assert_is_close_vec(2. * u, Vec3(2., 4., 6.));
    }

    #[test]
    fn test_div() {
        let (u, v) = setup();
        assert_is_close_vec(u / v, Vec3(0.5, 0.4, -0.75));
        assert_is_close_vec(v / u, Vec3(2., 2.5, -1.3333333333));
        assert_is_close_vec(u / 2., Vec3(0.5, 1., 1.5));
        assert_is_close_vec(2. / u, Vec3(2., 1., 0.6666666667));
        assert_is_close_vec(0. / v, Vec3(0., 0., 0.));
    }
}
