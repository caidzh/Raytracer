use crate::vec3::Vector;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(a: Vector, b: Vector) -> Self {
        Self {
            origin: a,
            direction: b,
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
