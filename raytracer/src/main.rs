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
pub mod quad;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod texture;
pub mod vec3;

use bvh::BvhNode;
use texture::{CheckerTexture, ImageTexture};

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::quad::Quad;
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
fn quads() {
    let mut world: HittableList = Default::default();

    let left_red = Arc::new(Lambertian::new(Vector::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Vector::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Vector::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Vector::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Vector::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Vector::new(-3.0, -2.0, 5.0),
        Vector::new(0.0, 0.0, -4.0),
        Vector::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(-2.0, -2.0, 0.0),
        Vector::new(4.0, 0.0, 0.0),
        Vector::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(3.0, -2.0, 1.0),
        Vector::new(0.0, 0.0, 4.0),
        Vector::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(-2.0, 3.0, 1.0),
        Vector::new(4.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(-2.0, -3.0, 5.0),
        Vector::new(4.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -4.0),
        lower_teal,
    )));
    let mut cam: Camera = Default::default();
    cam.render(world);
}
fn simple_light() {
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
    let difflight = Arc::new(DiffuseLight::color_new(Vector::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new(
        Vector::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(3.0, 1.0, -2.0),
        Vector::new(2.0, 0.0, 0.0),
        Vector::new(0.0, 2.0, 0.0),
        difflight,
    )));
    let mut cam: Camera = Default::default();
    cam.render(world);
}
fn cornell_box() {
    let mut world: HittableList = Default::default();
    let red = Arc::new(Lambertian::new(Vector::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vector::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vector::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::color_new(Vector::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Vector::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 555.0, 0.0),
        Vector::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 555.0, 0.0),
        Vector::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(343.0, 554.0, 332.0),
        Vector::new(-130.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(555.0, 555.0, 555.0),
        Vector::new(-555.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(0.0, 0.0, 555.0),
        Vector::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 555.0, 0.0),
        white,
    )));
    let mut cam: Camera = Default::default();
    cam.render(world);
}
fn main() {
    let f = random_double_range(0.0, 1.0);
    if f < 0.0001 {
        bouncing_spheres();
        checkered_spheres();
        earth();
        perlin_spheres();
        quads();
        simple_light();
    } else {
        cornell_box();
    }
    exit(0);
}
