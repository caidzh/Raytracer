use crate::vec3::Vector;

#[derive(Default)]
pub struct Onb {
    pub axis: [Vector; 3],
}

impl Onb {
    pub fn build_from_w(&mut self, w: Vector) {
        let unit_w = w.unit();
        let a = if unit_w.x.abs() > 0.9 {
            Vector::new(0.0, 1.0, 0.0)
        } else {
            Vector::new(1.0, 0.0, 0.0)
        };
        let v = unit_w.cross(&a).unit();
        let u = unit_w.cross(&v);
        self.axis[0] = u;
        self.axis[1] = v;
        self.axis[2] = unit_w;
    }
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vector {
        self.axis[0] * a + self.axis[1] * b + self.axis[2] * c
    }
    pub fn vec_local(&self, a: Vector) -> Vector {
        self.axis[0] * a.x + self.axis[1] * a.y + self.axis[2] * a.z
    }
}
