use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone)]
pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub bbox: AABB,
}

impl BvhNode {
    pub fn initialise(list: &mut HittableList) -> Self {
        let len = list.size();
        Self::new(&mut list.objects, 0, len)
    }
    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = AABB::default();
        for object_index in start..end {
            bbox = AABB::box_new(&bbox, &objects[object_index].bounding_box());
        }
        let axis = bbox.longest_axis();
        let object_span: usize = end - start;
        if object_span == 1 {
            let left = &objects[start];
            let right = &objects[start];
            return Self {
                left: left.clone(),
                right: right.clone(),
                bbox: AABB::box_new(&left.bounding_box(), &right.bounding_box()),
            };
        } else if object_span == 2 {
            let left = &objects[start];
            let right = &objects[start + 1];
            return Self {
                left: left.clone(),
                right: right.clone(),
                bbox: AABB::box_new(&left.bounding_box(), &right.bounding_box()),
            };
        } else {
            objects[start..end].sort_by(|a, b| Self::box_compare(a, b, axis));
            let mid = start + object_span / 2;
            let left = Self::new(objects, start, mid);
            let right = Self::new(objects, mid, end);
            return Self {
                left: Rc::new(left.clone()),
                right: Rc::new(right.clone()),
                bbox: AABB::box_new(&left.bounding_box(), &right.bounding_box()),
            };
        }
    }
    pub fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis_index: u32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap_or(Ordering::Equal)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        if !self.bbox.hit(r, ray_t) {
            return None;
        }
        let mut closest_so_far = ray_t.max;
        if let Some(temp_rec) = self.left.hit(r, ray_t) {
            closest_so_far = temp_rec.t;
            rec = Some(temp_rec);
        }
        if let Some(temp_rec) = self.right.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
            rec = Some(temp_rec);
        }
        rec
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
