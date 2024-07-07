use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::onb::Onb;
use crate::ray::Ray;
use crate::rtweekend::{random_double, INFINITY, PI};
use crate::vec3::Vector;

use std::sync::Arc;

pub struct Sphere {
    pub center1: Vector,
    pub radius: f64,
    pub mat: Option<Arc<dyn Material>>,
    pub is_moving: bool,
    pub center_vec: Vector,
    pub bbox: AABB,
}
impl Sphere {
    pub fn new(c: Vector, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center1: (c),
            radius: (r),
            mat: Some(m),
            is_moving: false,
            center_vec: Vector::new(0.0, 0.0, 0.0),
            bbox: {
                let rvec = Vector::new(r, r, r);
                AABB::point_new(&(c - rvec), &(c + rvec))
            },
        }
    }
    pub fn new_moving(c: Vector, d: Vector, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center1: (c),
            radius: (r),
            mat: Some(m),
            is_moving: (true),
            center_vec: (d - c),
            bbox: {
                let rvec = Vector::new(r, r, r);
                let box1 = AABB::point_new(&(c - rvec), &(c + rvec));
                let box2 = AABB::point_new(&(d - rvec), &(d + rvec));
                AABB::box_new(&box1, &box2)
            },
        }
    }
    pub fn sphere_center(&self, time: f64) -> Vector {
        self.center1 + self.center_vec * time
    }
    pub fn get_sphere_uv(p: &Vector) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = f64::atan2(-p.z, p.x) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
    pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vector {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);
        let phi = 2.0 * PI * r1;
        let x = phi.cos() * ((1.0 - z * z).sqrt());
        let y = phi.sin() * ((1.0 - z * z).sqrt());
        Vector::new(x, y, z)
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
        let center = if self.is_moving {
            self.sphere_center(r.time)
        } else {
            self.center1
        };
        let oc: Vector = center - r.origin;
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
            let outward_normal: Vector = (rec.p - self.center1) / self.radius;
            rec.set_face_normal(r, &outward_normal);
            (rec.u, rec.v) = Self::get_sphere_uv(&outward_normal);
            rec.mat = self.mat.clone();

            Some(rec)
        }
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn pdf_value(&self, origin: Vector, direction: Vector) -> f64 {
        if let Some(_rec) = self.hit(
            &Ray::new(origin, direction, 0.0),
            &Interval::new(0.001, INFINITY),
        ) {
            let cos_theta_max = (1.0
                - self.radius * self.radius / ((self.center1 - origin).length_square()))
            .sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }
    fn random(&self, origin: Vector) -> Vector {
        let direction = self.center1 - origin;
        let distance_squared = direction.length_square();
        let mut uvw: Onb = Default::default();
        uvw.build_from_w(direction);
        uvw.vec_local(Self::random_to_sphere(self.radius, distance_squared))
    }
}
