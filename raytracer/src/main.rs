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
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::rtweekend::{random_double, random_double_range};
use crate::sphere::Sphere;
use crate::vec3::Vector;

fn main() {
    let mut world: HittableList = Default::default();

    let ground_material = Rc::new(Lambertian::new(Vector::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vector::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Vector::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;
                if choose_mat < 0.8 {
                    let albedo = Vector::random() * Vector::random();
                    let center2 = center + Vector::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )))
                } else if choose_mat < 0.95 {
                    let albedo = Vector::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)))
                } else {
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Vector::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Vector::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Vector::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Vector::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Vector::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam: Camera = Default::default();

    cam.render(&world);

    exit(0);
}
