use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
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
