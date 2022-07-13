use crate::{vec3::Vec3, ray::{Intersection, Ray}};

/**
 *  Material
 *  - albedo: Vec3
 *  scatter diffuse
 *  scatter metal
 * 
 *  diffuse = Material::diffuse()
 *  metal = Material::metal()
 * 
 *  Sphere::new(center, radius, diffuse)
 *  Sphere::new(center, radius, metal)
 * 
 *  sphere.scatter() => scatter Trait
 * 
 *  fn scatter(&self) {
 *      match &self.material_type {
 *          MaterialType.METAL => scatter_diffuse()
 *          MaterialType.DIFFUSE => scatter_metal()
 *      }
 *  }
 */

pub enum MaterialType {
    Diffuse,
    Metal,
}

pub struct Material {
    pub albedo: Vec3,
    pub roughness: f32,
    pub material_type: MaterialType,
}

impl Material {

    pub fn metal(albedo: Vec3, roughness: f32) -> Material {
        Material { albedo, roughness, material_type: MaterialType::Metal }
    }

    pub fn diffuse(albedo: Vec3) -> Material {
        Material { albedo, roughness: 0.0, material_type: MaterialType::Diffuse }
    }

    fn lambertian_scatter(
            &self,
            _r_in: &Ray, 
            intersection: &Intersection,
        ) -> (bool, Vec3, Ray) {
        let mut scatter_direction = intersection.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = intersection.normal;
        }

        let scattered = Ray::new(intersection.point, scatter_direction);
        let attenuation = self.albedo;
        (true, attenuation, scattered)
    }

    fn metal_scatter(
            &self,
            r_in: &Ray, 
            intersection: &Intersection,
        ) -> (bool, Vec3, Ray) {
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

    pub fn scatter(&self, r_in: &Ray, intersection: &Intersection) -> (bool, Vec3, Ray) {
        match self.material_type {
            MaterialType::Diffuse => self.lambertian_scatter(r_in, intersection),
            MaterialType::Metal => self.metal_scatter(r_in, intersection),
        }
    }
}
