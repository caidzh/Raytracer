use crate::vec3::Vector;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
    pub time: f64,
}

impl Ray {
    pub fn new(a: Vector, b: Vector, c: f64) -> Self {
        Self {
            origin: a,
            direction: b,
            time: c,
        }
    }
    pub fn at(&self, t: f64) -> Vector {
        Vector {
            x: self.origin.x + t * self.direction.x,
            y: self.origin.y + t * self.direction.y,
            z: self.origin.z + t * self.direction.z,
        }
    }
}
