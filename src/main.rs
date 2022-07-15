use std::{f32, vec};

use raytracer::{
    vec3::Vec3, 
    ray::{Ray, Intersectable, Intersection}, 
    sphere::Sphere, 
    camera::Camera, 
    utils::{random_f32, clamp}, 
    material::{Material, MaterialType}
};

// Antialiasing
const SAMPLES_PER_PIXEL: i32 = 50;

// Max recursive depth for Diffuse bouncing
const MAX_DEPTH: i32 = 50;

/**
 *  Generate a random scene with a lot of balls.
 */
fn random_scene() -> Vec<Sphere> {
    let mut world: Vec<Sphere> = vec![];

    let mat_ground = Material::new(Vec3::new(0.8, 0.8, 0.8), 0.1, 0.0, MaterialType::Metal);
    world.push(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground));

    let point = Vec3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Vec3::new(a as f32 + 0.9 * random_f32(), 0.2, b as f32 + 0.9 * random_f32());

            if (center - point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let diffuse = Material::new(albedo, 0.0, 0.0, MaterialType::Diffuse);
                    world.push(Sphere::new(center, 0.2, diffuse));
                }
                else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let roughness = random_f32();
                    let metal = Material::new(albedo, roughness, 0.0, MaterialType::Metal);
                    world.push(Sphere::new(center, 0.2, metal));
                }
                else {
                    // glass
                    let glass = Material::new(Vec3::zero(), 0.0, 1.5, MaterialType::Dielectric);
                    world.push(Sphere::new(center, 0.2, glass));
                }
            }
        }
    }

    let mat_glass = Material::new(Vec3::zero(), 0.0, 1.5, MaterialType::Dielectric);
    world.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat_glass));

    let mat_diffuse = Material::new(Vec3::new(0.4, 0.2, 0.1), 0.0, 0.0, MaterialType::Diffuse);
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat_diffuse));

    let mat_metal = Material::new(Vec3::new(0.7, 0.6, 0.5), 0.0, 0.0, MaterialType::Metal);
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat_metal));

    world
}

fn main() {

    // Image 
    let width: i32 = 1280;
    let aspect_ratio: f32 = 3.0 / 2.0;
    let height: i32 = ((width as f32) / aspect_ratio) as i32;

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;
    let camera = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus);

    // World
    let world = random_scene();

    // Create .ppm image with std out. A .ppm image is just a text file.
    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        eprintln!("Remaining lines: {}", j+1);
        for i in 0..width {
            let mut pixel_color = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random_f32()) / (width-1) as f32;
                let v = (j as f32 + random_f32()) / (height-1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            write_color(&pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done.");
}

/**
 *  Writes the Vec3 as [0, 255] color to standard out.
 */
pub fn write_color(pixel_color: &Vec3, samples_per_pixel: i32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by the number of samples.
    let scale = 1.0 / (samples_per_pixel as f32);
    r = f32::sqrt(scale * r);
    g = f32::sqrt(scale * g);
    b = f32::sqrt(scale * b);

    // write the translated [0, 255] value of each color
    let ir = (clamp(r, 0.0, 0.999) * 256.0) as i32;
    let ig = (clamp(g, 0.0, 0.999) * 256.0) as i32;
    let ib = (clamp(b, 0.0, 0.999) * 256.0) as i32;
    println!("{} {} {}", ir, ig, ib);
}

/**
 *  Determine the color for a ray (pixel) depending on intersections with the
 *  world of spheres.
 */
pub fn ray_color(r: Ray, world: &Vec<Sphere>, depth: i32) -> Vec3 {
    
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::zero();
    }
    
    if let Some(intersection) = closest_intersection(&r, &world) {
        // Scatter ray based on material
        let (scatter, attenuation, scattered) = 
            intersection.material.scatter(&r, &intersection);

        if scatter {
            return attenuation * ray_color(scattered, world, depth - 1);
        } else {
            return Vec3::zero();
        }
    }

    let unit_dir = r.direction.normalized();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

/**
 *  Returns the closest intersection for ray, if any.
 */
pub fn closest_intersection(r: &Ray, world: &Vec<Sphere>) -> Option<Intersection> {

    let mut hit = false;
    let mut closest_so_far = f32::MAX;
    let mut closest_intersection: Intersection = Intersection::default();

    for sphere in world {
        if let Some(intersection) = sphere.hit(r, 0.001, closest_so_far) {
            closest_intersection = intersection;
            closest_so_far = closest_intersection.t;
            hit = true;
        }
    }

    if hit {
        return Some(closest_intersection);
    }
    None
}
