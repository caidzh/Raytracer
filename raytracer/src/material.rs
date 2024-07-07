use crate::hittable::HitRecord;
use crate::pdf::CosinePdf;
use crate::pdf::Pdf;
use crate::pdf::SpherePdf;
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::rtweekend::PI;
use crate::texture::{SolidColor, Texture};
use crate::vec3::Vector;
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn emitted(&self, _r_in: &Ray, _rec: HitRecord, _u: f64, _v: f64, _p: Vector) -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: HitRecord, _scattered: &mut Ray) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: Vector) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(a)),
        }
    }
    pub fn arc_new(a: Arc<dyn Texture>) -> Self {
        Self { tex: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        // let mut scatter_direction: Vector = Vector::random_on_hemisphere(&rec.normal);
        // let mut scatter_direction: Vector = rec.normal + Vector::random_unit_vector();

        // if scatter_direction.near_zero() {
        //     scatter_direction = rec.normal
        // }
        // *scattered = Ray::new(rec.p, scatter_direction, r_in.time);
        // *attenuation = self.tex.value(rec.u, rec.v, rec.p);

        // let mut uvw: Onb = Default::default();
        // uvw.build_from_w(rec.normal);
        // let scatter_direction = uvw.vec_local(Vector::random_cosine_direction());
        // *scattered = Ray::new(rec.p, scatter_direction.unit(), r_in.time);
        // *attenuation = self.tex.value(rec.u, rec.v, rec.p);
        // *pdf = uvw.axis[2].dot(&scattered.direction) / PI;

        srec.attenuation = self.tex.value(rec.u, rec.v, rec.p);
        srec.pdf_ptr = Some(Arc::new(CosinePdf::new(rec.normal)));
        srec.skip_pdf = false;
        true
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: HitRecord, _scattered: &mut Ray) -> f64 {
        // let cos_theta = rec.normal.dot(&scattered.direction.unit());
        // if cos_theta < 0.0 {
        //     0.0
        // } else {
        //     cos_theta / PI
        // }
        1.0 / (4.0 * PI)
    }
}

pub struct Metal {
    albedo: Vector,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Vector, b: f64) -> Self {
        Self {
            albedo: a,
            fuzz: b.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let mut reflected: Vector = Vector::reflect(&r_in.direction.unit(), &rec.normal);
        reflected = reflected.unit() + (Vector::random_unit_vector() * self.fuzz);
        srec.attenuation = self.albedo;
        srec.pdf_ptr = None;
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::new(rec.p, reflected, r_in.time);
        true
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(a: f64) -> Self {
        Self {
            refraction_index: a,
        }
    }
    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = Vector::new(1.0, 1.0, 1.0);
        srec.pdf_ptr = None;
        srec.skip_pdf = true;
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.direction.unit();
        let cos_theta = rec.normal.dot(&(unit_direction * -1.0)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            Vector::reflect(&unit_direction, &rec.normal)
        } else {
            Vector::refract(&unit_direction, &rec.normal, ri)
        };
        srec.skip_pdf_ray = Ray::new(rec.p, direction, r_in.time);
        true
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(t: Arc<dyn Texture>) -> Self {
        Self { tex: t }
    }
    pub fn color_new(emit: Vector) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _r_in: &Ray, rec: HitRecord, u: f64, v: f64, p: Vector) -> Vector {
        if !rec.front_face {
            return Vector::new(0.0, 0.0, 0.0);
        }
        self.tex.value(u, v, p)
    }
}

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn color_new(a: Vector) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(a)),
        }
    }
    pub fn new(t: Arc<dyn Texture>) -> Self {
        Self { tex: t }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        // *scattered = Ray::new(rec.p, Vector::random_unit_vector(), r_in.time);
        srec.attenuation = self.tex.value(rec.u, rec.v, rec.p);
        srec.pdf_ptr = Some(Arc::new(SpherePdf {}));
        srec.skip_pdf = false;
        true
    }
}

#[derive(Default)]
pub struct ScatterRecord {
    pub attenuation: Vector,
    pub pdf_ptr: Option<Arc<dyn Pdf>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}
