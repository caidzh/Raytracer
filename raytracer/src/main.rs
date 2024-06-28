use std::process::exit;
use std::rc::Rc;
pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;
use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Vector;

fn main() {
    let mut world: HittableList = Default::default();
    let sphere1: Sphere = Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5);
    let sphere2: Sphere = Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0);
    world.add(Rc::new(sphere1));
    world.add(Rc::new(sphere2));

    let mut cam: Camera = Default::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    cam.render(&world);

    exit(0);
}
