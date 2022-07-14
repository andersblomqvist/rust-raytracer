use std::{ops::{self, Neg}};

use crate::utils::{random_f32, random_range_f32};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3 { x, y, z } }
    pub fn zero() -> Vec3 { Vec3 { x: 0.0, y: 0.0, z: 0.0 } }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_f32(),
            y: random_f32(),
            z: random_f32(),
        }
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: random_range_f32(min, max),
            y: random_range_f32(min, max),
            z: random_range_f32(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p = Vec3::random_range(-1.0, 1.0);
        loop {
            if p.length_squared() >= 1.0 {
                p = Vec3::random_range(-1.0, 1.0);
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::normalized(&Vec3::random_in_unit_sphere())
    }

    /**
     *  Sqrt magnitude of the vector as in math: ||v|| = scalar value.
     */
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    /**
     *  ||v||^2. Does not use sqrt()
     */
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + 
        self.y * self.y + 
        self.z * self.z
    }

    /**
     *  Dot product: v1.dot(v2) is <v1,v2>
     */
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    /**
     *  Cross product: v1.cross(v2) is v1 x v2
     */
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /**
     *  Returns a nomalized version of itself.
     */
    pub fn normalized(&self) -> Vec3 {
        self / self.length()
    }

    /**
     *  Return true of the vector is close to zero in all diemensions.
     */
    pub fn near_zero(&self) -> bool {
        let s = 0.0001;
        f32::abs(self.x) < s && f32::abs(self.y) < s && f32::abs(self.z) < s
    }

    /**
     *  Returns the refection vector for v when it hits a surface with normal n.
     */
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    /**
     *  Returns the refraction vector when uv hits a surface with normal n and
     *  refractive indicies etai_over_etat.
     */
    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min(-uv.dot(n), 1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * n;
        r_out_perp + r_out_parallel
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::zero()
    }
}

/**
 *  Operator overloading. Each operation is element wise.
 */

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { 
            x: -self.x, 
            y: -self.y, 
            z: -self.z 
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}


impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        };
    }
}
