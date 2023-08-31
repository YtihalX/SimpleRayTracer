use std::rc::Rc;

use crate::{
    aabb::Aabb,
    aarect::{XYRect, YZRect, ZXRect},
    hittable::Hittable,
    hittable_list::HittableList,
    material::Scatter,
    vec3::Point3,
};

pub struct Cube {
    cube_min: Point3,
    cube_max: Point3,
    sides: HittableList,
}

impl Cube {
    pub fn new(p0: Point3, p1: Point3, ptr: Rc<dyn Scatter>) -> Self {
        let cube_min = p0;
        let cube_max = p1;
        let mut sides = HittableList::new();
        sides.push(Rc::new(XYRect::new(
            ptr.clone(),
            [p0.x(), p1.x()],
            [p0.y(), p1.y()],
            p1.z(),
        )));
        sides.push(Rc::new(XYRect::new(
            ptr.clone(),
            [p0.x(), p1.x()],
            [p0.y(), p1.y()],
            p0.z(),
        )));

        sides.push(Rc::new(ZXRect::new(
            ptr.clone(),
            [p0.x(), p1.x()],
            [p0.z(), p1.z()],
            p1.y(),
        )));
        sides.push(Rc::new(ZXRect::new(
            ptr.clone(),
            [p0.x(), p1.x()],
            [p0.z(), p1.z()],
            p0.y(),
        )));

        sides.push(Rc::new(YZRect::new(
            ptr.clone(),
            [p0.y(), p1.y()],
            [p0.z(), p1.z()],
            p1.x(),
        )));
        sides.push(Rc::new(YZRect::new(
            ptr.clone(),
            [p0.y(), p1.y()],
            [p0.z(), p1.z()],
            p0.x(),
        )));
        Self {
            cube_min,
            cube_max,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, _time: [f64; 2], output_box: &mut crate::aabb::Aabb) -> bool {
        *output_box = Aabb::new(self.cube_min, self.cube_max);
        true
    }
}
