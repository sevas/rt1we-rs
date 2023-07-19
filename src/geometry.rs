use std::ops;

#[derive(Debug, Copy, Clone)]
/// Vec3 representation
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const UNIT_X: Vec3 = Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const UNIT_Y: Vec3 = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const UNIT_Z: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub fn new() -> Vec3 {
        Vec3::ZERO
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

        Vec3 {
            x: py * qz - pz * qy,
            y: pz * qx - px * qz,
            z: px * qy - py * qx,
        }
    }

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn mul(&self, s: f32) -> Vec3 {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
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
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Returns sum of 2 Vec3, using references
///
/// # Examples
/// ```
/// let p = Vec3{...};
/// let q = Vec3{...};
/// let r = &p + &q;
/// ```
impl<'a, 'b> ops::Add<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;
    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Returns diff of 2 Vec3, using references
///
/// # Examples
/// ```
/// let p = Vec3{...};
/// let q = Vec3{...};
/// let r = &p - &q;
/// ```
impl<'a, 'b> ops::Sub<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Multiply a Vec3 by a scalar
///
/// # Examples
/// ```
/// let p = Vec3 { ... }
/// let q = p * 3.5;
/// ```
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f32) -> Vec3 {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

/// Multiply a Vec3 ref by a scalar
///
/// # Examples
/// ```
/// let p = Vec3 { ... }
/// let q = p * 3.5;
/// ```
impl<'a> ops::Mul<f32> for &'a Vec3 {
    type Output = Vec3;
    fn mul(self, s: f32) -> Vec3 {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

/// Multiply a scalar by a Vec3
///
/// # Examples
/// ```
/// let p = Vec3 { ... }
/// let q = 3.5 * p;
/// ```
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl<'a> ops::Mul<&'a Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

/// Divide a Vec3 by a scalar
///
/// # Examples
/// ```
/// let p = Vec3 { ... }
/// let q = p * 3.5;
/// ```
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, s: f32) -> Vec3 {
        Vec3 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

impl<'a> ops::Div<f32> for &'a Vec3 {
    type Output = Vec3;
    fn div(self, s: f32) -> Vec3 {
        Vec3 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
        // self * 1.0 / s
    }
}

/// Change the sign of a Vec3 ref
///
/// # Examples
/// ```
/// let p = Vec3 { ... }
/// let q = -&p;
/// ```
impl<'a> ops::Neg for &'a Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Change the sign of a Vec3
///
/// # Examples
/// ```
/// let p = Vec3 { ... }
/// let q = -p;
/// ```
impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
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
/// let p = Vec3 {...};
/// let q = Vec3 {...};
/// let pdotq = dot(&p, &q);
///
/// ```
pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub type Point = Vec3;
pub type Color = Vec3;

pub fn make_color_from_u8(r: u8, g: u8, b: u8) -> Color {
    Color {
        x: r as f32 / 255.0,
        y: g as f32 / 255.0,
        z: b as f32 / 255.0,
    }
}

pub const RED: Color = Color {
    x: 200.0 / 255.0,
    y: 0.0 / 255.0,
    z: 0.0 / 255.0,
};
pub const GREEN: Color = Color {
    x: 0.0 / 255.0,
    y: 200.0 / 255.0,
    z: 0.0 / 255.0,
};
pub const BLUE: Color = Color {
    x: 0.0 / 255.0,
    y: 0.0 / 255.0,
    z: 200.0 / 255.0,
};
pub const WHITE: Color = Color {
    x: 255.0 / 255.0,
    y: 255.0 / 255.0,
    z: 255.0 / 255.0,
};
pub const BLACK: Color = Color {
    x: 0.0 / 255.0,
    y: 0.0 / 255.0,
    z: 0.0 / 255.0,
};
pub const CYAN: Color = Color {
    x: 34.0 / 255.0,
    y: 166.0 / 255.0,
    z: 153.0 / 255.0,
};
pub const YELLOW: Color = Color {
    x: 242.0 / 255.0,
    y: 190.0 / 255.0,
    z: 34.0 / 255.0,
};

#[cfg(test)]
pub(crate) mod test {
    mod vec3 {
        use crate::geometry::lerp;
        use crate::Vec3;

        #[test]
        fn test_add() {
            let p = Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let q = Vec3 {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            };

            let p_plus_q = p + q;
            let expected = Vec3 {
                x: 5.0,
                y: 7.0,
                z: 9.0,
            };
            assert_eq!(expected, p_plus_q);
        }

        #[test]
        fn test_add_ref() {
            let p = Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let q = Vec3 {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            };

            let p_plus_q = &p + &q;
            let expected = Vec3 {
                x: 5.0,
                y: 7.0,
                z: 9.0,
            };
            assert_eq!(expected, p_plus_q);
        }

        #[test]
        fn test_mul() {
            let p = Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let p_times_2 = p * 2.0;
            let expected = Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0,
            };
            assert_eq!(expected, p_times_2);
        }

        #[test]
        fn test_unary_neg() {
            let p = Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let minus_p = -p;
            let expected = Vec3 {
                x: -1.0,
                y: -2.0,
                z: -3.0,
            };
            assert_eq!(expected, minus_p);
        }

        #[test]
        fn test_unary_neg_ref() {
            let p = Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let minus_p = -&p;
            let expected = p * -1.0;
            assert_eq!(expected, minus_p);
        }

        #[test]
        fn test_div() {
            let p = Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let p_div_2 = p / 2.0;
            let expected = Vec3 {
                x: 0.5,
                y: 1.0,
                z: 1.5,
            };
            assert_eq!(expected, p_div_2);
        }

        #[test]
        fn test_dot_product() {
            let p = Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let q = Vec3 {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            };

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
            let p = Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let q = Vec3 {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            };

            let p_cross_q = p.cross(&q);
            let expected = Vec3 {
                x: -3.0,
                y: 6.0,
                z: -3.0,
            };
            assert_eq!(expected, p_cross_q);
        }

        #[test]
        fn test_normed() {
            let p = Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let pn = p.normed();
            let sz = p.len();
            let expected = Vec3 {
                x: 1.0 / sz,
                y: 2.0 / sz,
                z: 3.0 / sz,
            };
            assert_eq!(expected, pn);
            assert!(f32::abs(pn.len() - 1.0) < f32::EPSILON);
        }

        #[test]
        fn test_expression() {
            let p = Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let q = Vec3 {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            };

            let sum = &p + &q;
            let diff = &p - &q;

            assert_eq!(
                Vec3 {
                    x: 5.0,
                    y: 7.0,
                    z: 9.0
                },
                sum
            );
            assert_eq!(
                Vec3 {
                    x: -3.0,
                    y: -3.0,
                    z: -3.0
                },
                diff
            );

            let expr = &p + &(&q - &p);
            assert_eq!(
                Vec3 {
                    x: 4.0,
                    y: 5.0,
                    z: 6.0
                },
                expr
            );
        }

        #[test]
        fn test_lerp() {
            let p = Vec3::ZERO;
            let q = Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            let t = 0.5;

            let half = lerp(&p, &q, t);
            assert_eq!(
                Vec3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5
                },
                half
            );
        }
    }
}
