mod ppmio;
mod ray;
mod types;

use crate::ppmio::ppmwrite;
use crate::ray::Ray;
use crate::types::{dot, lerp, Color, ImageRGBA, Point, Vec3, RED, WHITE};
use std::io::Write;

fn hit_sphere(center: &Point, radius: f32, r: &Ray) -> f32 {
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
fn hit_sphere2(center: &Point, radius: f32, r: &Ray) -> f32 {
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

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere2(
        &Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
        r,
    );

    if t > 0.0 {
        let n = (r.at(t)
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            })
        .normed();
        return 0.5
            * Color {
                x: n.x + 1.0,
                y: n.y + 1.0,
                z: n.z + 1.0,
            };
    }
    let unit_direction = &r.dir.normed();
    let t = 0.5 * (unit_direction.y + 1.0);
    lerp(
        &WHITE,
        &Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        },
        1.0 - t,
    )
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

    let origin = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: vp_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: vp_height,
        z: 0.0,
    };
    let lower_left_corner = &origin
        - &(horizontal / 2.0)
        - (vertical / 2.0)
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    for j in (0..im.height).rev() {
        print!("\rScanlines remaining {j}");

        for i in 0..im.width {
            let u = i as f32 / (im.width as f32 - 1.0);
            let v = j as f32 / (im.height as f32 - 1.0);
            let ray = Ray {
                orig: origin,
                dir: &lower_left_corner + &(horizontal * u) + (vertical * v) - origin,
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
    ppmwrite("out/image004.ppm", im);
}
