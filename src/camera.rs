use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::{
    ray::Ray,
    vec3::{cross, random_unit_disk, Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f64,
    time: [f64; 2],
    rng: ThreadRng,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookfor: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time: [f64; 2],
    ) -> Self {
        let theta = vfov / 180f64 * std::f64::consts::PI;
        let h = (theta / 2f64).tan();
        let viewport_height = 2f64 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (lookfrom - lookfor).unit();
        let u = cross(&vup, &w).unit();
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        let lens_radius = aperture / 2f64;
        let rng = thread_rng();

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            _w: w,
            lens_radius,
            time,
            rng,
        }
    }

    pub fn get_ray(&mut self, u: f64, v: f64) -> Ray {
        let rd = random_unit_disk(&mut self.rng) * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
            self.rng.gen_range(self.time[0]..self.time[1]),
        )
    }
}
