use crate::{vec3::Vec3, ray::{Intersection, Ray}, utils::random_f32};

#[derive(Clone, Copy)]
pub enum MaterialType {
    Diffuse,
    Metal,
    Dielectric,
}

#[derive(Clone, Copy)]
pub struct Material {
    pub albedo: Vec3,
    pub roughness: f32,
    pub ir: f32,
    pub material_type: MaterialType,
}

impl Material {

    pub fn new(albedo: Vec3, roughness: f32, ir: f32, material_type: MaterialType) -> Material {
        Material {
            albedo,
            roughness,
            ir,
            material_type,
        }
    }
    
    fn lambertian_scatter(&self, _r_in: &Ray, intersection: &Intersection) -> (bool, Vec3, Ray) {
        let mut scatter_direction = intersection.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = intersection.normal;
        }

        let scattered = Ray::new(intersection.point, scatter_direction);
        let attenuation = self.albedo;
        (true, attenuation, scattered)
    }

    fn metal_scatter(&self, r_in: &Ray, intersection: &Intersection) -> (bool, Vec3, Ray) {
        let reflected = Vec3::reflect(r_in.direction.normalized(), intersection.normal);

        let scattered = Ray::new(
            intersection.point, 
            reflected + self.roughness * Vec3::random_in_unit_sphere()
        );

        let attenuation = self.albedo;
        
        if scattered.direction.dot(intersection.normal) > 0.0 {
            (true, attenuation, scattered)
        } else {
            (false, attenuation, scattered)
        }
    }

    fn dielectric_scatter(&self, r_in: &Ray, intersection: &Intersection,) -> (bool, Vec3, Ray) {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        
        let mut refraction_ratio = self.ir;
        if intersection.front_face {
            refraction_ratio = 1.0 / self.ir;
        }

        let unit_direction = r_in.direction.normalized();

        let cos_theta = f32::min(-unit_direction.dot(intersection.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let mut total_reflection = false;
        if refraction_ratio * sin_theta > 1.0 {
            total_reflection = true;
        }

        let direction;
        if total_reflection || reflectance(cos_theta, refraction_ratio) > random_f32() {
            direction = Vec3::reflect(unit_direction, intersection.normal);
        } else {
            direction = Vec3::refract(unit_direction, intersection.normal, refraction_ratio);
        }

        let scattered = Ray::new(intersection.point, direction);

        (true, attenuation, scattered)
    }

    pub fn scatter(&self, r_in: &Ray, intersection: &Intersection) -> (bool, Vec3, Ray) {
        match self.material_type {
            MaterialType::Diffuse => self.lambertian_scatter(r_in, intersection),
            MaterialType::Metal => self.metal_scatter(r_in, intersection),
            MaterialType::Dielectric => self.dielectric_scatter(r_in, intersection),
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material { 
            albedo: Vec3::default(), 
            roughness: 0.0,
            ir: 0.0,
            material_type: MaterialType::Diffuse 
        }
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    let r1 = 1.0 - cosine;
    r0 + (1.0 - r0) * r1.powi(5)
}
