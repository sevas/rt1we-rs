//! 3D geometry functions and data structures.
use rand::Rng;
use std::ops;
use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Copy, Clone)]
/// Vec3 representation.
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const UNIT_X: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const UNIT_Y: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const UNIT_Z: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let px = self.x;
        let py = self.y;
        let pz = self.z;
        let qx = other.x;
        let qy = other.y;
        let qz = other.z;

        Vec3 { x: py * qz - pz * qy, y: pz * qx - px * qz, z: px * qy - py * qx }
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    /// Returns normed Vec3
    pub fn normed(&self) -> Vec3 {
        let len = self.len();
        Vec3 { x: self.x / len, y: self.y / len, z: self.z / len }
    }

    pub fn norm(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    /// Returns a random vector with values in the `[0;1]` range.
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 { x: rng.gen(), y: rng.gen(), z: rng.gen() }
    }

    /// Returns a random vector with values in a given range.
    pub fn random_range(lo: f32, hi: f32) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 { x: rng.gen_range(lo..hi), y: rng.gen_range(lo..hi), z: rng.gen_range(lo..hi) }
    }

    /// Returns true if the vector is close to 0 in all dimensions
    pub fn near_zero(&self) -> bool {
        self.x.abs() < f32::EPSILON && self.y.abs() < f32::EPSILON && self.z.abs() < f32::EPSILON
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let v = Vec3::random_range(-1.0, 1.0);

        if v.len_squared() < 1.0 {
            break v;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normed()
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2.0 * &(dot(&n, &v) * n))
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = dot(&-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + &(cos_theta * n));
    let r_out_parallel = -1.0 * (1.0 - r_out_perp.len_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

// older method
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, &normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::ZERO
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

/// Returns sum of 2 Vec3, using references
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::Vec3;
/// let p = Vec3 {x:1.0, y:2.0, z:3.0};
/// let q = Vec3{x:0.5, y:0.5, z:0.5};
/// let r = &p + &q;
/// ```
impl<'a, 'b> ops::Add<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;
    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

/// Operator += for Vec3
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::Vec3;
/// let mut p = Vec3{x: 1.0, y: 2.0, z: 3.0 };
/// let q = Vec3{x: 0.5, y: 0.5, z: 0.5};
/// p += q;
/// ```
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

/// Operator - for Vec3
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::Vec3;
/// let p = Vec3 {x:1.0, y:2.0, z:3.0};
/// let q = Vec3{x: 0.5, y: 0.5, z: 0.5};
/// let s = p - q;
/// ```
impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

/// Operator -= for Vec3
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::Vec3;
/// let mut p = Vec3{x: 1.0, y: 2.0, z: 3.0 };
/// let q = Vec3{x: 0.5, y: 0.5, z: 0.5};
/// p -= q;
/// ```
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/// Returns diff of 2 Vec3, using references
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::Vec3;
/// let p = Vec3 {x:1.0, y:2.0, z:3.0};
/// let q = Vec3{x: 0.5, y: 0.5, z: 0.5};
/// let r = &p - &q;
/// ```
impl<'a, 'b> ops::Sub<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

/// Multiply a Vec3 by a scalar
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::Vec3;
/// let p = Vec3 {x: 1.0, y: 2.0, z: 3.0};
/// let q = p * 3.5;
/// ```
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f32) -> Vec3 {
        Vec3 { x: self.x * s, y: self.y * s, z: self.z * s }
    }
}

/// Multiply a Vec3 ref by a scalar
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::Vec3;
/// let p = Vec3 {x: 1.0, y: 2.0, z: 3.0};
/// let q = p * 3.5;
/// ```
impl<'a> ops::Mul<f32> for &'a Vec3 {
    type Output = Vec3;
    fn mul(self, s: f32) -> Vec3 {
        Vec3 { x: self.x * s, y: self.y * s, z: self.z * s }
    }
}

