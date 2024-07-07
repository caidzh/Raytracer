use std::sync::Arc;

use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::rtweekend::{random_double, PI};
use crate::vec3::Vector;

pub trait Pdf: Send + Sync {
    fn value(&self, direction: Vector) -> f64;
    fn generate(&self) -> Vector;
}

pub struct SpherePdf {}

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vector) -> f64 {
        1.0 / (4.0 * PI)
    }
    fn generate(&self) -> Vector {
        Vector::random_unit_vector()
    }
}

#[derive(Default)]
pub struct CosinePdf {
    pub uvw: Onb,
}

impl CosinePdf {
    pub fn init(&mut self, w: Vector) {
        self.uvw.build_from_w(w);
    }
    pub fn new(w: Vector) -> Self {
        let mut val: Self = Default::default();
        val.uvw.build_from_w(w);
        val
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vector) -> f64 {
        let cosine_theta = direction.unit().dot(&self.uvw.axis[2]);
        (cosine_theta / PI).max(0.0)
    }
    fn generate(&self) -> Vector {
        self.uvw.vec_local(Vector::random_cosine_direction())
    }
}

pub struct HittablePdf {
    pub objects: Arc<dyn Hittable>,
    pub origin: Vector,
}

impl HittablePdf {
    pub fn new(o: Arc<dyn Hittable>, v: Vector) -> Self {
        Self {
            objects: o,
            origin: v,
        }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: Vector) -> f64 {
        self.objects.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vector {
        self.objects.random(self.origin)
    }
}

pub struct MixturePdf {
    pub p: [Arc<dyn Pdf>; 2],
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: Vector) -> f64 {
        self.p[0].value(direction) * 0.5 + self.p[1].value(direction) * 0.5
    }
    fn generate(&self) -> Vector {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
