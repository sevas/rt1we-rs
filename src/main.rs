//! Toy raytracer, following the [Raytracer in One Weekend](https://github.com/RayTracing/raytracing.github.io/) Series.
//!
//! This module implements the main render loop and scene management. Might refactor later.
#[macro_use]
extern crate assert_float_eq;
mod geometry;
mod image;
mod ppmio;
mod ray;
mod trig;

use crate::geometry::{
    dot, lerp, random_in_unit_sphere, random_unit_vector, reflect, refract, Color, Point, Vec3,
};
use crate::image::{flipv, ImageRGBA};
use crate::ppmio::ppmwrite;
use crate::ray::{hit_sphere2, Ray};
use rand::Rng;
use std::time::Instant;

/// Define a single ray-to-object hit.
#[derive(Copy, Clone)]
pub struct HitRecord {
    p: Point,
    normal: Vec3,
    material_id: usize,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Point { x: 0.0, y: 0.0, z: 0.0 },
            material_id: 0,
            normal: Vec3::ZERO,
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.dir, &outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal.clone();
        }
    }
}

/// Material scattering behaviour.
trait Material {
    /// Scatter or absorb a ray.
    ///
    /// # Arguments
    /// - `r_in` - Ray coming in the hit point
    /// - `rec` - The hit record
    /// - `attenuation` - How much the
    /// - `scattered` - The output scattered ray
    fn scatter(
        &self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray,
    ) -> bool;
}

/// Lambertian (diffuse) material.
#[derive(Copy, Clone, Debug)]
struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray { orig: rec.p, dir: scatter_direction };
        *attenuation = self.albedo;
        println!(
            "[mat=lambertian] IN: {0:?}  OUT: {1:?}  ATT: {2:?}",
            &rec.normal, scatter_direction, attenuation
        );
        true
    }
}

/// Shiny metal (reflective) material.
#[derive(Copy, Clone, Debug)]
struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.dir.normed(), &rec.normal);
        *scattered = Ray { orig: rec.p, dir: reflected + self.fuzz * random_in_unit_sphere() };
        *attenuation = self.albedo;
        let res = dot(&scattered.dir, &rec.normal) > 0.0;
        res
    }
}

/// Refractive material.
#[derive(Copy, Clone, Debug)]
struct Dieletric {
    refraction_index: f32,
}

// attenuation = color(1.0, 1.0, 1.0);
//             double refraction_ratio = rec.front_face ? (1.0/ir) : ir;
//
//             vec3 unit_direction = unit_vector(r_in.direction());
//             vec3 refracted = refract(unit_direction, rec.normal, refraction_ratio);
//
//             scattered = ray(rec.p, refracted);
//             return true;

impl Material for Dieletric {
    fn scatter(
        &self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::WHITE;
        let refraction_ratio =
            if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let unit_dir = r_in.dir.normed();
        let refracted = refract(&unit_dir, &rec.normal, refraction_ratio);
        *scattered = Ray { orig: rec.p, dir: refracted };
        println!("[mat=dielectric] IN: {unit_dir:?} OUT: {refracted:?}");

        true
    }
}

/// Trait for objects we can hit with a ray.
trait Hittable {
    ///
    fn hit(self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

/// Sphere object description.
#[derive(Copy, Clone)]
pub struct Sphere {
    center: Point,
    radius: f32,
    material_id: usize,
}

impl Hittable for Sphere {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = &r.orig - &self.center;
        let a = r.dir.len_squared();
        let half_b = dot(&oc, &r.dir);
        let c = oc.len_squared() - self.radius * self.radius;
        let disc = (half_b * half_b) - (a * c);

        if disc < 0.0 {
            return false;
        }

        let sqrt_disc = disc.sqrt();

        let root = (-half_b - sqrt_disc) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrt_disc) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.material_id = self.material_id;
        rec.set_face_normal(r, &outward_normal);
        true
    }
}

/// Collection of object that can be hit by a ray.
pub struct HittableList {
    objects: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    /// Add an object to the list.
    ///
    /// # Arguments
    /// - `object` - The object to add.
    pub fn add(&mut self, object: &Sphere) {
        self.objects.push((*object).into());
    }

