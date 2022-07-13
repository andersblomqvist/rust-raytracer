use crate::{ray::{Intersectable, Intersection, Ray}, vec3::Vec3, material::Material};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Intersectable for Sphere {

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) ->  Option<Intersection> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminat = half_b * half_b - a * c;
        
        if discriminat < 0.0 {
            return None;
        } 
        
        let sqrtd = f32::sqrt(discriminat);
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = r.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let t = root;

        let mut intersection = Intersection::new(point, outward_normal, t, &self.material);
        intersection.set_face_normal(r, outward_normal);

        Some(intersection)
    }
}