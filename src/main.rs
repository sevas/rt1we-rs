mod types;
mod ray;

use crate::types::{ImageRGBA, Vec3};
use crate::ray::{Ray};
use std::fs::File;
use std::io::{BufWriter, Write};


fn ppmwrite(fname: &str, im: ImageRGBA) {
    let f = File::create(fname).expect("Unable to create file");
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


fn render(im: &mut ImageRGBA) {
    for j in (0..im.height).rev() {
        print!("\rScanlines remaining {j}");

        for i in 0..im.width {
            let r = i as f64 / (im.width - 1) as f64;
            let g = j as f64 / (im.height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;
            im.put(i, j, ir, ig, ib, 255);
        }
    }

    println!("\nDone.")
}

fn main() {
    let mut im = ImageRGBA::new(256, 256);
    render(&mut im);
    ppmwrite("image2.ppm", im);
}

