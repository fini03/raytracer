use std::sync::Arc;
use super::Ray;
use crate::{
    math::{Vec3, Point3},
    surface::Material,
    kdtree::AABB,
};

pub trait Hittable: Send + Sync {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord>;

    fn shadow_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool;

    fn bound(&self) -> AABB;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub tex_coords: Vec3,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn from_values(
        r: &Ray,
        p: Point3,
        outward_normal: &Vec3,
        t: f32,
        tex_coords: Vec3,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = r.dir.dot(outward_normal) < 0.;
        Self {
            p,
            normal: outward_normal.clone(),
            t,
            front_face,
            tex_coords,
            material,
        }
    }
}
