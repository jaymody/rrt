use crate::{hitrecord::HitRecord, ray::Ray, vec3::Vec3};

const T_MIN: f64 = 0.001;

/// A shape defines the math that determines if a given a ray hits it.
pub trait Shape {
    fn intersect(&self, ray: Ray, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    radius: f64,
    center: Vec3,
}

impl Sphere {
    pub fn new(radius: f64, center: Vec3) -> Self {
        Sphere { radius, center }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: Ray, record: &mut HitRecord) -> bool {
        // The equation for a sphere centered at the origin is:
        //
        //    x^2 + y^2 + z^2 = r^2
        //
        // If we pack x, y, z into a vector X, we can see that the left hand
        // side of the equation is the same as dot product of X with itself:
        //
        //    X . X = r^2
        //
        // The equation for a sphere is centered at C is:
        //
        //    (x - C_x)^2 + (y - C_y)^2 + (z - C_z) = r^2
        //
        // Using our vector notation, we can see that this is the same as:
        //
        //    (X - C) . (X - C) = r^2
        //
        // To find the point(s) where a line p + t*d hits the sphere, we simply
        // replace X with the equation for the line:
        //
        //    (p + t*d - C) . (p + t*d - C) = r^2
        //
        // If we expand and collect terms, we get:
        //
        //    (d.d)t^2 + (2d.(p - C))t + ((p - C).(p - C) - r^2) = 0
        //
        // If we assign p := p - C, then we get the simpler:
        //
        //    (d.d)t^2 + (2d.p)t + (p.p - r^2) = 0
        //
        // We can use the quadratic equation to solve for t:
        //
        //    t = (-b +- sqrt(b^2 - 4ac)) / 2a
        //
        // where a = d.d, b = 2d.p, and c = p.p - r^2. We can simplify the
        // equation further by exploiting the fact that the b term has a 2 term
        // in it. Let's replace b = 2h:
        //
        //    t = (-2h +- sqrt((2h)^2 - 4ac)) / 2a
        //    t = (-2h +- sqrt(4h^2 - 4ac)) / 2a
        //    t = (-2h +- sqrt(4(h^2 - ac))) / 2a
        //    t = (-2h +- 2 * sqrt(h^2 - ac)) / 2a
        //    t = (h +- * sqrt(h^2 - ac)) / a
        //
        // The two's cancel out everywhere, and this is better because h = d.p
        // is simpler to compute (no multiplication by 2). In total we save
        // 2 multiplications by 2, and a multiplication by 4.
        //
        // As usual, there are either 2 solutions (discriminant > 0), 1 solution
        // (discriminant = 0), or no solutions (discriminant < 0). We return
        // None if there is no solution (no hit), else we use the value of
        // t that results in a point that is closest to the ray origin.
        let p = ray.origin - self.center;
        let d = ray.direction;

        // Quadratic equation terms
        let a = d.dot(d);
        let h = p.dot(d); // h = 2/b
        let c = p.dot(p) - self.radius * self.radius;

        // The discriminant in the quadratic equation. Well not exactly,
        // technically this is 1/4 of the discriminant since:
        //
        //    h*h - a*c = b/2 * b/2 - a*c = 1/4 * (b^2 - 4ac)
        //
        // However this won't affect us checking if there are no solutions
        // (discriminant * 1/4 is still negative is discriminant is negative).
        let discriminant = h * h - a * c;

        // No solution, ray does not hit the sphere.
        if discriminant.is_sign_negative() {
            return false;
        }

        // There are two possibles points of intersection, we want the one
        // that's closest to the ray origin. This corresponds to the smallest
        // positive root (we don't care about negative values of t, so we don't
        // consider them, even if they are closer to the origin). The term `a`
        // is always positive (since it's the dot product of a vector with
        // itself), so this means that (-h - sqrt_d) / a will always be smaller
        // than (-h + sqrt_d) / a, so instead of computing them both and taking
        // the min, we can compute the first and only compute the second if the
        // first is not positive.
        let sqrt_d = discriminant.sqrt();
        let mut t = (-h - sqrt_d) / a;
        if t < T_MIN || t > record.t {
            t = (-h + sqrt_d) / a;
            if t < T_MIN || t > record.t {
                return false;
            }
        }

        record.t = t;
        record.p = ray.at(t);
        record.normal = (record.p - self.center) / self.radius;
        true
    }
}
