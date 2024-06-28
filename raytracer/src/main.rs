use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::f64;
use std::{fs::File, process::exit};
pub mod ray;
pub mod vec3;
use crate::ray::Ray;
use crate::vec3::Vector;

fn ray_color(r: &Ray) -> Vector {
    let t: f64 = hit_sphere(&Vector::new(0.0, 0.0, -1.0), 0.5, r);
    match t > 0.0 {
        true => {
            let v: Vector = r.at(t) - Vector::new(0.0, 0.0, -1.0);
            let n: Vector = v.unit();
            Vector::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5 * 255.99
        }
        false => {
            let unit_direction: Vector = r.direction.unit();
            let a = 0.5 * (unit_direction.y + 1.0);
            let white: Vector = Vector::new(1.0, 1.0, 1.0);
            let blue: Vector = Vector::new(0.5, 0.7, 1.0);
            (white * (1.0 - a) + blue * a) * 255.99
        }
    }
}

fn hit_sphere(center: &Vector, radius: f64, r: &Ray) -> f64 {
    let oc: Vector = (*center) - r.origin;
    let a: f64 = r.direction.length_square();
    let h:f64=r.direction.dot(&oc);
    let c: f64 = oc.length_square() - radius * radius;
    let discriminant: f64 = h*h-a*c;
    match discriminant < 0.0 {
        true => -1.0,
        false => (h - discriminant.sqrt()) / a,
    }
}

fn main() {
    let path = std::path::Path::new("output/book1/image5.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio).round() as u32;
    let image_height = if image_height < 1 { 1 } else { image_height };
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / (image_height as f64));
    let camera_center: Vector = Vector::new(0.0, 0.0, 0.0);

    let viewport_u: Vector = Vector::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vector = Vector::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u: Vector = viewport_u / (image_width as f64);
    let pixel_delta_v: Vector = viewport_v / (image_height as f64);

    let viewport_upper_left =
        camera_center - Vector::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, j);
            let pixel_center =
                pixel00_loc + (pixel_delta_u * (i as f64)) + (pixel_delta_v * (j as f64));
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_direction);
            let pixel_color: Vector = ray_color(&r);
            *pixel = image::Rgb([
                pixel_color.x as u8,
                pixel_color.y as u8,
                pixel_color.z as u8,
            ]);
        }
        progress.inc(1);
    }
    progress.finish();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
