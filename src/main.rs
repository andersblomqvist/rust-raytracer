use std::{f32, vec, sync::mpsc, thread, time};

use num_format::{ToFormattedString, Locale};
use raytracer::{
    vec3::{Vec3, IVec3}, 
    ray::{Ray, Intersectable, Intersection}, 
    sphere::Sphere, 
    camera::Camera, 
    utils::{random_f32, clamp}, 
    material::{Material, MaterialType}
};

// Antialiasing
const SAMPLES_PER_PIXEL: i32 = 500;

// Max recursive depth for Diffuse bouncing
const MAX_DEPTH: i32 = 32;

pub struct RenderContext {
    width: i32,
    height: i32,
    camera: Camera,
    world: Vec<Sphere>,
}

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
                if choose_mat < 0.3 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let diffuse = Material::new(albedo, 0.0, 0.0, MaterialType::Diffuse);
                    world.push(Sphere::new(center, 0.2, diffuse));
                }
                else if choose_mat < 0.6 {
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

    let mat_diffuse = Material::new(Vec3::new(0.8, 0.2, 0.1), 0.0, 0.0, MaterialType::Diffuse);
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat_diffuse));

    let mat_metal = Material::new(Vec3::new(0.7, 0.6, 0.5), 0.0, 0.0, MaterialType::Metal);
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat_metal));

    world
}

fn main() {

    // Image 
    let width: i32 = 256;
    let aspect_ratio: f32 = 16.0 / 9.0;
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

    let ctx = RenderContext {
        width,
        height,
        camera,
        world
    };

    let now = time::Instant::now();

    render(ctx);
    // render_multithreading(ctx);

    let time = now.elapsed().as_secs();
    let formatted_number = time.to_formatted_string(&Locale::fr);
    eprintln!("Calculation time: {} s", formatted_number);
}

/**
 *  Renderes the contex on a single thread
 */
pub fn render(ctx: RenderContext) {
    for y in (0..ctx.height).rev() {
        eprintln!("Remaining: {}", y+1);
        for x in 0..ctx.width {
            let mut pixel_color = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f32 + random_f32()) / (ctx.width-1) as f32;
                let v = (y as f32 + random_f32()) / (ctx.height-1) as f32;
                let r = ctx.camera.get_ray(u, v);
                pixel_color += ray_color(r, &ctx.world, MAX_DEPTH);
            }
            write_color_stdout(&pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done.");
}

/**
 *  Renderes the context on multiple threads. Should be faster.
 */
pub fn render_multithreading(ctx: RenderContext) {

    // Number of threads to spawn
    let n_threads = 16;
    
    if ctx.height % n_threads != 0 {
        eprintln!("Can't divide {} into {} equal parts!", ctx.height, n_threads);
        return;
    }

    let rows_per_thread = ctx.height / n_threads;

    let (sender, receiver) = mpsc::channel();

    // First thread will calculate rows [0 .. Y1]
    // Second thread will calculate rows [Y1 .. Y2]
    // This means when we collect the threads we should write to .ppm file in
    // the reversed order. Starting reading last thread.

    for thread in 0..n_threads {
        let sender_n = sender.clone();

        let mut segment: Vec<IVec3> = vec![];
        let from = rows_per_thread * thread;
        let to = rows_per_thread * (thread + 1);

        let local_world = ctx.world.to_vec().clone();

        thread::spawn(move || {
            
            // render segment
            for y in from..to {
                for x in (0..ctx.width).rev() {
                    let mut pixel_color = Vec3::zero();
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = (x as f32 + random_f32()) / (ctx.width-1) as f32;
                        let v = (y as f32 + random_f32()) / (ctx.height-1) as f32;
                        let r = ctx.camera.get_ray(u, v);
                        pixel_color += ray_color(r, &local_world, MAX_DEPTH);
                    }
                    let color = get_color(&pixel_color, SAMPLES_PER_PIXEL);
                    segment.push(color);
                }
            }
            // end render segment

            match sender_n.send((thread, segment)) {
                Ok(_) => {},
                Err(_) => eprintln!("Receiver has stopped listening, dropped worker {}", thread),
            }
        });
    }

    eprintln!("Spawned {} threads.", n_threads);
    eprintln!("Rendering ...");

    // Collect the threads
    let mut result: Vec<Vec<IVec3>> = vec![];
    let mut order: Vec<i32> = vec![];
    for _ in 0..n_threads {
        match receiver.recv() {
            Ok((thread, data)) => {
                eprintln!("Thread {} done.", thread);
                result.push(data);
                order.push(thread);
            }
            Err(_) => eprintln!("Failed to collect thread"),
        }
    }

    // Now we want to write thread N, N-1, N-2 ... 0
    // The order array tells us where a thread is in the result array.
    // Example: [15, 11, 1, 9, ...] says that thread 15 is at index 0.
    let mut n = n_threads - 1;
    eprintln!("Writing data to file ...");
    while n >= 0 {
        eprintln!("Progress: {}", n);
        'search: for i in 0..order.len() {
            let thread = order[i];
            if n == thread {
                write_segment_stdout(&result[i]);
                break 'search;
            }
        }
        n -= 1;
    }
}

fn write_segment_stdout(pixels: &Vec<IVec3>) {
    for i in (0..pixels.len()).rev() {
        let color = pixels[i];
        println!("{} {} {}", color.x, color.y, color.z);
    }
}

/**
 *  Writes the Vec3 as [0, 255] color to standard out.
 */
pub fn write_color_stdout(pixel_color: &Vec3, samples_per_pixel: i32) {
    let color = get_color(pixel_color, samples_per_pixel);
    println!("{} {} {}", color.x, color.y, color.z);
}

/**
 *  Get the pixel color in [0, 255] integer format.
 */
fn get_color(pixel_color: &Vec3, samples_per_pixel: i32) -> IVec3 {
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

    IVec3 { x: ir, y: ig, z: ib }
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
