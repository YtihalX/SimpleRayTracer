use std::rc::Rc;

use crate::aabb::Aabb;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_ptr: Option<Rc<dyn Scatter>>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            u: 0f64,
            v: 0f64,
            front_face: false,
            mat_ptr: None,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time: [f64; 2], output_box: &mut Aabb) -> bool;
}
