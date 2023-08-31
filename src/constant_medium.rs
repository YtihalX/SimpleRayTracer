use std::rc::Rc;

use rand::{random, thread_rng, Rng};

use crate::{
    hittable::{HitRecord, Hittable},
    material::{Isotropic, Scatter},
    texture::Texture,
    vec3::{Color, Vec3},
};

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    phase_function: Rc<dyn Scatter>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn from_texture(boundary: Rc<dyn Hittable>, d: f64, a: Rc<dyn Texture>) -> Self {
        Self {
            boundary,
            phase_function: Rc::new(Isotropic::from_texture(a)),
            neg_inv_density: -1f64 / d,
        }
    }
    pub fn from_color(boundary: Rc<dyn Hittable>, d: f64, c: Color) -> Self {
        Self {
            boundary,
            phase_function: Rc::new(Isotropic::from_color(c)),
            neg_inv_density: -1f64 / d,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        const enable_debug: bool = false;
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();
        if !self
            .boundary
            .hit(r, -f64::INFINITY, f64::INFINITY, &mut rec1)
        {
            return false;
        }
        if !self
            .boundary
            .hit(r, rec1.t + 0.0001, f64::INFINITY, &mut rec2)
        {
            return false;
        }
        rec1.t = rec1.t.max(t_min);
        rec2.t = rec2.t.min(t_max);
        if rec1.t >= rec2.t {
            return false;
        }
        rec1.t = rec1.t.max(0f64);

        let ray_len = r.direction().modulus();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_len;
        let mut rng = thread_rng();
        let hit_distance = self.neg_inv_density * rng.gen_range(0f64..1f64).ln();
        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.t = rec1.t + hit_distance / ray_len;
        rec.point = r.at(rec.t);

        rec.normal = Vec3::new(1f64, 0f64, 0f64);
        rec.front_face = true;
        rec.mat_ptr = Some(self.phase_function.clone());
        true
    }

    fn bounding_box(&self, time: [f64; 2], output_box: &mut crate::aabb::Aabb) -> bool {
        self.boundary.bounding_box(time, output_box)
    }
}
