use std::rc::Rc;

use crate::{
    aabb::Aabb,
    hittable::Hittable,
    material::Scatter,
    vec3::{Point3, Vec3},
};

pub struct XYRect {
    mp: Rc<dyn Scatter>,
    x: [f64; 2],
    y: [f64; 2],
    k: f64,
}

impl XYRect {
    pub fn new(mp: Rc<dyn Scatter>, x: [f64; 2], y: [f64; 2], k: f64) -> Self {
        Self { mp, x, y, k }
    }
}

impl Hittable for XYRect {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x[0] || x > self.x[1] || y < self.y[0] || y > self.y[1] {
            return false;
        }
        rec.u = (x - self.x[0]) / (self.x[1] - self.x[0]);
        rec.v = (y - self.y[0]) / (self.y[1] - self.y[0]);
        rec.t = t;
        let outward_normal = Vec3::new(0f64, 0f64, 1f64);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.point = r.at(t);
        true
    }
    fn bounding_box(&self, _time: [f64; 2], output_box: &mut crate::aabb::Aabb) -> bool {
        *output_box = Aabb::new(
            Point3::new(self.x[0], self.y[0], self.k - 0.0001),
            Point3::new(self.x[1], self.y[1], self.k + 0.0001),
        );
        true
    }
}
pub struct ZXRect {
    mp: Rc<dyn Scatter>,
    x: [f64; 2],
    z: [f64; 2],
    k: f64,
}

impl ZXRect {
    pub fn new(mp: Rc<dyn Scatter>, x: [f64; 2], z: [f64; 2], k: f64) -> Self {
        Self { mp, x, z, k }
    }
}

impl Hittable for ZXRect {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x[0] || x > self.x[1] || z < self.z[0] || z > self.z[1] {
            return false;
        }
        rec.u = (x - self.x[0]) / (self.x[1] - self.x[0]);
        rec.v = (z - self.z[0]) / (self.z[1] - self.z[0]);
        rec.t = t;
        let outward_normal = Vec3::new(0f64, 1f64, 0f64);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.point = r.at(t);
        true
    }
    fn bounding_box(&self, _time: [f64; 2], output_box: &mut crate::aabb::Aabb) -> bool {
        *output_box = Aabb::new(
            Point3::new(self.x[0], self.k - 0.0001, self.z[0]),
            Point3::new(self.x[1], self.k + 0.0001, self.z[1]),
        );
        true
    }
}
pub struct YZRect {
    mp: Rc<dyn Scatter>,
    y: [f64; 2],
    z: [f64; 2],
    k: f64,
}

impl YZRect {
    pub fn new(mp: Rc<dyn Scatter>, y: [f64; 2], z: [f64; 2], k: f64) -> Self {
        Self { mp, y, z, k }
    }
}

impl Hittable for YZRect {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y[0] || y > self.y[1] || z < self.z[0] || z > self.z[1] {
            return false;
        }
        rec.u = (y - self.y[0]) / (self.y[1] - self.y[0]);
        rec.v = (z - self.z[0]) / (self.z[1] - self.z[0]);
        rec.t = t;
        let outward_normal = Vec3::new(1f64, 0f64, 0f64);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.point = r.at(t);
        true
    }
    fn bounding_box(&self, _time: [f64; 2], output_box: &mut crate::aabb::Aabb) -> bool {
        *output_box = Aabb::new(
            Point3::new(self.k - 0.0001, self.y[0], self.z[0]),
            Point3::new(self.k + 0.0001, self.y[1], self.z[1]),
        );
        true
    }
}
