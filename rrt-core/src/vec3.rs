use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::Rng;

/// A data structure to represent a 3D vector with a x, y, and z component.
/// Vec3 supports common arithmetic such as scalar/vector multiplication,
/// addition, subtraction, division, vector negation, and some common linear
/// algebra operations such as the dot product, cross product, etc ...
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    pub const ONE: Vec3 = Vec3 {
        x: 1.,
        y: 1.,
        z: 1.,
    };
    pub const X: Vec3 = Vec3 {
        x: 1.,
        y: 0.,
        z: 0.,
    };
    pub const Y: Vec3 = Vec3 {
        x: 0.,
        y: 1.,
        z: 0.,
    };
    pub const Z: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 1.,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
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

    pub fn rotate_about_x_axis(self, mut theta: f64) -> Vec3 {
        theta = theta.to_radians();
        Vec3::new(
            self.x,
            self.y * theta.cos() - self.z * theta.sin(),
            self.y * theta.sin() + self.z * theta.cos(),
        )
    }

    pub fn rotate_about_y_axis(self, mut theta: f64) -> Vec3 {
        theta = theta.to_radians();
        Vec3::new(
            self.x * theta.cos() + self.z * theta.sin(),
            self.y,
            self.x * -theta.sin() + self.z * theta.cos(),
        )
    }

    pub fn rotate_about_z_axis(self, mut theta: f64) -> Vec3 {
        theta = theta.to_radians();
        Vec3::new(
            self.x * theta.cos() - self.y * theta.sin(),
            self.x * theta.sin() + self.y * theta.cos(),
            self.z,
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[macro_export]
macro_rules! implement_op {
    ($struct:ident, $op:ident, $op_trait:ident, $c1: ident, $c2: ident, $c3: ident) => {
        impl $op_trait<$struct> for $struct {
            type Output = $struct;
            fn $op(self, v: $struct) -> $struct {
                $struct::new(
                    self.$c1.$op(v.$c1),
                    self.$c2.$op(v.$c2),
                    self.$c3.$op(v.$c3),
                )
            }
        }
        impl $op_trait<f64> for $struct {
            type Output = $struct;
            fn $op(self, f: f64) -> $struct {
                $struct::new(self.$c1.$op(f), self.$c2.$op(f), self.$c3.$op(f))
            }
        }
        impl $op_trait<$struct> for f64 {
            type Output = $struct;
            fn $op(self, v: $struct) -> $struct {
                $struct::new(self.$op(v.$c1), self.$op(v.$c2), self.$op(v.$c3))
            }
        }
    };
}

implement_op!(Vec3, add, Add, x, y, z);
implement_op!(Vec3, sub, Sub, x, y, z);
implement_op!(Vec3, div, Div, x, y, z);
implement_op!(Vec3, mul, Mul, x, y, z);

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
