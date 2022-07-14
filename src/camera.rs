use crate::{vec3::Vec3, ray::Ray, utils::deg_to_rad};

pub struct Camera {
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,

    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,

    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3, 
        lookat: Vec3, 
        vup: Vec3, 
        vfov: f32, 
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32) -> Camera {

        let theta = deg_to_rad(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist * w;
        
        let lens_radius = aperture / 2.0;

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            origin,
            horizontal,
            vertical,
            lower_left_corner,            
            u,
            v,
            lens_radius
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray { 
            origin: self.origin + offset, 
            direction: self.lower_left_corner + s * self.horizontal + 
                t * self.vertical - self.origin - offset
        }
    }
}