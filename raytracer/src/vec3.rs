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
