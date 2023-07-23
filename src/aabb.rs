use crate::{ray::Ray, vec3::Point3};

#[derive(Clone)]
pub struct Aabb {
    pub max: Point3,
    pub min: Point3,
}

impl Aabb {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self { max: b, min: a }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let t0 = ((self.min.e[i] - r.origin().e[i]) / r.direction().e[i])
                .min((self.max.e[i] - r.origin().e[i]) / r.direction().e[i]);
            let t1 = ((self.min.e[i] - r.origin().e[i]) / r.direction().e[i])
                .max((self.max.e[i] - r.origin().e[i]) / r.direction().e[i]);
            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let small = Point3::new(
            box0.min.x().min(box1.min.x()),
            box0.min.y().min(box1.min.y()),
            box0.min.z().min(box1.min.z()),
        );
        let big = Point3::new(
            box0.max.x().max(box1.max.x()),
            box0.max.y().max(box1.max.y()),
            box0.max.z().max(box1.max.z()),
        );

        Aabb::new(small, big)
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Aabb {
            max: Point3::default(),
            min: Point3::default(),
        }
    }
}
