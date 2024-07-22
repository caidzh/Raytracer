use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::rtweekend::{random_double, random_double_range, INFINITY};
use crate::vec3::Vector;

pub struct Triangle {
    q: Vector,
    u: Vector,
    v: Vector,
    w: Vector,
    normal: Vector,
    mat: Arc<dyn Material>,
    bbox: AABB,
    d: f64,
    area: f64,
}

impl Triangle {
    pub fn new(a: Vector, b: Vector, c: Vector, d: Arc<dyn Material>) -> Self {
        let mut val: Triangle = Self {
            q: a,
            u: b - a,
            v: c - a,
            w: Default::default(),
            normal: Default::default(),
            mat: d,
            bbox: Default::default(),
            d: 0.0,
            area: 0.0,
        };
        let n = val.u.cross(&val.v);
        val.normal = n.unit();
        val.d = val.normal.dot(&a);
        val.w = n / n.length_square();
        val.set_bounding_box();
        val.area = n.length() / 2.0;
        val
    }
    pub fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = AABB::point_new(&self.q, &(self.q + self.u));
        let bbox_diagonal2 = AABB::point_new(&self.q, &(self.q + self.v));
        self.bbox = AABB::box_new(&bbox_diagonal1, &bbox_diagonal2);
    }
    fn is_interior(a: f64, b: f64) -> Option<HitRecord> {
        let mut val: HitRecord = Default::default();
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a)
            || !unit_interval.contains(b)
            || !unit_interval.contains(a + b)
        {
            return None;
        }
        val.u = a;
        val.v = b;
        Some(val)
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&r.direction);
        if denom.abs() < 1e-8 {
            return None;
        }
        let t = (self.d - self.normal.dot(&r.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));
        if let Some(mut val) = Self::is_interior(alpha, beta) {
            val.t = t;
            val.p = intersection;
            val.mat = Some(self.mat.clone());
            val.set_face_normal(r, &self.normal);
            Some(val)
        } else {
            None
        }
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn pdf_value(&self, origin: Vector, direction: Vector) -> f64 {
        if let Some(rec) = self.hit(
            &Ray::new(origin, direction, 0.0),
            &Interval::new(0.001, INFINITY),
        ) {
            let distance_squared = rec.t * rec.t * direction.length_square();
            let cosine = (direction.dot(&rec.normal) / direction.length()).abs();
            distance_squared / (cosine * self.area)
        } else {
            0.0
        }
    }
    fn random(&self, origin: Vector) -> Vector {
        let a = random_double();
        let b = random_double_range(0.0, 1.0 - a);
        let p = self.q + self.u * a + self.v * b;
        p - origin
    }
}