    /// Process a single ray cast.
    ///
    /// # Arguments
    /// - `r` - The ray.
    /// - `t_min` - Minimum distance for which the ray cast is considered a valid hit.
    /// - `t_max` - Maximum distance for which the ray cast is considered a valid hit.
    /// - `rec` - Keep track of the hit properties.
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for each in &self.objects {
            if each.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

/// Using single sphere as input
fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere2(&Point { x: 0.0, y: 0.0, z: -1.0 }, 0.5, r);

    if t > 0.0 {
        // println!("ray hit sphere at {t}");

        let n = (r.at(t) - Vec3 { x: 0.0, y: 0.0, z: -1.0 }).normed();
        return 0.5 * Color { x: n.x + 1.0, y: n.y + 1.0, z: n.z + 1.0 };
    }

    let unit_direction = &r.dir.normed();
    let t = 0.5 * (unit_direction.y + 1.0);
    lerp(&Color::WHITE, &Color { x: 0.5, y: 0.7, z: 1.0 }, t)
}

/// Cast a single ray in the scene and return the computed pixel color.
///
/// This is a recursive function. As long as a hit produces a scattered ray, the function
/// will be called again with that new ray, until we reach `depth=0` or we have no more
/// scattering ray.
///
/// If no object is hit, we just return a *sky* color, which is a gradient modulated by the
/// ray direction.
///
/// # Arguments
/// - `r` - The ray.
/// - `world` - The list of object we can hit.
/// - `depth` - Remaining amount of ray bounces.
/// - `materials` - The collection of materials used in the scene.
fn ray_color_2(
    r: &Ray, world: &HittableList, depth: usize, materials: &Vec<Box<dyn Material>>,
) -> Color {
    let mut rec = HitRecord::new();

    if depth == 0 {
        println!("[depth=0]!!! depth limit reached");
        return Color { x: 0.0, y: 0.0, z: 0.0 };
    }

    if world.hit(r, 0.001, f32::INFINITY, &mut rec) {
        // --- using materials
        let mut scattered = Ray { orig: Vec3::ZERO, dir: Vec3::UNIT_Y };
        let mut attenuation = Color::BLACK;

        let was_scattered =
            materials[rec.material_id].scatter(r, &mut rec, &mut attenuation, &mut scattered);

        println!("[depth={depth}]was scattered?  {was_scattered}");
        println!("[depth={depth}]attenuation?  {attenuation:?}");
        return if was_scattered {
            let px_color = ray_color_2(&scattered, world, depth - 1, &materials);
            println!("[depth={depth}]px_color {px_color:?}");
            attenuation * px_color
            // attenuation * ray_color_2(&scattered, world, depth - 1, &materials)
        } else {
            Color::BLACK
        };
        // --- simple lambertian
        // let target = rec.p + rec.normal + random_unit_vector();
        // //let target = rec.p + random_in_hemisphere(&rec.normal);
        // let new_ray = Ray {
        //     orig: rec.p,
        //     dir: target - rec.p,
        // };
        // return 0.5 * ray_color_2(&new_ray, &world, depth - 1);

        // --- return normal as color
        //return 0.5 * (&rec.normal + &WHITE);
    }

    // background sky
    println!("[depth={depth}] Hit the sky");
    let unit_direction = &r.dir.normed();
    let t = 0.5 * (unit_direction.y + 1.0);
    lerp(&Color::WHITE, &Color { x: 0.5, y: 0.7, z: 1.0 }, t)
}

fn clamp(v: f32, lo: f32, hi: f32) -> f32 {
    if v < lo {
        return lo;
    }
    if v > hi {
        return hi;
    }
    v
}

/// Represent a camera.
struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;

        let vp_height = 2.0;
        let vp_width = aspect_ratio * vp_height;
        let focal_length = 1.0;

        let origin = Point { x: 0.0, y: 0.0, z: 0.0 };
        let horizontal = Vec3 { x: vp_width, y: 0.0, z: 0.0 };
        let vertical = Vec3 { x: 0.0, y: vp_height, z: 0.0 };
        let lower_left_corner = origin
            - (horizontal / 2.0)
            - (vertical / 2.0)
            - Vec3 { x: 0.0, y: 0.0, z: focal_length };

        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    /// Generate a ray from the camera origin to the given pixel coordinates.
    /// The coordinates are normalized between 0 and 1.
    /// (0, 0) is the lower left corner, (1, 1) is the upper right corner.
    /// # Arguments
    /// - `u` - Horizontal coordinate
    /// - `v` - Vertical coordinate
    /// # Returns
    /// A ray from the camera origin to the given pixel coordinates.
    /// The coordinates are normalized between 0 and 1.
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let dir =
            self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin;

        Ray { orig: self.origin.clone(), dir }
    }
}

