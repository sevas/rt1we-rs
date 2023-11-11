//! Ray casting functions and data strutures.
use crate::geometry::{dot, Point, Vec3};

#[derive(Debug)]
/// Ray representation.
pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
}

impl Ray {
    /// Get point at a distance along the ray
    ///
    /// # Parameters
    /// - `t` - Distance
    pub fn at(&self, t: f32) -> Point {
        &self.orig + &(&self.dir * t)
    }
}

pub fn hit_sphere(center: &Point, radius: f32, r: &Ray) -> f32 {
    // Sphere hits are the points where:
    //      x^2 + y^2 + z^2 - r^2 = 0
    // For a sphere of radius r, center C, it's all the points P satisfying:
    //      (P-C)·(P-C) - r^2 = 0
    // Equation is rewritten in terms of vectors, parametrized by variable t:
    //      (A + t*b - C)·(A + t*b - C) - r^2 = 0
    // Sphere hit is now finding the roots of the univariate quadratic equation (parametrized by t):
    //      t^2 * b·b + t * 2*b·(A-C) + (A-C)·(A-C) - r^2 = 0
    // with:
    //      b = r.dir
    //      A = r.orig
    //      C = center
    //      r = radius
    let oc = &r.orig - &center;
    let a = dot(&r.dir, &r.dir);
    let b = 2.0 * dot(&oc, &r.dir);
    let c = dot(&oc, &oc) - radius * radius;
    let disc = b * b - 4.0 * a * c;

    if disc < 0.0 {
        return -1.0;
    } else {
        (-b - disc.sqrt()) / (2.0 * a)
    }
}

// Same as hit_sphere(), but simplified formulas, mostly removes sqrt's
pub fn hit_sphere2(center: &Point, radius: f32, r: &Ray) -> f32 {
    let oc = &r.orig - &center;
    let a = r.dir.len_squared();
    let half_b = dot(&oc, &r.dir);
    let c = oc.len_squared() - radius * radius;
    let disc = (half_b * half_b) - (a * c);

    if disc < 0.0 {
        return -1.0;
    } else {
        (-half_b - disc.sqrt()) / a
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::geometry::Point;
    use crate::geometry::Vec3;
    use crate::ray::{hit_sphere, hit_sphere2, Ray};

    #[test]
    fn test_projection() {
        let r =
            Ray { orig: Point { x: 0.0, y: 0.0, z: 0.0 }, dir: Vec3 { x: 1.0, y: 1.0, z: 1.0 } };

        let projected = r.at(5.0);
        let expected = Vec3 { x: 5.0, y: 5.0, z: 5.0 };
        assert_eq!(expected, projected);
    }

    #[test]
    fn test_hit_sphere_returns_correct_distance_when_hitting_a_sphere_just_in_front() {
        let center = Vec3 { x: 0.0, y: 0.0, z: -1.0 };
        let radius = 0.5;
        let ray = Ray { orig: Vec3::ZERO, dir: -Vec3::UNIT_Z };

        let hit_distance = hit_sphere(&center, radius, &ray);
        assert_eq!(hit_distance, 0.5);
        let hit_distance = hit_sphere2(&center, radius, &ray);
        assert_eq!(hit_distance, 0.5);
    }

    #[test]
    fn test_hit_sphere_returns_minus_1_when_ray_does_not_hit_the_sphere() {
        let center = Vec3 { x: 0.0, y: 10.0, z: -1.0 };
        let radius = 0.5;
        let ray = Ray { orig: Vec3::ZERO, dir: -Vec3::UNIT_Z };

        let hit_distance = hit_sphere(&center, radius, &ray);
        assert_eq!(hit_distance, -1.0);
        let hit_distance = hit_sphere2(&center, radius, &ray);
        assert_eq!(hit_distance, -1.0);
    }
}
