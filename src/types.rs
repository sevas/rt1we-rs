#[derive(Debug)]
pub struct VectorF {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl VectorF {
    fn dot(&self, other: &VectorF) -> f32 {
        self.x * other.x + self.y * other.y + self.z * self.y
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
        let idx = (j * self.width as usize + i) * 4  as usize;

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