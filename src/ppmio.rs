//! Read and Write functions for raw PPM images.
//!
//! We only support the legacy format with 'P3' magic number.
//! Details for this format can be read on the [netpbm documentation](https://netpbm.sourceforge.net/doc/ppm.html)
use crate::image::ImageRGBA;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::str::FromStr;

/// Write an image as PPM file.
///
/// # Arguments
/// - `fpath` - The file path to write to.
/// - `im` - The image data to write.
///
/// # Notes
/// The alpha channel is dropped.
///
pub fn ppmwrite(fpath: &str, im: &ImageRGBA) {
    let f = File::create(fpath).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let w = im.width;
    let h = im.height;
    let header = format!("P3\n{w} {h}\n255\n");

    f.write_all(header.as_bytes()).expect("unable to write data");
    let count = w * h;
    for i in 0..count {
        let r = im.pixels[i * 4];
        let g = im.pixels[i * 4 + 1];
        let b = im.pixels[i * 4 + 2];

        f.write_fmt(format_args!("{r} {g} {b}\n")).expect("unable to write data");
    }
}

/// Read a PPM image.
///
/// # Arguments
/// - `fpath` - File path of the file to read.
///
/// # File structrure
/// ```
/// P3
/// $width $height
/// $maxval
/// r b g
/// r g b
/// ...
/// r g b
/// EOF
/// ```
pub fn ppmread(fpath: &str) -> ImageRGBA {
    let f = File::open(fpath).expect("Unable to open file");
    let mut f = BufReader::new(f);

    let mut magic_bytes = String::new();
    let _ = f.read_line(&mut magic_bytes);
    let mut dim = String::new();
    let _ = f.read_line(&mut dim);
    let mut maxval = String::new();
    let _ = f.read_line(&mut maxval);

    let w_h: Vec<&str> = dim.split_whitespace().collect();
    let w = usize::from_str(w_h[0]).unwrap();
    let h = usize::from_str(w_h[1]).unwrap();

    let mut im = ImageRGBA::new(w, h);

    let count = w * h;

    for i in 0..count {
        let mut px_str = String::new();
        let _ = f.read_line(&mut px_str);
        let rgb: Vec<&str> = px_str.split_whitespace().collect();
        let r = u8::from_str(rgb[0]).unwrap();
        let g = u8::from_str(rgb[1]).unwrap();
        let b = u8::from_str(rgb[2]).unwrap();

        im.pixels[i * 4] = r;
        im.pixels[i * 4 + 1] = g;
        im.pixels[i * 4 + 2] = b;
        im.pixels[i * 4 + 3] = 255;
    }

    im
}

#[cfg(test)]
pub(crate) mod test {
    use crate::image::ImageRGBA;
    use crate::ppmio::{ppmread, ppmwrite};

    #[test]
    fn test_read_write_roundtrip() {
        let mut im = ImageRGBA::new(5, 3);
        im.put_u32(2, 2, 0x0F0A0AFF);

        let fpath = "/tmp/rt1wk-rs_im.ppm";
        // let file = NamedTempFile::new()?;
        // let fpath = file.into_temp_path();
        ppmwrite(fpath, &im);

        let im_r = ppmread(fpath);
        let count = im.height * im.width * 4;
        for i in 0..count {
            assert_eq!(im.pixels[i], im_r.pixels[i]);
        }
    }
}
