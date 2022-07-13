use crate::{vec3::Vec3, ray::Ray};

pub struct Camera {
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,

    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        viewport_width: f32, 
        viewport_height: f32, 
        focal_length: f32) -> Camera {
            
        let origin = Vec3::new(0.0, 0.0, 0.5);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(
            0.0,
            0.0,
            focal_length
        );

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray { 
            origin: self.origin, 
            direction: self.lower_left_corner + u * self.horizontal + 
                v * self.vertical - self.origin
        }
    }
}