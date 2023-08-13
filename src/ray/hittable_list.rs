use std::default::Default;
use super::{Hittable, HitRecord};
use crate::kdtree::AABB;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn extend(&mut self, mut objects: Vec<Box<dyn Hittable>>) {
        self.objects.append(&mut objects)
    }
}

impl Default for HittableList {
    fn default() -> Self {
        HittableList::new()
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &super::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord> {
        hit_scan(
            self.objects.iter().map(|h| h.as_ref()),
            r,
            t_min,
            t_max,
        )
    }

    fn shadow_hit(
        &self,
        r: &super::Ray,
        t_min: f32,
        t_max: f32,
    ) -> bool {
        shadow_hit_scan(
            self.objects.iter().map(|h| h.as_ref()),
            r,
            t_min,
            t_max,
        )
    }

    fn bound(&self) -> AABB {
        let mut aabb = AABB::empty();
        for object in &self.objects {
            aabb.merge(&object.bound())
        }
        aabb
    }
}

pub fn hit_scan<'a, I>(
    hittables: I,
    r: &super::Ray,
    t_min: f32,
    t_max: f32,
) -> Option<HitRecord>
where
    I: Iterator<Item = &'a dyn Hittable>,
{
    let mut tmp_record = None;
    let mut closest = t_max;

    for object in hittables {
        let rec = object.hit(r, t_min, closest);
        if let Some(ref hit) = rec {
            closest = hit.t;
            tmp_record = rec;
        }
    }

    tmp_record
}

pub fn shadow_hit_scan<'a, I>(
    hittables: I,
    r: &super::Ray,
    t_min: f32,
    t_max: f32,
) -> bool
where
    I: Iterator<Item = &'a dyn Hittable>,
{
    for object in hittables {
        if object.shadow_hit(r, t_min, t_max) {
            return true;
        }
    }

    false
}
