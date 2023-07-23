use std::{f64::consts::PI, rc::Rc};

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::Scatter,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Rc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Rc<dyn Scatter>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        (phi / (2f64 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().modsq();
        let half_b = dot(&oc, &r.direction());
        let c = oc.modsq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.point = r.at(root);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        (rec.u, rec.v) = Sphere::get_sphere_uv(&outward_normal);
        rec.mat_ptr = Some(self.mat_ptr.clone());

        true
    }

    fn bounding_box(&self, _time: [f64; 2], output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center - Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}

pub struct MovingSphere {
    center: [Point3; 2],
    time: [f64; 2],
    radius: f64,
    mat_ptr: Rc<dyn Scatter>,
}

impl MovingSphere {
    pub fn new(center: [Point3; 2], time: [f64; 2], radius: f64, mat_ptr: Rc<dyn Scatter>) -> Self {
        Self {
            center,
            time,
            radius,
            mat_ptr,
        }
    }
    fn center(&self, time: f64) -> Point3 {
        self.center[0]
            + (self.center[1] - self.center[0])
                * ((time - self.time[0]) / (self.time[1] - self.time[0]))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().modsq();
        let half_b = dot(&oc, &r.direction());
        let c = oc.modsq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.point = r.at(root);
        let outward_normal = (rec.point - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Some(self.mat_ptr.clone());

        true
    }

    fn bounding_box(&self, time: [f64; 2], output_box: &mut Aabb) -> bool {
        let box0 = Aabb::new(
            self.center(time[0]) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time[0]) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(time[1]) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time[1]) + Vec3::new(self.radius, self.radius, self.radius),
        );
        *output_box = Aabb::surrounding_box(&box0, &box1);
        true
    }
}
