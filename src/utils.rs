use rand::{thread_rng, Rng};

/**
 *  Random number between [0, 1)
 */
pub fn random_f32() -> f32 {
    let mut rng = thread_rng();
    rng.gen()
}

/**
 *  Random number between [min, max)
 */
pub fn random_range_f32(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    }
    else if x > max {
        max
    }
    else {
        x
    }
}