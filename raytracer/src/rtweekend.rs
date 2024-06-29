use rand::distributions::Uniform;
use rand::Rng;

// pub const pi: f64 = 3.1415926535897932385;
pub const INFINITY: f64 = f64::INFINITY;

// fn degrees_to_radians(degrees: f64) -> f64 {
//     return degrees * pi / 180.0;
// }

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
