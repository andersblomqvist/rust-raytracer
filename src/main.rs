use std::f32;

use raytracer::{
    vec3::Vec3, 
    ray::{Ray, Intersectable, Intersection}, 
    sphere::Sphere, 
    camera::Camera, 
    utils::{random_f32, clamp}, 
    material::{Material, MaterialType}
};

// Antialiasing
const SAMPLES_PER_PIXEL: i32 = 32;

// Max recursive depth for Diffuse bouncing
const MAX_DEPTH: i32 = 16;

fn main() {

    // Image 
    let width: i32 = 400;
    let aspect_ratio: f32 = 16.0 / 9.0;
    let height: i32 = ((width as f32) / aspect_ratio) as i32;

    // Camera
    let lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.6;
    let camera = Camera::new(lookfrom, lookat, vup, 30.0, aspect_ratio, aperture, dist_to_focus);

    // World
    let mat_ground = Material::new(Vec3::new(0.8, 0.8, 0.8), 0.1, 0.0, MaterialType::Metal);
    let mat_center = Material::new(Vec3::new(0.3, 0.5, 0.9), 0.0, 0.0, MaterialType::Diffuse);
    let mat_left   = Material::new(Vec3::new(0.0, 0.0, 0.0), 0.0, 1.5, MaterialType::Dielectric);
    let mat_right  = Material::new(Vec3::new(0.8, 0.3, 0.2), 0.6, 1.3, MaterialType::Dielectric);

    let world: Vec<Sphere> = vec![
        Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, mat_ground),
        Sphere::new(Vec3::new( 0.0,    0.0, -1.0),   0.5, mat_center),
        Sphere::new(Vec3::new(-1.0,    0.0, -1.0),  -0.45, mat_left),
        Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.5, mat_left),
        Sphere::new(Vec3::new( 1.0,    0.0, -1.0),   0.5, mat_right),
    ];

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
