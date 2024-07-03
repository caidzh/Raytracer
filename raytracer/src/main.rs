use std::process::exit;
use std::sync::Arc;
pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod image;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod texture;
pub mod vec3;

use bvh::BvhNode;
use texture::{CheckerTexture, ImageTexture};

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::rtweekend::{random_double, random_double_range};
use crate::sphere::Sphere;
use crate::texture::NoiseTexture;
use crate::vec3::Vector;

fn bouncing_spheres() {
    let mut world: HittableList = Default::default();
    let checker = Arc::new(CheckerTexture::color_new(
        0.32,
        Vector::new(0.2, 0.3, 0.1),
        Vector::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::arc_new(checker)),
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
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    let albedo = Vector::random() * Vector::random();
                    let center2 = center + Vector::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )))
                } else if choose_mat < 0.95 {
                    let albedo = Vector::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)))
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vector::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Vector::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vector::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Vector::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vector::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let world_node = BvhNode::initialise(&mut world);
    world.initialise(Arc::new(world_node));

    let mut cam: Camera = Default::default();

    cam.render(world);
}

fn checkered_spheres() {
    let mut world: HittableList = Default::default();
    let checker = Arc::new(CheckerTexture::color_new(
        0.32,
        Vector::new(0.2, 0.3, 0.1),
        Vector::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vector::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::arc_new(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vector::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::arc_new(checker)),
    )));

    let mut cam: Camera = Camera::default();

    cam.render(world);
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::arc_new(earth_texture));
    let globe = Arc::new(Sphere::new(Vector::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut cam: Camera = Default::default();
    let mut world = HittableList::default();
    world.initialise(globe);
    cam.render(world);
}
fn perlin_spheres() {
    let mut world: HittableList = Default::default();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::arc_new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vector::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::arc_new(pertext)),
    )));
    let mut cam: Camera = Default::default();
    cam.render(world);
}
fn main() {
    let f = random_double_range(0.0, 1.0);
    if f < 0.001 {
        bouncing_spheres();
        checkered_spheres();
        earth();
    } else {
        perlin_spheres();
    }
    exit(0);
}
