use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vector;
pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vector,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}

pub struct Lambertian {
    albedo: Vector,
}

impl Lambertian {
    pub fn new(a: Vector) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction: Vector = rec.normal + Vector::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected: Vector = Vector::reflect(&r_in.direction.unit(), &rec.normal);
        reflected = reflected.unit() + (Vector::random_unit_vector() * self.fuzz);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        scattered.direction.dot(&rec.normal) > 0.0
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
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vector::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.direction.unit();
        let cos_theta = rec.normal.dot(&(unit_direction * -1.0)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract {
            Vector::reflect(&unit_direction, &rec.normal)
        } else {
            Vector::refract(&unit_direction, &rec.normal, ri)
        };
        *scattered = Ray::new(rec.p, direction);
        true
    }
}
