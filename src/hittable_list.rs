use std::rc::Rc;

use rand::{rngs::ThreadRng, thread_rng};

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    pub rng: ThreadRng,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            rng: thread_rng(),
        }
    }

    pub fn push(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn ray_color(&mut self, r: Ray, background: &Color, depth: usize) -> Color {
        if depth == 0 {
            return Color::default();
        }
        let mut rec = HitRecord::default();
        if !self.hit(&r, 0.001, f64::INFINITY, &mut rec) {
            return *background;
        }
        match rec.mat_ptr.as_ref() {
            Some(p) => {
                let mut attenuation = Vec3::default();
                let mut scattered = Ray::new(Point3::default(), Vec3::default(), 0f64);
                let emitted = p.emitted(rec.u, rec.v, &rec.point);
                if !p.scatter(&r, &rec, &mut self.rng, &mut attenuation, &mut scattered) {
                    return emitted;
                }
                return attenuation * self.ray_color(scattered, background, depth - 1) + emitted;
            }
            None => {
                return Color::default();
            }
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = temp_rec.t;
                rec.u = temp_rec.u;
                rec.v = temp_rec.v;
                rec.front_face = temp_rec.front_face;
                rec.point = temp_rec.point;
                rec.normal = temp_rec.normal;
                rec.mat_ptr = match temp_rec.mat_ptr.as_ref() {
                    Some(p) => Some(p.clone()),
                    None => None,
                }
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time: [f64; 2], output_box: &mut crate::aabb::Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box = Aabb::default();
        let mut first_box = true;
        for object in self.objects.iter() {
            if !object.bounding_box(time, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box.clone()
            } else {
                Aabb::surrounding_box(output_box, &temp_box)
            };
            first_box = false;
        }

        true
    }
}
