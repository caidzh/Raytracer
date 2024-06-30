use std::process::exit;
use std::rc::Rc;
pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::Vector;

fn main() {
    let mut world: HittableList = Default::default();

    let material_ground = Rc::new(Lambertian::new(Vector::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vector::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.50));
    let material_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Rc::new(Metal::new(Vector::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        Vector::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Vector::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Vector::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Vector::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Rc::new(Sphere::new(
        Vector::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut cam: Camera = Default::default();

    cam.render(&world);

    exit(0);
}