/// Multiply a scalar by a Vec3
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::Vec3;
/// let p = Vec3 {x:1.0, y:2.0, z:3.0};
/// let q = 3.5 * p;
/// ```
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 { x: self * v.x, y: self * v.y, z: self * v.z }
    }
}

impl<'a> ops::Mul<&'a Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: &Vec3) -> Vec3 {
        Vec3 { x: self * v.x, y: self * v.y, z: self * v.z }
    }
}

/// Divide a Vec3 by a scalar
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::Vec3;
/// let p = Vec3 {x:1.0, y:2.0, z:3.0};
/// let q = p / 3.5;
/// ```
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, s: f32) -> Vec3 {
        Vec3 { x: self.x / s, y: self.y / s, z: self.z / s }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<'a> ops::Div<f32> for &'a Vec3 {
    type Output = Vec3;
    fn div(self, s: f32) -> Vec3 {
        Vec3 { x: self.x / s, y: self.y / s, z: self.z / s }
    }
}

/// Change the sign of a Vec3 ref
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::Vec3;
/// let p = Vec3 {x:1.0, y:2.0, z:3.0};
/// let q = -&p;
/// ```
impl<'a> ops::Neg for &'a Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

/// Change the sign of a Vec3
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::Vec3;
/// let p = Vec3 {x:1.0, y:2.0, z:3.0};
/// let q = -p;
/// ```
impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        let eps = f32::EPSILON;
        (f32::abs(self.x - other.x) < eps)
            && (f32::abs(self.y - other.y) < eps)
            && (f32::abs(self.z - other.z) < eps)
    }
}

pub fn lerp(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
    (1.0 - t) * a + (t * b)
}

/// Dot product of 2 Vec3
///
/// # Examples
/// ```
/// use rt1we_renderer::geometry::{Vec3, dot};
/// let p = Vec3 {x:1.0, y:2.0, z:3.0};
/// let q = Vec3 {x:3.0, y:2.0, z:1.0};
/// let pdotq = dot(&p, &q);
///
/// ```
pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub type Point = Vec3;
pub type Color = Vec3;

pub fn make_color_from_u8(r: u8, g: u8, b: u8) -> Color {
    Color { x: r as f32 / 255.0, y: g as f32 / 255.0, z: b as f32 / 255.0 }
}

impl Color {
    pub const RED: Color = Color { x: 200.0 / 255.0, y: 0.0 / 255.0, z: 0.0 / 255.0 };
    pub const GREEN: Color = Color { x: 0.0 / 255.0, y: 200.0 / 255.0, z: 0.0 / 255.0 };
    pub const BLUE: Color = Color { x: 0.0 / 255.0, y: 0.0 / 255.0, z: 200.0 / 255.0 };
    pub const WHITE: Color = Color { x: 1.0, y: 1.0, z: 1.0 };
    pub const BLACK: Color = Color { x: 0.0 / 255.0, y: 0.0 / 255.0, z: 0.0 / 255.0 };
    pub const CYAN: Color = Color { x: 34.0 / 255.0, y: 166.0 / 255.0, z: 153.0 / 255.0 };
    pub const YELLOW: Color = Color { x: 242.0 / 255.0, y: 190.0 / 255.0, z: 34.0 / 255.0 };
}

#[cfg(test)]
pub(crate) mod test {
    mod vec3 {
        use crate::geometry::{
            lerp, make_color_from_u8, random_in_hemisphere, reflect, refract, Vec3,
        };

        #[test]
        fn test_default_vec3_is_all_zeros() {
            let vec: Vec3 = Default::default();

            assert_eq!(vec, Vec3::ZERO);
        }

        #[test]
        fn test_random_vector_has_values_in_0_1_range() {
            let rand_vec3 = Vec3::random();
            assert!(rand_vec3.x >= 0.0 && rand_vec3.x <= 1.0);
            assert!(rand_vec3.y >= 0.0 && rand_vec3.y <= 1.0);
            assert!(rand_vec3.z >= 0.0 && rand_vec3.z <= 1.0);
        }

