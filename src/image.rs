#[derive(Debug)]
pub struct ImageRGBA {
    pub width: usize,
    pub height: usize,

    pub pixels: Vec<u8>,
}

impl ImageRGBA {
    pub fn new(width: usize, height: usize) -> ImageRGBA {
        let sz = width * height * 4;
        let mut pixels = vec![0u8; sz];
        ImageRGBA::init(&mut pixels);
        ImageRGBA {
            width,
            height,
            pixels,
        }
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
        let idx = (j * self.width as usize + i) * 4usize;

        (
            self.pixels[idx],
            self.pixels[idx + 1],
            self.pixels[idx + 2],
            self.pixels[idx + 3],
        )
    }

    pub fn at_u32(&self, i: usize, j: usize) -> u32 {
        let idx = (j * self.width as usize + i) * 4usize;

        let (r, g, b, a) = (
            self.pixels[idx] as u32,
            self.pixels[idx + 1] as u32,
            self.pixels[idx + 2] as u32,
            self.pixels[idx + 3] as u32,
        );

        (r << 24) | (g << 16) | (b << 8) | a
    }

    pub fn put(&mut self, i: usize, j: usize, r: u8, g: u8, b: u8, a: u8) {
        let idx = (j * self.width as usize + i) * 4;

        self.pixels[idx] = r;
        self.pixels[idx + 1] = g;
        self.pixels[idx + 2] = b;
        self.pixels[idx + 3] = a;
    }

    pub fn put_u32(&mut self, i: usize, j: usize, rgba: u32) {
        let idx = (j * self.width as usize + i) * 4;

        let r = (rgba >> 24) as u8;
        let g = (rgba >> 16) as u8;
        let b = (rgba >> 8) as u8;
        let a = (rgba & 0xFF) as u8;

        self.pixels[idx] = r;
        self.pixels[idx + 1] = g;
        self.pixels[idx + 2] = b;
        self.pixels[idx + 3] = a;
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::image::ImageRGBA;

    #[test]
    fn test_new_image_is_dark_gray() {
        let w = 10usize;
        let h = 10usize;
        let im = ImageRGBA::new(w, h);

        for j in 0..h {
            for i in 0..w {
                let (r, g, b, a) = im.at(i, j);
                assert_eq!(r, 10);
                assert_eq!(g, 10);
                assert_eq!(b, 10);
                assert_eq!(a, 255);

                let px = im.at_u32(i, j);
                assert_eq!(px, 0x0A0A0AFF)
            }
        }
    }

    fn test_can_put_pixel_as_u8() {
        let w = 10usize;
        let h = 10usize;
        let mut im = ImageRGBA::new(w, h);

        im.put(5, 5, 255, 0, 0, 255);

        let (r, g, b, a) = im.at(5, 5);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
        assert_eq!(a, 255);
    }

    fn test_can_get_pixel_as_u32() {}
}
