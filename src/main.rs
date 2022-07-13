use std::f32;

use raytracer::{
    vec3::Vec3, 
    ray::{Ray, Intersectable}, 
    sphere::Sphere, 
    camera::Camera, 
    utils::{random_f32, clamp}, 
    material::Material
};

// Antialiasing
const SAMPLES_PER_PIXEL: i32 = 32;

// Diffuse boucing recursive depth
const MAX_DEPTH: i32 = 16;

fn main() {

    // Image 
    let width: i32 = 1280;
    let aspect_ratio: f32 = 16.0 / 9.0;
    let height: i32 = ((width as f32) / aspect_ratio) as i32;

    // World
    let mat_ground = Material::metal(Vec3::new(0.8, 0.8, 0.8), 0.3);
    let mat_center = Material::diffuse(Vec3::new(0.3, 0.5, 0.9));
    let mat_left   = Material::metal(Vec3::new(0.9, 0.9, 0.9), 0.01);
    let mat_right  = Material::metal(Vec3::new(0.8, 0.3, 0.2), 0.6);

    // Order matters
    let world: Vec<Sphere> = vec![
        Sphere::new(Vec3::new( 0.0,    0.0, -1.0),   0.5, mat_center),
        Sphere::new(Vec3::new(-1.3,    0.0, -1.0),   0.5, mat_left),
        Sphere::new(Vec3::new( 1.0,    0.5, -2.0),   1.0, mat_right),
        Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, mat_ground),
    ];

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let camera = Camera::new(
        aspect_ratio, 
        viewport_width, 
        viewport_height, 
        focal_length
    );

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

    for sphere in world {
        if let Some(intersection) = sphere.hit(&r, 0.001, f32::MAX) {
            // Scatter ray based on material
            let (scatter, attenuation, scattered) = 
                intersection.material.scatter(&r, &intersection);

            if scatter {
                return attenuation * ray_color(scattered, world, depth - 1);
            } else {
                return Vec3::zero();
            }
        }
    }

    let unit_dir = r.direction.normalized();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

