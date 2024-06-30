use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vector;

use std::rc::Rc;

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub mat: Option<Rc<dyn Material>>,
}
impl Sphere {
    pub fn new(c: Vector, r: f64, m: Rc<dyn Material>) -> Self {
        Self {
            center: (c),
            radius: (r),
            mat: Some(m),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec: HitRecord = HitRecord::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 0.0),
            0.0,
            false,
        );
        let oc: Vector = self.center - r.origin;
        let a: f64 = r.direction.length_square();
        let h: f64 = r.direction.dot(&oc);
        let c: f64 = oc.length_square() - self.radius * self.radius;

        let discriminant: f64 = h * h - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrtd: f64 = discriminant.sqrt();
            let mut root: f64 = (h - sqrtd) / a;
            if !ray_t.surrounds(root) {
                root = (h + sqrtd) / a;
                if !ray_t.surrounds(root) {
                    return None;
                }
            }
            rec.t = root;
            rec.p = r.at(rec.t);
            let outward_normal: Vector = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, &outward_normal);
            rec.mat = self.mat.clone();

            Some(rec)
        }
    }
}
