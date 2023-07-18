use std::ops;

#[derive(Debug)]
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

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }

    pub fn mul(&self, s: f32) -> Vec3 {
        Vec3 { x: self.x * s, y: self.y * s, z: self.z * s }
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<'a, 'b> ops::Add<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f32) -> Vec3 {
        Vec3 { x: self.x * s, y: self.y * s, z: self.z * s }
    }
}


impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 { x: self * v.x, y: self * v.y, z: self * v.z }
    }
}


impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, s: f32) -> Vec3 {
        Vec3 { x: self.x / s, y: self.y / s, z: self.z / s }
        // self * 1.0 / s
    }
}

impl<'a> ops::Neg for &'a Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}


impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}


impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        let eps = f32::EPSILON;
        (f32::abs(self.x - other.x) < eps) && (f32::abs(self.y - other.y) < eps) && (f32::abs(self.z - other.z) < eps)
    }
}


#[derive(Debug)]
pub struct ImageRGBA {
    pub width: usize,
    pub height: usize,

    pub pixels: Vec<u8>,
}

impl ImageRGBA {
    pub fn new(width: usize, height: usize) -> ImageRGBA {
        let sz = width * height * 4;
        let mut pixels = vec![0u8; sz as usize];
        ImageRGBA::init(&mut pixels);
        ImageRGBA { width, height, pixels }
    }

    fn init(pixels: &mut Vec<u8>) {
        let l = pixels.len();
        let count = l / 4;
        for i in 0..count {
            pixels[i * 4] = 10;
            pixels[i * 4 + 1] = 10;
            pixels[i * 4 + 2] = 10;
            pixels[i * 4 + 3] = 255
        }
    }


    pub fn at(&self, i: usize, j: usize) -> (u8, u8, u8, u8) {
        let idx = (j * self.width as usize + i) * 4 as usize;

        (self.pixels[idx], self.pixels[idx + 1], self.pixels[idx + 2], self.pixels[idx + 3])
    }

    pub fn put(&mut self, i: usize, j: usize, r: u8, g: u8, b: u8, a: u8) {
        let idx = (j * self.width as usize + i) * 4;

        self.pixels[idx] = r;
        self.pixels[idx + 1] = g;
        self.pixels[idx + 2] = b;
        self.pixels[idx + 3] = a;
    }
}


#[cfg(test)]
pub(crate) mod test {
    mod vec3 {
        use crate::Vec3;

        #[test]
        fn test_add() {
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
        fn test_mul() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };

            let p_times_2 = p * 2.0;
            let expected = Vec3 { x: 2.0, y: 4.0, z: 6.0 };
            assert_eq!(expected, p_times_2);
        }

        #[test]
        fn test_unary_neg() {
            let p = Vec3 { x: 1.0, y: 2.0, z: 3.0 };


            let minus_p = -p;
            let expected = Vec3{x: -1.0, y: -2.0, z: -3.0 };
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
    }
}
