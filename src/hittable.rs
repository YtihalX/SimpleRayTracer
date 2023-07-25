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

pub struct Translate {
    offset: Vec3,
    ptr: Rc<dyn Hittable>,
}

impl Translate {
    pub fn new(offset: Vec3, ptr: Rc<dyn Hittable>) -> Self {
        Self { offset, ptr }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }
        rec.point += self.offset;
        let normal = rec.normal;
        rec.set_face_normal(&moved_r, &normal);
        true
    }
    fn bounding_box(&self, time: [f64; 2], output_box: &mut Aabb) -> bool {
        if !self.ptr.bounding_box(time, output_box) {
            return false;
        }
        *output_box = Aabb::new(output_box.min + self.offset, output_box.max + self.offset);
        true
    }
}

pub struct RotateY {
    sin: f64,
    cos: f64,
    valid: bool,
    bbox: Aabb,
    ptr: Rc<dyn Hittable>,
}

impl RotateY {
    pub fn new(angle: f64, ptr: Rc<dyn Hittable>) -> Self {
        let angle = angle / 180f64 * std::f64::consts::PI;
        let sin = angle.sin();
        let cos = angle.cos();
        let mut bbox = Aabb::default();
        let valid = ptr.bounding_box([0f64, 1f64], &mut bbox);
        let mut min = Point3::new(std::f64::INFINITY, std::f64::INFINITY, std::f64::INFINITY);
        let mut max = Point3::new(
            -std::f64::INFINITY,
            -std::f64::INFINITY,
            -std::f64::INFINITY,
        );

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max.x() + (1 - i) as f64 * bbox.min.x();
                    let y = j as f64 * bbox.max.y() + (1 - j) as f64 * bbox.min.y();
                    let z = k as f64 * bbox.max.z() + (1 - k) as f64 * bbox.min.z();

                    let new_x = cos * x + sin * z;
                    let new_z = -sin * x + cos * z;

                    let tester = Vec3::new(new_x, y, new_z);
                    for c in 0..3 {
                        min.e[c] = min.e[c].min(tester.e[c]);
                        max.e[c] = max.e[c].max(tester.e[c]);
                    }
                }
            }
        }
        bbox = Aabb::new(min, max);
        Self {
            sin,
            cos,
            valid,
            bbox,
            ptr,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();
        origin.e[0] = self.cos * origin.e[0] - self.sin * origin.e[2];
        origin.e[2] = self.sin * r.origin().e[0] + self.cos * origin.e[2];

        direction.e[0] = self.cos * direction.e[0] - self.sin * direction.e[2];
        direction.e[2] = self.sin * r.direction().e[0] + self.cos * direction.e[2];

        let rotated_r = Ray::new(origin, direction, r.time());
        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }
        let mut point = rec.point;
        let mut normal = rec.normal;

        point.e[0] = self.cos * rec.point.e[0] + self.sin * rec.point.e[2];
        point.e[2] = -self.sin * rec.point.e[0] + self.cos * rec.point.e[2];

        normal.e[0] = self.cos * rec.normal.e[0] + self.sin * rec.normal.e[2];
        normal.e[2] = -self.sin * rec.normal.e[0] + self.cos * rec.normal.e[2];

        rec.point = point;
        rec.set_face_normal(&rotated_r, &normal);
        true
    }

    fn bounding_box(&self, _time: [f64; 2], output_box: &mut Aabb) -> bool {
        *output_box = self.bbox.clone();
        self.valid
    }
}
