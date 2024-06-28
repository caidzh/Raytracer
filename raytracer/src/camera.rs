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
use crate::rtweekend::INFINITY;
use crate::vec3::Vector;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub center: Vector,
    pub pixel00_loc: Vector,
    pub pixel_delta_u: Vector,
    pub pixel_delta_v: Vector,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 0.0,
            image_width: 0,
            image_height: 0,
            center: Vector::new(0.0, 0.0, 0.0),
            pixel00_loc: Vector::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector::new(0.0, 0.0, 0.0),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &HittableList) {
        let path = std::path::Path::new("output/book1/image5.jpg");
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
                let pixel = img.get_pixel_mut(i, j);
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * (i as f64))
                    + (self.pixel_delta_v * (j as f64));
                let ray_direction = pixel_center - self.center;
                let r: Ray = Ray::new(self.center, ray_direction);
                let pixel_color: Vector = Self::ray_color(&r, &world);
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
    }
    fn initialise(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio).round() as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };
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
    fn ray_color(r: &Ray, world: &HittableList) -> Vector {
        let mut rec: HitRecord = HitRecord::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 0.0),
            0.0,
            false,
        );
        if world.hit(r, &Interval::new(0.0, INFINITY), &mut rec) {
            Vector::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0) * 0.5 * 255.99
        } else {
            let unit_direction: Vector = r.direction.unit();
            let a = 0.5 * (unit_direction.y + 1.0);
            let white: Vector = Vector::new(1.0, 1.0, 1.0);
            let blue: Vector = Vector::new(0.5, 0.7, 1.0);
            (white * (1.0 - a) + blue * a) * 255.99
        }
    }
}
