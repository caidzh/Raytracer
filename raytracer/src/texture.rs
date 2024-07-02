use crate::image::RtwImage;
use crate::interval::Interval;
use crate::vec3::Vector;
use std::sync::Arc;
pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Vector) -> Vector;
}

pub struct SolidColor {
    pub albedo: Vector,
}

impl SolidColor {
    pub fn new(a: Vector) -> Self {
        Self { albedo: a }
    }
    pub fn rgb_new(r: f64, g: f64, b: f64) -> Self {
        Self {
            albedo: Vector::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vector) -> Vector {
        self.albedo
    }
}

pub struct CheckerTexture {
    pub inv_scale: f64,
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(s: f64, e: Arc<dyn Texture>, o: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / s,
            even: e,
            odd: o,
        }
    }
    pub fn color_new(s: f64, c1: Vector, c2: Vector) -> Self {
        Self {
            inv_scale: 1.0 / s,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vector) -> Vector {
        let xinteger = (self.inv_scale * p.x).floor() as i32;
        let yinteger = (self.inv_scale * p.y).floor() as i32;
        let zinteger = (self.inv_scale * p.z).floor() as i32;

        let iseven = (xinteger + yinteger + zinteger) % 2 == 0;
        if iseven {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    pub image: RtwImage,
}

impl ImageTexture {
    pub fn new(image_filename: &str) -> Self {
        Self {
            image: RtwImage::new(image_filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vector) -> Vector {
        if self.image.image_height <= 0 {
            return Vector::new(0.0, 1.0, 1.0);
        }
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);
        let i = (self.image.image_width as f64 * u) as i32;
        let j = (self.image.image_height as f64 * v) as i32;
        let pixel = self.image.pixel_data(i, j);
        let color_scale = 1.0 / 255.0;
        Vector::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64) * color_scale
    }
}
