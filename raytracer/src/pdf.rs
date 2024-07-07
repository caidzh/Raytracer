use std::sync::Arc;

use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::rtweekend::PI;
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
