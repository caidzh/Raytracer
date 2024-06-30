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
use crate::material::Lambertian;
use crate::sphere::Sphere;
use crate::vec3::Vector;
use crate::rtweekend::PI;

fn main() {
    let mut world: HittableList = Default::default();

    let r=(PI/4.0).cos();

    let material_left=Rc::new(Lambertian::new(Vector::new(0.0,0.0,1.0)));
    let material_right=Rc::new(Lambertian::new(Vector::new(1.0,0.0,0.0)));

    world.add(Rc::new(Sphere::new(Vector::new(-r,0.0,-1.0),r,material_left)));
    world.add(Rc::new(Sphere::new(Vector::new(r,0.0,-1.0),r,material_right)));

    let mut cam: Camera = Default::default();

    cam.render(&world);

    exit(0);
}
