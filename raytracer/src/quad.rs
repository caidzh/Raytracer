use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vector;

pub struct Quad {
    q: Vector,
    u: Vector,
    v: Vector,
    w: Vector,
    normal: Vector,
    mat: Arc<dyn Material>,
    bbox: AABB,
    d: f64,
}

impl Quad {
    pub fn new(a: Vector, b: Vector, c: Vector, d: Arc<dyn Material>) -> Self {
        let mut val: Quad = Self {
            q: a,
            u: b,
            v: c,
            w: Default::default(),
            normal: Default::default(),
            mat: d,
            bbox: Default::default(),
            d: 0.0,
        };
        let n = b.cross(&c);
        val.normal = n.unit();
        val.d = val.normal.dot(&a);
        val.w = n / n.length_square();
        val.set_bounding_box();
        val
    }
    pub fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = AABB::point_new(&self.q, &(self.q + self.u + self.v));
        let bbox_diagonal2 = AABB::point_new(&(self.q + self.u), &(self.q + self.v));
        self.bbox = AABB::box_new(&bbox_diagonal1, &bbox_diagonal2);
    }
    fn is_interior(a: f64, b: f64) -> Option<HitRecord> {
        let mut val:HitRecord=Default::default();
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return None;
        }
        val.u = a;
        val.v = b;
        Some(val)
    }
}

impl Hittable for Quad {
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
}
