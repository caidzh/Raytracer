use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    pub bbox: AABB,
}
impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }
    pub fn initialise(&mut self, object: Arc<dyn Hittable>) {
        self.add(object);
    }
    pub fn size(&self) -> usize {
        self.objects.len()
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        let object_ref = &object;
        self.objects.push(Arc::clone(&object));
        self.bbox = AABB::box_new(&self.bbox, &object_ref.bounding_box());
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far: f64 = ray_t.max;
        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
