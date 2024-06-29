use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::f64;
use std::fs::File;

use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::rtweekend::INFINITY;
use crate::vec3::Vector;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub pixel_samples_scale: f64,
    pub center: Vector,
    pub pixel00_loc: Vector,
    pub pixel_delta_u: Vector,
    pub pixel_delta_v: Vector,
    pub max_depth: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            image_height: 0,
            samples_per_pixel: 10,
            pixel_samples_scale: 0.0,
            center: Vector::new(0.0, 0.0, 0.0),
            pixel00_loc: Vector::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector::new(0.0, 0.0, 0.0),
            max_depth: 50,
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &HittableList) {
        let path = std::path::Path::new("output/book1/image8.jpg");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
        self.initialise();
        let quality = 100;
        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };
        for j in (0..self.image_height).rev() {
            for i in 0..self.image_width {
                let mut pixel_color: Vector = Vector::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color = pixel_color + Self::ray_color(&r, self.max_depth, world);
                }
                pixel_color = pixel_color * self.pixel_samples_scale;
                Self::write_color(&mut img, i, j, pixel_color)
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
    }
    fn initialise(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio).round() as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };
        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);
        self.center = Vector::new(0.0, 0.0, 0.0);
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64 / (self.image_height as f64));
        let viewport_u: Vector = Vector::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vector = Vector::new(0.0, -viewport_height, 0.0);
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);
        let viewport_upper_left =
            self.center - Vector::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }
    fn sample_square() -> Vector {
        Vector::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset: Vector = Self::sample_square();
        let pixel_sample: Vector = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset.x)
            + self.pixel_delta_v * (j as f64 + offset.y);
        let ray_origin: Vector = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }
    fn ray_color(r: &Ray, depth: u32, world: &HittableList) -> Vector {
        if depth == 0 {
            return Vector::new(0.0, 0.0, 0.0);
        }
        let mut rec: HitRecord = HitRecord::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 0.0),
            0.0,
            false,
        );
        if world.hit(r, &Interval::new(0.0, INFINITY), &mut rec) {
            let direction: Vector = Vector::random_on_hemisphere(&rec.normal);
            Self::ray_color(&Ray::new(rec.p, direction), depth - 1, world) * 0.5
        } else {
            let unit_direction: Vector = r.direction.unit();
            let a = 0.5 * (unit_direction.y + 1.0);
            let white: Vector = Vector::new(1.0, 1.0, 1.0);
            let blue: Vector = Vector::new(0.5, 0.7, 1.0);
            white * (1.0 - a) + blue * a
        }
    }
    fn write_color(img: &mut RgbImage, i: u32, j: u32, pixel_color: Vector) {
        let pixel = img.get_pixel_mut(i, j);
        let intensity: Interval = Interval::new(0.000, 0.999);
        let rbyte: u8 = (intensity.clamp(pixel_color.x) * 255.99).round() as u8;
        let gbyte: u8 = (intensity.clamp(pixel_color.y) * 255.99).round() as u8;
        let bbyte: u8 = (intensity.clamp(pixel_color.z) * 255.99).round() as u8;
        *pixel = image::Rgb([rbyte, gbyte, bbyte]);
    }
}
