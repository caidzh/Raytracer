use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::random_double;
use crate::ray::Ray;
use crate::rtweekend::INFINITY;
use crate::texture::Texture;
use crate::vec3::Vector;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(b: Arc<dyn Hittable>, d: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }
    pub fn color_new(b: Arc<dyn Hittable>, d: f64, albedo: Vector) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::color_new(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, &Interval::universe()) {
            if let Some(mut rec2) = self
                .boundary
                .hit(r, &Interval::new(rec1.t + 0.0001, INFINITY))
            {
                if rec1.t < ray_t.min {
                    rec1.t = ray_t.min;
                }
                if rec2.t > ray_t.max {
                    rec2.t = ray_t.max;
                }
                if rec1.t >= rec2.t {
                    None
                } else {
                    if rec1.t < 0.0 {
                        rec1.t = 0.0;
                    }
                    let ray_length = r.direction.length();
                    let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                    let hit_distance = self.neg_inv_density * random_double().ln();
                    if hit_distance > distance_inside_boundary {
                        return None;
                    }
                    let mut rec: HitRecord = Default::default();
                    rec.t = rec1.t + hit_distance / ray_length;
                    rec.p = r.at(rec.t);
                    rec.normal = Vector::new(1.0, 0.0, 0.0);
                    rec.front_face = true;
                    rec.mat = Some(self.phase_function.clone());
                    Some(rec)
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
