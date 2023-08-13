use super::light::*;
use crate::math::{Color, Point3, Vec3};
use crate::ray::{Ray, HitRecord, Hittable};
use rand::Rng;

pub struct Spot {
    pub color: Color,
    pub position: Point3,
    pub direction: Vec3,
    pub alpha1: f32,
    pub alpha2: f32,
    pub alpha_range: f32,
}

impl<M, H, R> LightSource<M, H, R> for Spot
where
    M: LightModel,
    H: Hittable,
    R: Rng,
{
    fn intensity(
        &self,
        r: &Ray,
        hit_record: &HitRecord,
        hittables: &H,
        _rng: &mut R,
    ) -> Color {
        let p = &hit_record.p;
        let l_not_norm = &self.position - p;
        let len = l_not_norm.length();
        let l = &(l_not_norm / len);

        let angle = self.direction.dot(&-l).acos();
        if angle > self.alpha2 {
            return Color::new();
        }

        let s_ray = Ray::from_values(p, &l);
        if hittables.shadow_hit(&s_ray, 0.01, len) {
            return Color::new();
        }

        let v = &-r.dir.unit_vector();
        let n = &hit_record.normal;
        let l_c = &self.color;
        let m_c = &hit_record.material.color(r, hit_record);
        let mut l_p = hit_record.material.phong().clone();
        let ior = hit_record.material.refraction();

        let angle = self.direction.dot(&-l).acos();
        let alpha1 = self.alpha1;
        let alpha_range = self.alpha_range;
        let interpolation = (angle - alpha1) / alpha_range;
        let spotfactor = 1. - interpolation.clamp(0., 1.);
        l_p.kd *= spotfactor;
        l_p.ks *= spotfactor;

        M::intensity(l, v, n, l_c, m_c, &l_p, ior)
    }
}
