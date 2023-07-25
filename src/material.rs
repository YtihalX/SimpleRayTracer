use std::rc::Rc;

use rand::{rngs::ThreadRng, Rng};

use crate::{
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::{dot, random_unit_sphere, reflect, refract, Color, Point3},
};
pub trait Scatter {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
}

pub trait CoScatter: Clone + Scatter {}
impl<T: Clone + Scatter> CoScatter for T {}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn from_color(cl: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::from_color(cl)),
        }
    }
    pub fn from_texture(t: Rc<dyn Texture>) -> Self {
        Self { albedo: t }
    }
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_sphere(rng);
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.point);
        *scattered = Ray::new(rec.point, scatter_direction, r_in.time());
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(cl: Color, f: f64) -> Self {
        Self {
            albedo: cl,
            fuzz: f,
        }
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.direction().unit(), &rec.normal);
        *attenuation = self.albedo;
        *scattered = Ray::new(
            rec.point,
            reflected + random_unit_sphere(rng) * self.fuzz,
            r_in.time(),
        );
        dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

#[derive(Clone)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1f64 - ref_idx) / (1f64 + ref_idx);
        r0 *= r0;
        r0 + (1f64 - r0) * (1f64 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction().unit();
        let cos_theta = 1f64.min(-dot(&unit_direction, &rec.normal));
        let sin_theta = (1f64 - cos_theta * cos_theta).sqrt();

        let nrefract = refraction_ratio * sin_theta > 1f64;
        let direction = if nrefract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0f64..1f64)
        {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };
        *attenuation = Color::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(rec.point, direction, r_in.time());
        true
    }
}

pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_texture(emit: Rc<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn from_color(c: Color) -> Self {
        Self {
            emit: Rc::new(SolidColor::from_color(c)),
        }
    }
}

impl Scatter for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _rng: &mut ThreadRng,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn from_color(c: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::from_color(c)),
        }
    }
    pub fn from_texture(a: Rc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Scatter for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.point, random_unit_sphere(rng), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.point);
        true
    }
}
