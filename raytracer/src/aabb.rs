use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vector;

#[derive(Copy, Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }
}

impl AABB {
    pub fn new(a: Interval, b: Interval, c: Interval) -> Self {
        let val = Self { x: a, y: b, z: c };
        val.pad_to_minimums();
        val
    }
    pub fn point_new(a: &Vector, b: &Vector) -> Self {
        let val = Self {
            x: if a.x <= b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },
            y: if a.y <= b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },
            z: if a.z <= b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        };
        val.pad_to_minimums();
        val
    }
    pub fn box_new(a: &AABB, b: &AABB) -> Self {
        Self {
            x: Interval::interval_new(a.x, b.x),
            y: Interval::interval_new(a.y, b.y),
            z: Interval::interval_new(a.z, b.z),
        }
    }
    pub fn axis_interval(&self, n: u32) -> Interval {
        if n == 1 {
            self.y
        } else if n == 2 {
            self.z
        } else {
            self.x
        }
    }
    fn pad_to_minimums(&self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z.expand(delta);
        }
    }
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let mut ray_t_copy = Interval::new(ray_t.min, ray_t.max);
        let ray_orig = r.origin;
        let ray_dir = r.direction;
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir.at(axis);
            let t0 = (ax.min - ray_orig.at(axis)) * adinv;
            let t1 = (ax.max - ray_orig.at(axis)) * adinv;
            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t_copy.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t_copy.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t_copy.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t_copy.max = t0;
                }
            }
            if ray_t_copy.max <= ray_t_copy.min {
                return false;
            }
        }
        true
    }
    pub fn longest_axis(&self) -> u32 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }
}
