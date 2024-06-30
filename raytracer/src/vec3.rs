use crate::rtweekend::{random_double, random_double_range};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { x: a, y: b, z: c }
    }
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
    pub fn length_square(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn unit(&self) -> Vector {
        let len: f64 = self.length();
        Vector {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
    pub fn print(&self) {
        println!("x:{} y:{} z:{}", self.x, self.y, self.z);
    }
    pub fn random() -> Vector {
        Vector::new(random_double(), random_double(), random_double())
    }
    pub fn random_range(min: f64, max: f64) -> Vector {
        Vector::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }
    pub fn random_in_unit_sphere() -> Vector {
        loop {
            let p: Vector = Self::random_range(-1.0, 1.0);
            if p.length_square() < 1.0 {
                return p;
            }
        }
    }
    pub fn random_unit_vector() -> Vector {
        Self::random_in_unit_sphere().unit()
    }
    pub fn random_on_hemisphere(normal: &Vector) -> Vector {
        let on_unit_sphere: Vector = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            on_unit_sphere * -1.0
        }
    }
    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
    pub fn reflect(v: &Vector, n: &Vector) -> Vector {
        let product = v.dot(n);
        (*v) - (*n) * (product * 2.0)
    }
    pub fn refract(uv: &Vector, n: &Vector, etai_over_etat: f64) -> Vector {
        let cos_theta = (n.dot(&((*uv) * -1.0))).min(1.0);
        let r_out_perp = ((*uv) + (*n) * cos_theta) * etai_over_etat;
        let r_out_parallel = (*n) * (-((1.0 - r_out_perp.length_square()).abs().sqrt()));
        r_out_perp + r_out_parallel
    }
}
impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
