use std::rc::Rc;

use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

use crate::aabb::Aabb;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    abox: Aabb,
}

impl BvhNode {
    pub fn from_objects(
        src_objects: &mut Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time: [f64; 2],
        rng: &mut ThreadRng,
    ) -> Self {
        let axis = rng.gen_range(0..3);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };
        let object_span = end - start;
        let left;
        let right;
        let abox;
        if object_span == 1 {
            left = src_objects[start].clone();
            right = left.clone();
        } else if object_span == 2 {
            src_objects[start..end].sort_by(comparator);
            left = src_objects[start].clone();
            right = src_objects[start + 1].clone();
        } else {
            src_objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            left = Rc::new(BvhNode::from_objects(src_objects, start, mid, time, rng));
            right = Rc::new(BvhNode::from_objects(src_objects, mid, end, time, rng));
        }
        let mut box_left = Aabb::default();
        let mut box_right = Aabb::default();
        if !left.bounding_box(time, &mut box_left) || !right.bounding_box(time, &mut box_right) {
            eprintln!("No bounding box!");
        }
        abox = Aabb::surrounding_box(&box_left, &box_right);
        Self { left, right, abox }
    }
    pub fn from_list(list: &HittableList, time: [f64; 2]) -> Self {
        let mut objects = list.objects.clone();
        let mut rng = thread_rng();
        BvhNode::from_objects(&mut objects, 0, list.objects.len(), time, &mut rng)
    }
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        if !self.abox.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);
        hit_left || hit_right
    }

    fn bounding_box(&self, _time: [f64; 2], output_box: &mut Aabb) -> bool {
        *output_box = self.abox.clone();
        true
    }
}

pub fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> bool {
    let mut box_a = Aabb::default();
    let mut box_b = Aabb::default();
    if !a.bounding_box([0f64, 0f64], &mut box_a) || !b.bounding_box([0f64, 0f64], &mut box_b) {
        eprintln!("No bounding box");
    }
    box_a.min.e[axis] < box_b.min.e[axis]
}
pub fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    match box_compare(a, b, 0) {
        true => Ordering::Less,
        false => Ordering::Greater,
    }
}
pub fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    match box_compare(a, b, 1) {
        true => Ordering::Less,
        false => Ordering::Greater,
    }
}
pub fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    match box_compare(a, b, 2) {
        true => Ordering::Less,
        false => Ordering::Greater,
    }
}
