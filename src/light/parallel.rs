use super::light::*;
use crate::math::{Color, Vec3};
use crate::ray::{Ray, HitRecord, Hittable};
use rand::Rng;

pub struct Parallel {
    pub color: Color,
    pub direction: Vec3,
}

impl<M, H, R> LightSource<M, H, R> for Parallel
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
        let l = &-&self.direction;
        let s_ray = Ray::from_values(&hit_record.p, l);
        if hittables.shadow_hit(&s_ray, 0.01, f32::INFINITY) {
            return Color::new();
        }

        let v = &-r.dir.unit_vector();
        let n = &hit_record.normal;
        let l_c = &self.color;
        let m_c = &hit_record.material.color(r, hit_record);
        let l_p = hit_record.material.phong();
        let ior = hit_record.material.refraction();

        M::intensity(l, v, n, l_c, m_c, l_p, ior)
    }
}
