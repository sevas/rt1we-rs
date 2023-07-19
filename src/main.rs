mod types;
mod ray;

use crate::types::{ImageRGBA, Vec3, Color, WHITE, Point, lerp};
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


fn ray_color(r: &Ray) -> Color {
    let unit_direction = &r.dir.normed();
    let t = 0.5 * (unit_direction.y + 1.0);
    lerp(&WHITE, &Color { x: 0.5, y: 0.7, z: 1.0 }, 1.0-t)
}


fn render() -> ImageRGBA {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f32 / aspect_ratio) as usize;
    let mut im = ImageRGBA::new(width, height);

    // camera
    let vp_height = 2.0;
    let vp_width = aspect_ratio * vp_height;
    let focal_length = 1.0;

    let origin = Point { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = Vec3{x: vp_width as f32, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: vp_height as f32, z: 0.0};
    let lower_left_corner = &origin - &(horizontal / 2.0) - (vertical / 2.0) - Vec3{x: 0.0, y: 0.0, z: focal_length};


    for j in (0..im.height).rev() {
        print!("\rScanlines remaining {j}");

        for i in 0..im.width {
            let u = i as f32 / (im.width as f32 - 1.0);
            let v = j as f32 / (im.height as f32 - 1.0);
            let ray = Ray {
                orig: origin.into(),
                dir: &lower_left_corner + &(&horizontal * u) + (&vertical * v) - origin.into(),
            };
            let pixel_color = ray_color(&ray);
            let ir = (pixel_color.x * 255.0) as u8;
            let ig = (pixel_color.y * 255.0) as u8;
            let ib = (pixel_color.z * 255.0) as u8;

            im.put(i, j, ir, ig, ib, 255);
        }
    }

    println!("\nDone.");
    im.into()
}

fn main() {
    let im = render();
    ppmwrite("image2.ppm", im);
}