/// Set up a scene a render an image.
///
/// # Arguments
/// - `width` - Output image width
/// - `height` - Output image height
/// - `max_depth` - Maximum number of ray bounces after a hit.
/// - `samples_per_pixel` - How many random rays to generate and average to compute final pixel color.
fn render(width: usize, height: usize, max_depth: usize, samples_per_pixel: usize) -> ImageRGBA {
    let mut im = ImageRGBA::new(width, height);
    let materials: Vec<Box<dyn Material>> = vec![
        Box::new(Lambertian { albedo: Color { x: 0.8, y: 0.8, z: 0.0 } }),
        Box::new(Lambertian { albedo: Color { x: 0.7, y: 0.3, z: 0.3 } }),
        Box::new(Metal { albedo: Color { x: 0.8, y: 0.8, z: 0.8 }, fuzz: 0.3 }),
        Box::new(Metal { albedo: Color { x: 0.8, y: 0.6, z: 0.2 }, fuzz: 1.0 }),
        Box::new(Dieletric { refraction_index: 1.5 }),
        Box::new(Dieletric { refraction_index: 1.5 }),
    ];

    let lambertian_green_index = 0;
    let lambertian_pink_index = 1;
    let metal_shiny_index = 2;
    let metal_fuzzy_index = 3;
    let dielectric_index = 4;
    let dielectric2_index = 5;

    // world
    let mut world = HittableList::new();
    // center sphere
    world.add(&Sphere {
        center: Point { x: 0.0, y: 0.0, z: -1.0 },
        radius: 0.5,
        material_id: dielectric_index,
    });
    // left sphere
    world.add(&Sphere {
        center: Point { x: -1.0, y: 0.0, z: -1.0 },
        radius: 0.5,
        material_id: metal_shiny_index,
    });
    // right sphere
    world.add(&Sphere {
        center: Point { x: 1.0, y: 0.0, z: -1.0 },
        radius: 0.5,
        material_id: lambertian_pink_index,
    });
    // ground sphere
    world.add(&Sphere {
        center: Point { x: 0.0, y: -100.5, z: -1.0 },
        radius: 100.0,
        material_id: lambertian_green_index,
    });

    let cam = Camera::new();
    let mut rng = rand::thread_rng();
    println!("\n\n--- Starting render");

    for j in (0..im.height).rev() {
        //print!("\rScanlines remaining {j}");

        for i in 0..im.width {
            println!("=========== BEGIN rendering pixel at [{i}, {j}]");
            let mut pixel_color = Color::BLACK;

            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (im.width as f32 - 1.0);
                let v = (j as f32 + rng.gen::<f32>()) / (im.height as f32 - 1.0);

                let ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color_2(&ray, &world, max_depth, &materials);
            }
            pixel_color = pixel_color / samples_per_pixel as f32;

            // color correct for gamma=2.0
            let pixel_color_corrected =
                Vec3 { x: pixel_color.x.sqrt(), y: pixel_color.y.sqrt(), z: pixel_color.z.sqrt() };

            let ir = (clamp(pixel_color_corrected.x, 0.0, 0.999) * 256.0) as u8;
            let ig = (clamp(pixel_color_corrected.y, 0.0, 0.999) * 256.0) as u8;
            let ib = (clamp(pixel_color_corrected.z, 0.0, 0.999) * 256.0) as u8;

            println!("=========== DONE  rendering pixel at [{i}, {j}]");

            im.put(i, j, ir, ig, ib, 255);
        }
    }
    im
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 16;
    let height = (width as f32 / aspect_ratio) as usize;
    let max_depth = 5;

    let samples_per_pixel = 1;

    let start = Instant::now();
    let im = render(width, height, max_depth, samples_per_pixel);
    let elapsed = start.elapsed();

    println!("=============== Summary");
    println!("Time elapsed   : {elapsed:?}");
    println!("Image size     : {width}x{height}");
    println!("Max ray depth  : {max_depth}");
    println!("#Samples/px    : {samples_per_pixel}");

    let im = flipv(&im);
    ppmwrite("out/image015.ppm", &im);
    ppmwrite("out/latest.ppm", &im);
}

#[cfg(test)]
pub(crate) mod test {
    use crate::image::ImageRGBA;
    use crate::render;

    #[test]
    fn test_nominal_render() {
        let im = render(16, 9, 5, 1);
        let default_img = ImageRGBA::new(16, 9);

        assert_eq!(im.width, 16);
        assert_eq!(im.height, 9);

        let mut diff_count = 0usize;
        for (p1, p2) in im.pixels.iter().zip(default_img.pixels.iter()) {
            if p1 != p2 {
                diff_count += 1;
            }
        }
        assert!((diff_count as f32) / im.pixels.len() as f32 > 0.5);
    }
}