        #[test]
        fn test_random_vec_in_hemisphere_always_with_unit_sphere() {
            let random_vec3 = random_in_hemisphere(&Vec3::UNIT_Y);
            assert!(random_vec3.len() <= 1.0);

            let random_vec3 = random_in_hemisphere(&-Vec3::UNIT_Y);
            assert!(random_vec3.len() <= 1.0);
        }

        #[test]
        fn test_add_operator_sums_vec_components() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let q = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

            let p_plus_q = p + q;
            let expected = Vec3 { x: 5.0, y: 7.0, z: 9.0 };
            assert_eq!(expected, p_plus_q);
        }

        #[test]
        fn test_add_ref() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let q = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

            let p_plus_q = &p + &q;
            let expected = Vec3 { x: 5.0, y: 7.0, z: 9.0 };
            assert_eq!(expected, p_plus_q);
        }

        #[test]
        fn test_addassign() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let q = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
            let mut summed = p;

            summed += q;
            assert_eq!(summed, p + q);
        }

        #[test]
        fn test_sub_operator_sums_vec_components() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let q = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

            let p_minus_q = p - q;
            let expected = Vec3 { x: -3.0, y: -3.0, z: -3.0 };
            assert_eq!(expected, p_minus_q);
        }

        #[test]
        fn test_subassign() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let q = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
            let mut subbed = p;

            subbed -= q;
            assert_eq!(subbed, p - q);
        }

        #[test]
        fn test_mul() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };

            let p_times_2 = p * 2.0;
            let expected = Vec3 { x: 2.0, y: 4.0, z: 6.0 };
            assert_eq!(expected, p_times_2);
        }

        #[test]
        fn test_mulassign() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };

            let mut mult = p;
            mult *= 2.0;

            assert_eq!(mult, p * 2.0);
        }

        #[test]
        fn test_div_with_scalar() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let p_div_2 = p / 2.0;
            let expected = Vec3 { x: 0.5, y: 1.0, z: 1.5 };
            assert_eq!(expected, p_div_2);
        }

        #[test]
        fn test_divref_with_scalar() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let p_div_2 = &p / 2.0;
            let expected = Vec3 { x: 0.5, y: 1.0, z: 1.5 };
            assert_eq!(expected, p_div_2);
        }

        #[test]
        fn test_divassign_with_scalar() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let mut divided = p;

            divided /= 2.0;

            assert_eq!(divided, p / 2.0);
        }

        #[test]
        fn test_mul_vec_with_vec() {
            let v = Vec3 { x: 0.0, y: 1.0, z: 2.0 };
            let w = Vec3 { x: 3.0, y: 4.0, z: 5.0 };

            assert_eq!(v * w, Vec3 { x: 0.0, y: 4.0, z: 10.0 })
        }

        #[test]
        fn test_unary_neg() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };

            let minus_p = -p;
            let expected = Vec3 { x: -1.0, y: -2.0, z: -3.0 };
            assert_eq!(expected, minus_p);
        }

        #[test]
        fn test_unary_neg_ref() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };

            let minus_p = -&p;
            let expected = p * -1.0;
            assert_eq!(expected, minus_p);
        }

        #[test]
        fn test_div() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };

            let p_div_2 = p / 2.0;
            let expected = Vec3 { x: 0.5, y: 1.0, z: 1.5 };
            assert_eq!(expected, p_div_2);
        }

        #[test]
        fn test_dot_product() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let q = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

            let p_dot_q = p.dot(&q);
            let expected = 4.0 + 10.0 + 18.0f32;
            assert_eq!(expected, p_dot_q);
        }

        #[test]
        fn test_cross_product_x_cross_y_eq_z() {
            let p = Vec3::UNIT_X;
            let q = Vec3::UNIT_Y;

            let p_cross_q = p.cross(&q);
            let expected = Vec3::UNIT_Z;
            assert_eq!(expected, p_cross_q);
        }

        #[test]
        fn test_cross_product() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let q = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

            let p_cross_q = p.cross(&q);
            let expected = Vec3 { x: -3.0, y: 6.0, z: -3.0 };
            assert_eq!(expected, p_cross_q);
        }

        #[test]
        fn test_normed_returns_a_normalized_vec() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let pn = p.normed();
            let sz = p.len();
            let expected = Vec3 { x: 1.0 / sz, y: 2.0 / sz, z: 3.0 / sz };
            assert_eq!(expected, pn);
            assert!(f32::abs(pn.len() - 1.0) < f32::EPSILON);
        }

        #[test]
        fn test_norm_vec_in_place() {
            let mut p = Vec3 { x: 12.0, y: 45.0, z: 1.0 };
            assert!(p.len() > 1.0);

            p.norm();
            assert_f32_near!(p.len(), 1.0);
        }

        #[test]
        fn test_expression() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
            let q = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

            let sum = &p + &q;
            let diff = &p - &q;

            assert_eq!(Vec3 { x: 5.0, y: 7.0, z: 9.0 }, sum);
            assert_eq!(Vec3 { x: -3.0, y: -3.0, z: -3.0 }, diff);

            let expr = &p + &(&q - &p);
            assert_eq!(Vec3 { x: 4.0, y: 5.0, z: 6.0 }, expr);
        }

        #[test]
        fn test_lerp() {
            let p = Vec3::ZERO;
            let q = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
            let t = 0.5;

            let half = lerp(&p, &q, t);
            assert_eq!(Vec3 { x: 0.5, y: 0.5, z: 0.5 }, half);
        }

        #[test]
        fn test_near_zero_returns_true_when_all_components_are_close_to_0() {
            let v = Vec3::ZERO;
            assert!(v.near_zero());
        }

        #[test]
        fn test_near_zero_returns_false_when_any_components_is_not_close_to_0() {
            let v = Vec3 { x: 0.1, y: 0.0, z: 0.0 };
            assert!(!v.near_zero());
        }

        #[test]
        fn test_reflect() {
            let v = Vec3 { x: 1.0, y: -1.0, z: 0.0 }.normed();
            let n = Vec3::UNIT_Y;

            let reflected = reflect(&v, &n);
            let expected = Vec3 { x: 1.0, y: 1.0, z: 0.0 }.normed();
            assert_eq!(expected, reflected);
        }

        #[test]
        fn test_refract() {
            let uv = Vec3 { x: 1.0, y: 1.0, z: 0.0 };
            let n = Vec3 { x: -1.0, y: 0.0, z: 0.0 };
            let etai_over_etat = 1.0;
            let expected = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
            let actual = refract(&uv, &n, etai_over_etat);
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_refract_with_air_refraction_coefficient_returns_same_vector() {
            let v = Vec3 { x: 1.0, y: -1.0, z: 0.0 }.normed();
            let n = Vec3::UNIT_Y;

            let refracted = refract(&v, &n, 1.0);
            let expected = Vec3 { x: 1.0, y: -1.0, z: 0.0 }.normed();
            assert_eq!(expected, refracted);
        }

        #[test]
        fn test_refract_with_glass_refraction_coefficient_returns_refracted_vector() {
            let v = Vec3 { x: 1.0, y: -1.0, z: 0.0 }.normed();
            let n = Vec3::UNIT_Y;

            let refracted = refract(&v, &n, 1.3);
            let expected = Vec3 { x: 0.91923875, y: -0.39370057, z: 0.0 };
            assert_eq!(expected, refracted);
        }

        #[test]
        fn test_make_color_from_u8_normalizes_values_in_0_1_range() {
            let [r, g, b] = [127u8, 127u8, 127u8];
            let color = make_color_from_u8(r, g, b);
            assert_f32_near!(color.x, 127.0 / 255.0);
            assert_f32_near!(color.y, 127.0 / 255.0);
            assert_f32_near!(color.z, 127.0 / 255.0);
        }
    }
}
