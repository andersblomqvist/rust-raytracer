use crate::{vec3::Vec3, material::Material};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub struct Intersection<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a Material,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub trait Intersectable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Intersection>;
}

impl Intersection<'_> {
    pub fn new(point: Vec3, normal: Vec3, t: f32, material: &Material) -> Intersection {
        Intersection{ point, normal, t, front_face: false, material }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        let dir = outward_normal.dot(r.direction);
        if dir < 0.0 {
            self.normal = outward_normal;
            self.front_face = true;
        } else {
            self.normal = -outward_normal;
            self.front_face = false;
        }
    }
}