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
}

impl Metal {
    pub fn new(a: Vector) -> Self {
        Self { albedo: a }
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
        let reflected: Vector = Vector::reflect(&r_in.direction.unit(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}
