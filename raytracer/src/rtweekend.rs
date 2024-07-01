use rand::distributions::Uniform;
use rand::Rng;

pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0.0, 1.0);
    rng.sample(range)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(min, max);
    rng.sample(range)
}

pub fn random_int_range(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}
