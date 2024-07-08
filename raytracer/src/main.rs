use std::process::exit;
use std::sync::Arc;
pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod constant_medium;
pub mod hittable;
pub mod hittable_list;
pub mod image;
pub mod interval;
pub mod material;
pub mod onb;
pub mod pdf;
pub mod perlin;
pub mod quad;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod texture;
pub mod vec3;

use bvh::BvhNode;
use hittable::{RotateY, Translate};
use quad::box_object;
use texture::{CheckerTexture, ImageTexture};

use crate::camera::Camera;
use crate::constant_medium::ConstantMedium;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::quad::Quad;
use crate::rtweekend::{random_double, random_double_range};
use crate::sphere::Sphere;
use crate::texture::NoiseTexture;
use crate::vec3::Vector;

fn bouncing_spheres() {
    let mut world: HittableList = Default::default();
    let mut lights: HittableList = Default::default();
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
        material2.clone(),
    )));

    let material3 = Arc::new(Metal::new(Vector::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vector::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let world_node = BvhNode::initialise(&mut world);
    world.initialise(Arc::new(world_node));
    lights.add(Arc::new(Sphere::new(
        Vector::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let mut cam: Camera = Default::default();

    cam.render(world, Arc::new(lights));
}

fn checkered_spheres() {
    let mut world: HittableList = Default::default();
    let lights: HittableList = Default::default();
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

    cam.render(world, Arc::new(lights));
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::arc_new(earth_texture));
    let globe = Arc::new(Sphere::new(Vector::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut cam: Camera = Default::default();
    let mut world = HittableList::default();
    let lights: HittableList = Default::default();
    world.initialise(globe);
    cam.render(world, Arc::new(lights));
}
fn perlin_spheres() {
    let mut world: HittableList = Default::default();
    let lights: HittableList = Default::default();
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
    cam.render(world, Arc::new(lights));
}
fn quads() {
    let mut world: HittableList = Default::default();
    let lights: HittableList = Default::default();

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
    cam.render(world, Arc::new(lights));
}
fn simple_light() {
    let mut world: HittableList = Default::default();
    let lights: HittableList = Default::default();
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
    cam.render(world, Arc::new(lights));
}
fn cornell_box() {
    let mut world: HittableList = Default::default();
    let mut lights: HittableList = Default::default();
    let red = Arc::new(Lambertian::new(Vector::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vector::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vector::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::color_new(Vector::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Vector::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 555.0),
        Vector::new(0.0, 555.0, 0.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(0.0, 0.0, 555.0),
        Vector::new(0.0, 0.0, -555.0),
        Vector::new(0.0, 555.0, 0.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(213.0, 554.0, 227.0),
        Vector::new(130.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 105.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(0.0, 555.0, 0.0),
        Vector::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(0.0, 0.0, 555.0),
        Vector::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(555.0, 0.0, 555.0),
        Vector::new(-555.0, 0.0, 0.0),
        Vector::new(0.0, 555.0, 0.0),
        white.clone(),
    )));
    let box1 = box_object(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(165.0, 330.0, 165.0),
        white,
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vector::new(265.0, 0.0, 295.0)));
    world.add(box1);
    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vector::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));
    // let box2 = box_object(
    //     Vector::new(0.0, 0.0, 0.0),
    //     Vector::new(165.0, 165.0, 165.0),
    //     white,
    // );
    // let box2 = Arc::new(RotateY::new(box2, -18.0));
    // let box2 = Arc::new(Translate::new(box2, &Vector::new(130.0, 0.0, 65.0)));
    // world.add(box2);
    let m: Arc<dyn Material> = Arc::new(DiffuseLight::color_new(Vector::new(15.0, 15.0, 15.0)));
    lights.add(Arc::new(Quad::new(
        Vector::new(343.0, 554.0, 332.0),
        Vector::new(-130.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -105.0),
        m.clone(),
    )));
    lights.add(Arc::new(Sphere::new(
        Vector::new(190.0, 90.0, 190.0),
        90.0,
        m,
    )));
    let mut cam: Camera = Default::default();
    cam.render(world, Arc::new(lights));
}
fn cornell_smoke() {
    let mut world: HittableList = Default::default();
    let mut lights: HittableList = Default::default();
    let red = Arc::new(Lambertian::new(Vector::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vector::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vector::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::color_new(Vector::new(7.0, 7.0, 7.0)));

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
    lights.add(Arc::new(Quad::new(
        Vector::new(113.0, 554.0, 127.0),
        Vector::new(330.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(0.0, 555.0, 0.0),
        Vector::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vector::new(0.0, 0.0, 555.0),
        Vector::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 555.0, 0.0),
        white.clone(),
    )));
    let box1 = box_object(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vector::new(265.0, 0.0, 295.0)));
    world.add(Arc::new(ConstantMedium::color_new(
        box1,
        0.01,
        Vector::new(0.0, 0.0, 0.0),
    )));
    let box2 = box_object(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, &Vector::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::color_new(
        box2,
        0.01,
        Vector::new(1.0, 1.0, 1.0),
    )));
    let mut cam: Camera = Default::default();
    cam.render(world, Arc::new(lights));
}
fn final_scene() {
    let mut boxes1: HittableList = Default::default();
    let mut lights: HittableList = Default::default();
    let ground = Arc::new(Lambertian::new(Vector::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(box_object(
                Vector::new(x0, y0, z0),
                Vector::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }
    let mut world: HittableList = Default::default();

    world.add(Arc::new(BvhNode::initialise(&mut boxes1)));

    let light = Arc::new(DiffuseLight::color_new(Vector::new(7.0, 7.0, 7.0)));
    lights.add(Arc::new(Quad::new(
        Vector::new(123.0, 554.0, 147.0),
        Vector::new(300.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 265.0),
        light.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Vector::new(123.0, 554.0, 147.0),
        Vector::new(300.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Vector::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vector::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(Vector::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        Vector::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Vector::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vector::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundary = Arc::new(Sphere::new(
        Vector::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::color_new(
        boundary,
        0.2,
        Vector::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(Sphere::new(
        Vector::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::color_new(
        boundary,
        0.0001,
        Vector::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::arc_new(Arc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    world.add(Arc::new(Sphere::new(
        Vector::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new(
        Vector::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::arc_new(pertext)),
    )));

    let mut boxes2: HittableList = Default::default();
    let white = Arc::new(Lambertian::new(Vector::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Vector::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::initialise(&mut boxes2)),
            15.0,
        )),
        &Vector::new(-100.0, 270.0, 395.0),
    )));

    let mut cam: Camera = Default::default();
    cam.render(world, Arc::new(lights));
}
fn main() {
    let f = random_double_range(0.0, 1.0);
    if f < 0.000001 {
        checkered_spheres();
        earth();
        perlin_spheres();
        quads();
        simple_light();
        cornell_box();
        cornell_smoke();
        bouncing_spheres();
    } else {
        final_scene();
    }
    exit(0);
}
