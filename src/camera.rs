use crate::{vec3::Vec3, ray::Ray, utils::deg_to_rad};

pub struct Camera {
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,

    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Camera {

        let theta = deg_to_rad(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray { 
            origin: self.origin, 
            direction: self.lower_left_corner + s * self.horizontal + 
                t * self.vertical - self.origin
        }
    }
}