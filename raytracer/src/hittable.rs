use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::rtweekend::{degrees_to_radians, INFINITY};
use crate::vec3::Vector;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Vector,
    pub normal: Vector,
    pub mat: Option<Arc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(a: Vector, b: Vector, c: f64, d: bool) -> Self {
        Self {
            p: a,
            normal: b,
            mat: None,
            t: c,
            u: 0.0,
            v: 0.0,
            front_face: d,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector) {
        // println!("{}",r.direction.dot(outward_normal));
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        match self.front_face {
            true => {
                self.normal = *outward_normal;
            }
            false => {
                self.normal = (*outward_normal) * -1.0;
            }
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vector,
    bbox: AABB,
}

impl Translate {
    pub fn new(a: Arc<dyn Hittable>, b: &Vector) -> Self {
        let mut val: Translate = Self {
            object: a,
            offset: Vector::default(),
            bbox: AABB::default(),
        };
        val.offset = *b;
        val.bbox = val.object.bounding_box() + (*b);
        val
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let offset_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        if let Some(mut temp_rec) = self.object.hit(&offset_r, ray_t) {
            temp_rec.p = temp_rec.p + self.offset;
            return Some(temp_rec);
        }
        None
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(obj: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let mut val = Self {
            object: obj,
            sin_theta: radians.sin(),
            cos_theta: radians.cos(),
            bbox: Default::default(),
        };
        val.bbox = val.object.bounding_box();
        let mut min = Vector::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vector::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * val.bbox.x.max + (1.0 - (i as f64)) * val.bbox.x.min;
                    let y = j as f64 * val.bbox.y.max + (1.0 - (j as f64)) * val.bbox.y.min;
                    let z = k as f64 * val.bbox.z.max + (1.0 - (k as f64)) * val.bbox.z.min;

                    let newx = val.cos_theta * x + val.sin_theta * z;
                    let newz = -val.sin_theta * x + val.cos_theta * z;

                    let tester = Vector::new(newx, y, newz);
                    min.x = min.x.min(tester.x);
                    min.y = min.y.min(tester.y);
                    min.z = min.z.min(tester.z);
                    max.x = max.x.max(tester.x);
                    max.y = max.y.max(tester.y);
                    max.z = max.z.max(tester.z);
                }
            }
        }

        val.bbox = AABB::point_new(&min, &max);
        val
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.x = self.cos_theta * r.origin.x - self.sin_theta * r.origin.z;
        origin.z = self.sin_theta * r.origin.x + self.cos_theta * r.origin.z;

        direction.x = self.cos_theta * r.direction.x - self.sin_theta * r.direction.z;
        direction.z = self.sin_theta * r.direction.x + self.cos_theta * r.direction.z;

        let rotated_r = Ray::new(origin, direction, r.time);
        if let Some(mut rec) = self.object.hit(&rotated_r, ray_t) {
            let mut p = rec.p;
            p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
            p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;
            let mut normal = rec.normal;
            normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;
            rec.p = p;
            rec.normal = normal;
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
