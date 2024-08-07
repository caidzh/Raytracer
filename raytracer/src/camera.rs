use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::f64;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::canny::{Canny, Matrix};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::ScatterRecord;
// use crate::pdf::CosinePdf;
use crate::pdf::{HittablePdf, MixturePdf, Pdf};
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::rtweekend::INFINITY;
use crate::rtweekend::{self, degrees_to_radians};
// use crate::rtweekend::PI;
// use crate::rtweekend::random_double_range;
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
    pub sqrt_spp: i32,
    pub recip_sqrt_spp: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 1920,
            image_height: 0,
            samples_per_pixel: 2500,
            pixel_samples_scale: 0.0,
            center: Vector::new(0.0, 0.0, 0.0),
            pixel00_loc: Vector::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector::new(0.0, 0.0, 0.0),
            max_depth: 50,
            vfov: 40.0,
            lookfrom: Vector::new(800.0, 450.0, -800.0),
            lookat: Vector::new(800.0, 450.0, 0.0),
            vup: Vector::new(0.0, 1.0, 0.0),
            u: Vector::new(0.0, 0.0, 0.0),
            v: Vector::new(0.0, 0.0, 0.0),
            w: Vector::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Vector::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vector::new(0.0, 0.0, 0.0),
            background: Vector::new(0.0, 0.0, 0.0),
            sqrt_spp: 0,
            recip_sqrt_spp: 0.0,
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: HittableList, lights: Arc<dyn Hittable>) {
        let path = std::path::Path::new("output/test/image5.jpg");
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
            let lights = lights.clone();
            let copy = self.clone();
            let rend_line = thread::spawn(move || {
                for i in 0..copy.image_width {
                    let mut pixel_color: Vector = Vector::new(0.0, 0.0, 0.0);
                    // for _ in 0..copy.samples_per_pixel {
                    //     let r: Ray = copy.get_ray(i, j);
                    //     pixel_color = pixel_color + copy.ray_color(&r, copy.max_depth, &world);
                    // }
                    for s_j in 0..copy.sqrt_spp {
                        for s_i in 0..copy.sqrt_spp {
                            let r = copy.get_ray(i, j, s_i, s_j);
                            pixel_color = pixel_color
                                + copy.ray_color(&r, copy.max_depth, &world, lights.clone())
                        }
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
        // let img = Self::edge_detection(&mut img, 100, 150, 1);
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
        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as i32;
        self.pixel_samples_scale = 1.0 / ((self.sqrt_spp * self.sqrt_spp) as f64);
        self.recip_sqrt_spp = 1.0 / (self.sqrt_spp as f64);
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
    // fn sample_square() -> Vector {
    //     Vector::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    // }
    fn get_ray(&self, i: u32, j: u32, s_i: i32, s_j: i32) -> Ray {
        let offset: Vector = self.sample_square_stratified(s_i, s_j);
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
    fn sample_square_stratified(&self, s_i: i32, s_j: i32) -> Vector {
        let px = ((s_i as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        Vector::new(px, py, 0.0)
    }
    fn defocus_disk_sample(&self) -> Vector {
        let p = Vector::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
    fn ray_color(
        &self,
        r: &Ray,
        depth: u32,
        world: &HittableList,
        lights: Arc<dyn Hittable>,
    ) -> Vector {
        if depth == 0 {
            return Vector::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(r, &Interval::new(0.001, INFINITY)) {
            // let direction: Vector = rec.normal + Vector::random_unit_vector();
            // Self::ray_color(&Ray::new(rec.p, direction), depth - 1, world) * 0.5
            let mat = rec.mat.as_ref().unwrap();
            let color_from_emission = mat.emitted(r, rec.clone(), rec.u, rec.v, rec.p);
            let mut srec: ScatterRecord = Default::default();
            if mat.scatter(r, &rec, &mut srec) {
                // let scattering_pdf = mat.scattering_pdf(r, rec.clone(), &mut scattered);
                // let pdf = scattering_pdf;
                // let col = self.ray_color(&scattered, depth - 1, world);
                // return Vector::new(
                //     attenuation.x * col.x * scattering_pdf / pdf,
                //     attenuation.y * col.y * scattering_pdf / pdf,
                //     attenuation.z * col.z * scattering_pdf / pdf,
                // ) + color_from_emission;

                // let on_light = Vector::new(
                //     random_double_range(213.0, 343.0),
                //     554.0,
                //     random_double_range(227.0, 332.0),
                // );
                // let mut to_light = on_light - rec.p;
                // let distance_squared = to_light.length_square();
                // to_light = to_light.unit();
                // if to_light.dot(&rec.normal) < 0.0 {
                //     return color_from_emission;
                // }
                // let light_area = (343.0 - 213.0) * (332.0 - 227.0);
                // let light_cosine = to_light.y.abs();
                // if light_cosine < 0.000001 {
                //     return color_from_emission;
                // }
                // pdf = distance_squared / (light_cosine * light_area);
                // scattered = Ray::new(rec.p, to_light, r.time);
                // let scattering_pdf = mat.scattering_pdf(r, rec.clone(), &mut scattered);
                // let col = self.ray_color(&scattered, depth - 1, world);
                // return Vector::new(
                //     attenuation.x * col.x * scattering_pdf / pdf,
                //     attenuation.y * col.y * scattering_pdf / pdf,
                //     attenuation.z * col.z * scattering_pdf / pdf,
                // ) + color_from_emission;

                // let mut surface_pdf: CosinePdf = Default::default();
                // surface_pdf.init(rec.normal);
                // scattered = Ray::new(rec.p, surface_pdf.generate(), r.time);
                // pdf = surface_pdf.value(scattered.direction);
                // let scattering_pdf = mat.scattering_pdf(r, rec.clone(), &mut scattered);
                // let col = self.ray_color(&scattered, depth - 1, world);
                // return Vector::new(
                //     attenuation.x * col.x * scattering_pdf / pdf,
                //     attenuation.y * col.y * scattering_pdf / pdf,
                //     attenuation.z * col.z * scattering_pdf / pdf,
                // ) + color_from_emission;
                if srec.skip_pdf {
                    return srec.attenuation
                        * self.ray_color(&srec.skip_pdf_ray, depth - 1, world, lights);
                }
                let light_ptr = Arc::new(HittablePdf::new(lights.clone(), rec.p));
                let p = MixturePdf::new(light_ptr, srec.pdf_ptr.unwrap());
                let mut scattered = Ray::new(rec.p, p.generate(), r.time);
                let pdf = p.value(scattered.direction);
                let scattering_pdf = mat.scattering_pdf(r, rec.clone(), &mut scattered);
                let sample_color = self.ray_color(&scattered, depth - 1, world, lights);
                return Vector::new(
                    srec.attenuation.x * sample_color.x * scattering_pdf / pdf,
                    srec.attenuation.y * sample_color.y * scattering_pdf / pdf,
                    srec.attenuation.z * sample_color.z * scattering_pdf / pdf,
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
        if pixel_color.x.is_nan() {
            pixel_color.x = 0.0;
        }
        if pixel_color.y.is_nan() {
            pixel_color.y = 0.0;
        }
        if pixel_color.z.is_nan() {
            pixel_color.z = 0.0;
        }
        pixel_color.x = Self::linear_to_gamma(pixel_color.x);
        pixel_color.y = Self::linear_to_gamma(pixel_color.y);
        pixel_color.z = Self::linear_to_gamma(pixel_color.z);
        let rbyte: u8 = (intensity.clamp(pixel_color.x) * 255.99).round() as u8;
        let gbyte: u8 = (intensity.clamp(pixel_color.y) * 255.99).round() as u8;
        let bbyte: u8 = (intensity.clamp(pixel_color.z) * 255.99).round() as u8;
        *pixel = image::Rgb([rbyte, gbyte, bbyte]);
    }
    pub fn edge_detection(img: &mut RgbImage, l: u8, r: u8, connectnum: u8) -> RgbImage {
        let canny = Canny::new();
        let mut gradients: Matrix =
            Matrix::new(img.height() as usize - 2, img.width() as usize - 2);
        let mut direction: Matrix =
            Matrix::new(img.height() as usize - 2, img.width() as usize - 2);
        for i in 0..img.height() - 2 {
            for j in 0..img.width() - 2 {
                let mut dx: f64 = 0.0;
                let mut dy: f64 = 0.0;
                for k1 in 0..=2 {
                    for k2 in 0..=2 {
                        let pixel = img.get_pixel(j + k1, i + k2);
                        let gray = rtweekend::rgb_to_gray(pixel);
                        dx += gray * canny.sobel.gx.data[k1 as usize][k2 as usize];
                        dy += gray * canny.sobel.gy.data[k1 as usize][k2 as usize];
                    }
                }
                gradients.data[i as usize][j as usize] = (dx * dx + dy * dy).sqrt();
                if dx == 0.0 {
                    direction.data[i as usize][j as usize] = rtweekend::PI / 2.0;
                } else {
                    direction.data[i as usize][j as usize] = (dy / dx).atan();
                }
            }
        }
        let mut nms: Matrix = Matrix::new(img.height() as usize - 4, img.width() as usize - 4);
        for i in 0..img.height() - 4 {
            for j in 0..img.width() - 4 {
                nms.data[i as usize][j as usize] = gradients.data[i as usize + 1][j as usize + 1];
            }
        }
        for i in 1..img.height() - 3 {
            for j in 1..img.width() - 3 {
                let theta = direction.data[i as usize][j as usize];
                let mut weight = theta.tan();
                let mut d: Matrix = Matrix::new(2, 2);
                if theta > rtweekend::PI / 4.0 {
                    d.data[0][0] = 0.0;
                    d.data[0][1] = 1.0;
                    d.data[1][0] = 1.0;
                    d.data[1][1] = 1.0;
                    weight = 1.0 / weight;
                }
                if theta > 0.0 && theta <= rtweekend::PI / 4.0 {
                    d.data[0][0] = 1.0;
                    d.data[0][1] = 0.0;
                    d.data[1][0] = 1.0;
                    d.data[1][1] = 1.0;
                }
                if theta > -rtweekend::PI / 4.0 && theta <= 0.0 {
                    d.data[0][0] = 1.0;
                    d.data[0][1] = 0.0;
                    d.data[1][0] = 1.0;
                    d.data[1][1] = -1.0;
                    weight *= -1.0;
                }
                if theta <= -rtweekend::PI / 4.0 {
                    d.data[0][0] = 0.0;
                    d.data[0][1] = -1.0;
                    d.data[1][0] = 1.0;
                    d.data[1][1] = -1.0;
                    weight = -1.0 / weight;
                }
                let g1 = gradients.data[i as usize + (d.data[0][0] as usize)]
                    [j as usize + (d.data[0][1] as usize)];
                let g2 = gradients.data[i as usize + (d.data[1][0] as usize)]
                    [j as usize + (d.data[1][1] as usize)];
                let g3 = gradients.data[i as usize - (d.data[0][0] as usize)]
                    [j as usize - (d.data[0][1] as usize)];
                let g4 = gradients.data[i as usize - (d.data[1][0] as usize)]
                    [j as usize - (d.data[1][1] as usize)];
                let grade_count1 = g1 * weight + g2 * (1.0 - weight);
                let grade_count2 = g3 * weight + g4 * (1.0 - weight);
                if grade_count1 > gradients.data[i as usize][j as usize]
                    || grade_count2 > gradients.data[i as usize][j as usize]
                {
                    nms.data[i as usize - 1][j as usize - 1] = 0.0;
                }
            }
        }
        let mut final_image: RgbImage = ImageBuffer::new(img.width() - 4, img.height() - 4);
        for i in 1..img.height() - 4 {
            for j in 1..img.width() - 4 {
                let data = nms.data[i as usize][j as usize].floor() as u8;
                if data >= r {
                    let pixel = final_image.get_pixel_mut(j, i);
                    *pixel = image::Rgb([0, 0, 0]);
                } else if data <= l {
                    let pixel = final_image.get_pixel_mut(j, i);
                    let pixel1 = img.get_pixel(j + 1, i + 1);
                    *pixel = image::Rgb([pixel1[0], pixel1[1], pixel1[2]]);
                } else {
                    let mut count = 0;
                    if nms.data[i as usize - 1][j as usize - 1] == 0.0 {
                        count += 1;
                    }
                    if nms.data[i as usize - 1][j as usize] == 0.0 {
                        count += 1;
                    }
                    if nms.data[i as usize - 1][j as usize + 1] == 0.0 {
                        count += 1;
                    }
                    if nms.data[i as usize][j as usize - 1] == 0.0 {
                        count += 1;
                    }
                    if nms.data[i as usize][j as usize + 1] == 0.0 {
                        count += 1;
                    }
                    if nms.data[i as usize + 1][j as usize - 1] == 0.0 {
                        count += 1;
                    }
                    if nms.data[i as usize + 1][j as usize] == 0.0 {
                        count += 1;
                    }
                    if nms.data[i as usize + 1][j as usize + 1] == 0.0 {
                        count += 1;
                    }
                    if count >= connectnum {
                        let pixel = final_image.get_pixel_mut(j, i);
                        *pixel = image::Rgb([0, 0, 0]);
                    }
                }
            }
        }
        final_image
    }
    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }
}
