#[macro_use]
extern crate assert_float_eq;
mod geometry;
mod image;
mod ppmio;
mod ray;
mod trig;

use crate::geometry::{
    dot, lerp, random_in_hemisphere, random_in_unit_sphere, random_unit_vector, reflect, refract,
    Color, Point, Vec3, BLACK, WHITE,
};
use crate::image::ImageRGBA;
use crate::ppmio::ppmwrite;
use crate::ray::{hit_sphere2, Ray};
use rand::Rng;
use std::time::Instant;

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

/// Material scattering behaviour
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
        *attenuation = WHITE;
        let refraction_ratio =
            if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let unit_dir = r_in.dir.normed();
        let refracted = refract(&unit_dir, &rec.normal, refraction_ratio);
        *scattered = Ray { orig: rec.p, dir: refracted };
        println!("[mat=dielectric] IN: {unit_dir:?} OUT: {refracted:?}");

        true
    }
}

trait Hittable {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

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

    pub fn add(&mut self, object: &Sphere) {
        self.objects.push((*object).into());
    }

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
    lerp(&WHITE, &Color { x: 0.5, y: 0.7, z: 1.0 }, t)
}

// Using a world of objects as input
fn ray_color_2(
    r: &Ray, world: &HittableList, depth: usize, materials: &Vec<Box<dyn Material>>,
) -> Color {
    let mut rec = HitRecord::new();

    if depth == 0 {
        println!("[depth=0]!!! depth limit reached");
        return Color { x: 0.0, y: 0.0, z: 0.0 };
    }

    if world.hit(&r, 0.001, f32::INFINITY, &mut rec) {
        // --- using materials
        let mut scattered = Ray { orig: Vec3::ZERO, dir: Vec3::UNIT_Y };
        let mut attenuation = BLACK;

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
            // Color { x: 128.0 / 255.0, y: 0.0, z: 0.0 }
            BLACK
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
    println!("Hit the sky");
    let unit_direction = &r.dir.normed();
    let t = 0.5 * (unit_direction.y + 1.0);
    lerp(&WHITE, &Color { x: 0.5, y: 0.7, z: 1.0 }, t)
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
        let lower_left_corner = &origin
            - &(horizontal / 2.0)
            - (vertical / 2.0)
            - Vec3 { x: 0.0, y: 0.0, z: focal_length };

        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let dir =
            &self.lower_left_corner + &(u * self.horizontal) + (v * self.vertical) - self.origin;

        Ray { orig: self.origin.clone(), dir }
    }
}

fn render(width: usize, height: usize, max_depth: usize, samples_per_pixel: usize) -> ImageRGBA {
    let mut im = ImageRGBA::new(width, height);
    let mut materials: Vec<Box<dyn Material>> = Vec::new();

    materials.push(Box::new(Lambertian { albedo: Color { x: 0.8, y: 0.8, z: 0.0 } }));
    // materials.push(Box::new(Lambertian { albedo: Color { x: 0.7, y: 0.3, z: 0.3 } }));
    // materials.push(Box::new(Metal { albedo: Color { x: 0.8, y: 0.8, z: 0.8 }, fuzz: 0.3 }));
    materials.push(Box::new(Dieletric { refraction_index: 1.5 }));
    materials.push(Box::new(Dieletric { refraction_index: 1.5 }));
    materials.push(Box::new(Metal { albedo: Color { x: 0.8, y: 0.6, z: 0.2 }, fuzz: 1.0 }));

    let lambertian_index = 0;
    let dielectric_index = 1;
    let dielectric2_index = 2;
    let metal_index = 3;

    // world
    let mut world = HittableList::new();
    world.add(&Sphere {
        center: Point { x: 0.0, y: 0.0, z: -1.0 },
        radius: 0.5,
        material_id: dielectric_index,
    });
    world.add(&Sphere {
        center: Point { x: -1.0, y: 0.0, z: -1.0 },
        radius: 0.5,
        material_id: dielectric2_index,
    });
    //world.add(&Sphere { center: Point { x: 1.0, y: 0.0, z: -1.0 }, radius: 0.5, material_id: metal_index });
    world.add(&Sphere {
        center: Point { x: 0.0, y: -100.5, z: -1.0 },
        radius: 100.0,
        material_id: lambertian_index,
    });

    let cam = Camera::new();
    let mut rng = rand::thread_rng();
    println!("\n\n--- Starting render");

    for j in (0..im.height).rev() {
        //print!("\rScanlines remaining {j}");

        for i in 0..im.width {
            println!("=========== BEGIN rendering pixel at [{i}, {j}]");
            let mut pixel_color = BLACK;

            for s in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (im.width as f32 - 1.0);
                let v = (j as f32 + rng.gen::<f32>()) / (im.height as f32 - 1.0);

                let ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color_2(&ray, &world, max_depth, &materials);
            }
            pixel_color = pixel_color / samples_per_pixel as f32;

            // color correct for gamma=2.0
            let pixel_color_corrected =
                Vec3 { x: pixel_color.x.sqrt(), y: pixel_color.y.sqrt(), z: pixel_color.z.sqrt() };
            // gamma correction
            let ir = (clamp(pixel_color_corrected.x, 0.0, 0.999) * 256.0) as u8;
            let ig = (clamp(pixel_color_corrected.y, 0.0, 0.999) * 256.0) as u8;
            let ib = (clamp(pixel_color_corrected.z, 0.0, 0.999) * 256.0) as u8;

            println!("=========== DONE  rendering pixel at [{i}, {j}]");

            im.put(i, j, ir, ig, ib, 255);
        }
    }
    im
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 10;
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

    ppmwrite("out/image015.ppm", &im);
    ppmwrite("out/latest.ppm", &im);
}
