use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::image::RtwImage;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vector;

pub struct NormalMapping {
    pub image: RtwImage,
    q: Vector,
    u: Vector,
    v: Vector,
    w: Vector,
    normal: Vector,
    mat: Arc<dyn Material>,
    bbox: AABB,
    d: f64,
}

impl NormalMapping {
    pub fn new(
        image_filename: &str,
        a: Vector,
        b: Vector,
        c: Vector,
        d: Arc<dyn Material>,
    ) -> Self {
        let mut val = Self {
            image: RtwImage::new(image_filename),
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
        let mut val: HitRecord = Default::default();
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return None;
        }
        val.u = a;
        val.v = b;
        Some(val)
    }
    pub fn get_normal(&self, u: f64, v: f64) -> Vector {
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);
        let i = (self.image.image_width as f64 * u) as i32;
        let j = (self.image.image_height as f64 * v) as i32;
        let pixel = self.image.pixel_data(i, j);
        Vector::new(
            (pixel[0] as f64 / 255.99) * 2.0 - 1.0,
            (pixel[1] as f64 / 255.99) * 2.0 - 1.0,
            (pixel[2] as f64 / 255.99) * 2.0 - 1.0,
        )
    }
}

impl Hittable for NormalMapping {
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
            val.normal = self.get_normal(alpha, beta) + self.normal;
            let normal = val.normal;
            val.set_face_normal(r, &normal);
            Some(val)
        } else {
            None
        }
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
