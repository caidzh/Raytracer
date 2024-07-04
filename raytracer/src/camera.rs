use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::f64;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::degrees_to_radians;
use crate::rtweekend::random_double;
use crate::rtweekend::INFINITY;
use crate::vec3::Vector;

#[derive(Clone)]
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
    pub vfov: f64,
    pub lookfrom: Vector,
    pub lookat: Vector,
    pub vup: Vector,
    pub u: Vector,
    pub v: Vector,
    pub w: Vector,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub defocus_disk_u: Vector,
    pub defocus_disk_v: Vector,
    pub background: Vector,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 800,
            image_height: 0,
            samples_per_pixel: 5000,
            pixel_samples_scale: 0.0,
            center: Vector::new(0.0, 0.0, 0.0),
            pixel00_loc: Vector::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector::new(0.0, 0.0, 0.0),
            max_depth: 40,
            vfov: 40.0,
            lookfrom: Vector::new(478.0, 278.0, -600.0),
            lookat: Vector::new(278.0, 278.0, 0.0),
            vup: Vector::new(0.0, 1.0, 0.0),
            u: Vector::new(0.0, 0.0, 0.0),
            v: Vector::new(0.0, 0.0, 0.0),
            w: Vector::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Vector::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vector::new(0.0, 0.0, 0.0),
            background: Vector::new(0.0, 0.0, 0.0),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: HittableList) {
        let path = std::path::Path::new("output/book2/image23.jpg");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
        self.initialise();
        let quality = 100;
        let img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };
        let img = Arc::new(Mutex::new(img));
        let progress = Arc::new(Mutex::new(progress));
        let mut rend_lines = vec![];
        for j in (0..self.image_height).rev() {
            let img = Arc::clone(&img);
            let progress = Arc::clone(&progress);
            let world = world.clone();
            let copy = self.clone();
            let rend_line = thread::spawn(move || {
                for i in 0..copy.image_width {
                    let mut pixel_color: Vector = Vector::new(0.0, 0.0, 0.0);
                    for _ in 0..copy.samples_per_pixel {
                        let r: Ray = copy.get_ray(i, j);
                        pixel_color = pixel_color + copy.ray_color(&r, copy.max_depth, &world);
                    }
                    pixel_color = pixel_color * copy.pixel_samples_scale;
                    let mut img = img.lock().unwrap();
                    Self::write_color(&mut img, i, j, &mut pixel_color);
                    drop(img);

                    let progress = progress.lock().unwrap();
                    progress.inc(1);
                    // Self::write_color(&mut img, i, j, &mut pixel_color);
                }
            });
            rend_lines.push(rend_line);
        }
        for rend_line in rend_lines {
            rend_line.join().unwrap();
        }
        progress.lock().unwrap().finish();
        let img = Some(Arc::try_unwrap(img).unwrap().into_inner().unwrap());
        let img = img.as_ref().unwrap().clone();
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
        self.center = self.lookfrom;
        // let focal_length: f64 = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h * self.focus_dist;
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64 / (self.image_height as f64));
        self.w = (self.lookfrom - self.lookat).unit();
        self.u = self.vup.cross(&self.w).unit();
        self.v = self.w.cross(&self.u);
        let viewport_u: Vector = self.u * viewport_width;
        let viewport_v: Vector = self.v * (-viewport_height);
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);
        let viewport_upper_left =
            self.center - self.w * self.focus_dist - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
    fn sample_square() -> Vector {
        Vector::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset: Vector = Self::sample_square();
        let pixel_sample: Vector = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset.x)
            + self.pixel_delta_v * (j as f64 + offset.y);
        let ray_origin: Vector = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();
        Ray::new(ray_origin, ray_direction, ray_time)
    }
    fn defocus_disk_sample(&self) -> Vector {
        let p = Vector::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
    fn ray_color(&self, r: &Ray, depth: u32, world: &HittableList) -> Vector {
        if depth == 0 {
            return Vector::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(r, &Interval::new(0.001, INFINITY)) {
            // let direction: Vector = rec.normal + Vector::random_unit_vector();
            // Self::ray_color(&Ray::new(rec.p, direction), depth - 1, world) * 0.5
            let mut scattered: Ray =
                Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0);
            let mut attenuation: Vector = Vector::new(0.0, 0.0, 0.0);
            let mat = rec.mat.as_ref().unwrap();
            let color_from_emission = mat.emitted(rec.u, rec.v, rec.p);
            if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                let col = self.ray_color(&scattered, depth - 1, world);
                return Vector::new(
                    attenuation.x * col.x,
                    attenuation.y * col.y,
                    attenuation.z * col.z,
                ) + color_from_emission;
            }
            color_from_emission
        } else {
            // let unit_direction: Vector = r.direction.unit();
            // let a = 0.5 * (unit_direction.y + 1.0);
            // let white: Vector = Vector::new(1.0, 1.0, 1.0);
            // let blue: Vector = Vector::new(0.5, 0.7, 1.0);
            // white * (1.0 - a) + blue * a
            self.background
        }
    }
    fn write_color(img: &mut RgbImage, i: u32, j: u32, pixel_color: &mut Vector) {
        let pixel = img.get_pixel_mut(i, j);
        let intensity: Interval = Interval::new(0.000, 0.999);
        pixel_color.x = Self::linear_to_gamma(pixel_color.x);
        pixel_color.y = Self::linear_to_gamma(pixel_color.y);
        pixel_color.z = Self::linear_to_gamma(pixel_color.z);
        let rbyte: u8 = (intensity.clamp(pixel_color.x) * 255.99).round() as u8;
        let gbyte: u8 = (intensity.clamp(pixel_color.y) * 255.99).round() as u8;
        let bbyte: u8 = (intensity.clamp(pixel_color.z) * 255.99).round() as u8;
        *pixel = image::Rgb([rbyte, gbyte, bbyte]);
    }
    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }
}
