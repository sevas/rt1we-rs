//! Toy raytracer, following the [Raytracer in One Weekend](https://github.com/RayTracing/raytracing.github.io/) Series.

extern crate rt1we_renderer;
use std::time::Instant;

use rt1we_renderer::geometry::Vec3;
use rt1we_renderer::image::flipv;
use rt1we_renderer::ppmio::ppmwrite;
use rt1we_renderer::render::render;

#[cfg(not(tarpaulin_include))]
fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 160;
    let height = (width as f32 / aspect_ratio) as usize;
    let max_depth = 50;

    let samples_per_pixel = 100;

    let trajectory_points = vec![
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(2.0, 2.0, 1.0),
        Vec3::new(2.0, 0.1, 0.3),
        Vec3::new(-2.0, 0.1, 0.5),
    ];

    //let trajectory = interpolate(&trajectory_points, 2);
    let trajectory = vec![trajectory_points[0]];
    let count = trajectory.len();
    for (i, p) in trajectory.iter().enumerate() {
        print!("\n\n--- Rendering frame #{}/{}", i, count);
        let start = Instant::now();
        let im = render(width, height, max_depth, samples_per_pixel, p);
        let elapsed = start.elapsed();

        println!("\n--- Summary");
        println!("Time elapsed   : {elapsed:?}");
        println!("Image size     : {width}x{height}");
        println!("Max ray depth  : {max_depth}");
        println!("#Samples/px    : {samples_per_pixel}");

        let im = flipv(&im);

        let fpath = format!("out/anim_image_{:0>5}.ppm", i);
        ppmwrite(&fpath, &im);
        ppmwrite("out/latest.ppm", &im);
    }
}
