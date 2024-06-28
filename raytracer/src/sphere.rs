use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vector;

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
}
impl Sphere {
    pub fn new(c: Vector, r: f64) -> Self {
        Self {
            center: (c),
            radius: (r),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let oc: Vector = self.center - r.origin;
        let a: f64 = r.direction.length_square();
        let h: f64 = r.direction.dot(&oc);
        let c: f64 = oc.length_square() - self.radius * self.radius;

        let discriminant: f64 = h * h - a * c;
        if discriminant < 0.0 {
            false
        } else {
            let sqrtd: f64 = discriminant.sqrt();
            let root: f64 = (h - sqrtd) / a;
            if !ray_t.surrounds(root) {
                let root: f64 = (h + sqrtd) / a;
                if !ray_t.surrounds(root) {
                    return false;
                }
            }
            rec.t = root;
            rec.p = r.at(rec.t);
            let outward_normal: Vector = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, &outward_normal);

            true
        }
    }
}
