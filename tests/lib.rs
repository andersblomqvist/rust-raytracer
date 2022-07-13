#[cfg(test)]
mod tests {
    use raytracer::vec3::Vec3;

    #[test]
    fn vector_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        let v3 = v1 + v2;

        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 3.0);
        assert_eq!(v3.z, 3.0);
    }

    #[test]
    fn vector_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        v1 += v2;

        assert_eq!(v1.x, 3.0);
        assert_eq!(v1.y, 3.0);
        assert_eq!(v1.z, 3.0);
    }

    #[test]
    fn vector_sub_v1_v2() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        let v3 = v1 - v2;

        assert_eq!(v3.x, -1.0);
        assert_eq!(v3.y, 1.0);
        assert_eq!(v3.z, 3.0);
    }

    #[test]
    fn vector_sub_v2_v1() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        let v3 = v2 - v1;

        assert_eq!(v3.x, 1.0);
        assert_eq!(v3.y, -1.0);
        assert_eq!(v3.z, -3.0);
    }

    #[test]
    fn vector_sub_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        v1 -= v2;

        assert_eq!(v1.x, -1.0);
        assert_eq!(v1.y, 1.0);
        assert_eq!(v1.z, 3.0);
    }

    #[test]
    fn vector_mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        let v3 = v1 * v2;

        assert_eq!(v3.x, 2.0);
        assert_eq!(v3.y, 2.0);
        assert_eq!(v3.z, 0.0);
    }

    #[test]
    fn vector_mul_scalar() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        let v3 = v1 * v2 * 3.0;

        assert_eq!(v3.x, 6.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, 0.0);
    }

    #[test]
    fn vector_mul_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        v1 *= v2;

        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 0.0);
    }

    #[test]
    fn vector_div() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        let v3 = v1 / v2;

        assert_eq!(v3.x, 1.0/2.0);
        assert_eq!(v3.y, 2.0/1.0);
        assert_eq!(v3.z, 3.0/0.0);
        
    }

    #[test]
    fn vector_div_scalar() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v3 = v1 / 2.0;

        assert_eq!(v3.x, 1.0/2.0);
        assert_eq!(v3.y, 2.0/2.0);
        assert_eq!(v3.z, 3.0/2.0);
        
    }

    #[test]
    fn vector_div_assign() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let mut v2 = Vec3::new(2.0, 1.0, 0.0);
        v2 /= v1;

        assert_eq!(v2.x, 2.0/1.0);
        assert_eq!(v2.y, 1.0/2.0);
        assert_eq!(v2.z, 0.0/3.0);
    }

    #[test]
    fn vector_cross1() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let v3 = v1.cross(v2);
        assert_eq!(v3.x, 0.0);
        assert_eq!(v3.y, 0.0);
        assert_eq!(v3.z, 1.0);
    }

    #[test]
    fn vector_cross2() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let v3 = v2.cross(v1);
        assert_eq!(v3.x, 0.0);
        assert_eq!(v3.y, 0.0);
        assert_eq!(v3.z, -1.0);
    }

    #[test]
    fn vector_normalized() {
        let v1 = Vec3::new(3.0, 2.0, 5.0);
        let norm = v1.normalized();
        let len = norm.length();
        assert_eq!(len, 1.0);
    }

    #[test]
    fn vector_neg() {
        let v1 = Vec3::new(3.0, 2.0, -5.0);
        let v2 = -v1;
        assert_eq!(v2.x, -3.0);
        assert_eq!(v2.y, -2.0);
        assert_eq!(v2.z, 5.0);
    }

}