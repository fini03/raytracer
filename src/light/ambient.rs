use super::light::*;
use crate::math::Color;
use crate::ray::{Ray, Hittable, HitRecord};
use rand::Rng;

pub struct Ambient {
    pub color: Color,
}

impl<M, H, R> LightSource<M, H, R> for Ambient
where
    M: LightModel,
    H: Hittable,
    R: Rng,
{
    fn intensity(
        &self,
        r: &Ray,
        hit_record: &HitRecord,
        _hittables: &H,
        _rng: &mut R,
    ) -> Color {
        let m_c = hit_record.material.color(r, hit_record);
        let l_p = hit_record.material.phong();
        &self.color * m_c * l_p.ka
    }
}

