use crate::image::{flipv, ImageRGBA};
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn ppmwrite(fname: &str, im: ImageRGBA) {
    let f = File::create(fname).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let w = im.width;
    let h = im.height;
    let header = format!("P3\n{w} {h}\n255\n");

    let im = flipv(&im);
    f.write_all(header.as_bytes())
        .expect("unable to write data");
    let count = w * h;
    for i in 0..count {
        let r = im.pixels[i * 4];
        let g = im.pixels[i * 4 + 1];
        let b = im.pixels[i * 4 + 2];

        f.write_fmt(format_args!("{r} {g} {b}\n"))
            .expect("unable to write data");
    }
}
