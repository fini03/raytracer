use crate::{
    math::{Color, Point3, Vec3},
    light::structs::Light as SceneLight,
    surface::materials::Phong as LightParameters,
    ray::{Ray, Hittable, HitRecord},
};
use super::{
    ambient::Ambient,
    point::Point,
    parallel::Parallel,
    spot::Spot
};

use rand::{distributions::Uniform, prelude::Distribution, Rng};

pub struct Lights<M, H, R>
where
    M: LightModel,
    H: Hittable,
    R: Rng,
{
    lights: Vec<Box<dyn LightSource<M, H, R> + Send + Sync>>,
}

impl<M, H, R> Lights<M, H, R>
where
    M: LightModel,
    H: Hittable,
    R: Rng,
{
    pub fn from_scene(scene_lights: &[SceneLight]) -> Self {
        let mut lights = Vec::with_capacity(scene_lights.len());

        for light in scene_lights {
            let b: Box<dyn LightSource<M, H, R> + Send + Sync> =
                match light {
                    SceneLight::Ambient(a) => {
                        let l = Ambient {
                            color: a.color.clone(),
                        };
                        Box::new(l)
                    }
                    SceneLight::Parallel(p) => {
                        let l = Parallel {
                            color: p.color.clone(),
                            direction: p.direction.unit_vector(),
                        };
                        Box::new(l)
                    }
                    SceneLight::Point(p) => {
                        let l = Point {
                            color: p.color.clone(),
                            position: p.position.clone(),
                        };
                        Box::new(l)
                    }
                    SceneLight::Spot(s) => {
                        let alpha1 = s.fall_off.alpha1.to_radians();
                        let alpha2 = s.fall_off.alpha2.to_radians();

                        let l = Spot {
                            color: s.color.clone(),
                            position: s.position.clone(),
                            direction: s.direction.unit_vector(),
                            alpha1,
                            alpha2,
                            alpha_range: alpha2 - alpha1,
                        };
                        Box::new(l)
                    }
                    SceneLight::RectangularAreaRandom(r) => {
                        let l = RectangularAreaRandom {
                            color: r.color.clone(),
                            corner: r.corner.clone(),
                            v1: r.v1.clone(),
                            v2: r.v2.clone(),
                            num_samples: r.num_samples,
                        };

                        Box::new(l)
                    }
                    SceneLight::RectangularArea(r) => {
                        let l = RectangularArea {
                            color: r.color.clone(),
                            corner: r.corner.clone(),
                            v1: r.v1.clone(),
                            v2: r.v2.clone(),
                            num_steps: r.num_steps,
                        };

                        Box::new(l)
                    }
                };

            lights.push(b);
        }

        Self {
            lights,
        }
    }

    pub fn intensity(
        &self,
        r: &Ray,
        hit_record: &HitRecord,
        hittables: &H,
        rng: &mut R,
    ) -> Color
    where
        H: Hittable,
    {
        let mut color = Color::new();

        for light in &self.lights {
            color += light.intensity(r, hit_record, hittables, rng);
        }

        color
    }
}

pub trait LightSource<M, H, R>
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
        rng: &mut R,
    ) -> Color;
}

pub trait LightModel {
    fn intensity(
        light_direction: &Vec3,
        view_direction: &Vec3,
        normal: &Vec3,
        light_color: &Color,
        material_color: &Color,
        light_parameters: &LightParameters,
        index_of_refraction: f32,
    ) -> Color;
}

pub struct Phong;

impl LightModel for Phong {
    fn intensity(
        l: &Vec3,
        v: &Vec3,
        n: &Vec3,
        l_c: &Color,
        m_c: &Color,
        l_p: &LightParameters,
        _ior: f32,
    ) -> Color {
        let diffuse = l_c * m_c * l.dot(n).max(0.);
        let r = -l.reflect(n);
        let spec = r.unit_vector().dot(&v).max(0.).powf(l_p.exponent);
        let specular = l_c * spec;
        diffuse * l_p.kd + specular * l_p.ks
    }
}

pub struct CookTorrance;

impl LightModel for CookTorrance {
    fn intensity(
        l: &Vec3,
        v: &Vec3,
        n: &Vec3,
        l_c: &Color,
        m_c: &Color,
        l_p: &LightParameters,
        ior: f32,
    ) -> Color {
        let h = (l + v).unit_vector();

        // Pre-compute the needed dot products for all parts of
        // the equation
        let dot_n_l = n.dot(&l).abs().max(0.0001);
        let dot_n_h = n.dot(&h).abs().max(0.0001);
        let dot_n_v = n.dot(&v).abs().max(0.0001);
        let dot_h_v = h.dot(&v).abs().max(0.0001);

        // Calculate the diffuse part (with material color)
        let brdf_diffuse = m_c;

        // Geometric term
        let g = 2. * dot_n_h / dot_h_v;
        let s_g = (dot_n_v.min(dot_n_l) * g).min(1.);

        // NDF: Beckmann distribution
        let alpha = (2. / (l_p.exponent - 2.)).sqrt();
        let pi_alpha2 = std::f32::consts::PI * alpha * alpha;
        let cos2h = dot_n_h * dot_n_h;
        let sin2h = (1. - cos2h).max(0.);
        let tan2h = sin2h / cos2h;
        let cos4h = cos2h * cos2h;

        // Distribution value
        let s_d = if tan2h.is_infinite() {
            0f32
        } else {
            (-tan2h / (alpha * alpha)).exp() / (pi_alpha2 * cos4h)
        };

        // Fresnel (Schlick's approximation)
        let n = ior;
        let f_0 = (n - 1.) * (n - 1.) / ((n + 1.) * (n + 1.));
        let s_f = f_0 + (1. - f_0) * (1. - dot_h_v).powi(5);

        // Specular BRDF
        let r_s = s_f * s_d * s_g / (dot_n_v * dot_n_l * 4.);
        let brdf_specular = Vec3::from_values(r_s, r_s, r_s);

        // Putting it all together, the math is a little bit sus tho
        let specular_diffuse = l_c
            * dot_n_l
            * (l_p.kd * brdf_diffuse + l_p.ks * brdf_specular);

        specular_diffuse
    }
}

pub struct RectangularAreaRandom {
    color: Color,
    corner: Point3,
    v1: Vec3,
    v2: Vec3,
    num_samples: usize,
}

impl<M, H, R> LightSource<M, H, R> for RectangularAreaRandom
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
        rng: &mut R,
    ) -> Color {
        let p = &hit_record.p;
        let l_c = &self.color;
        let n = &hit_record.normal;
        let m_c = &hit_record.material.color(r, hit_record);
        let l_p = hit_record.material.phong();
        let ior = hit_record.material.refraction();
        let mut color = Color::new();
        let frand = Uniform::new(0.0, 1.0);

        for _ in 0..self.num_samples {
            let u = frand.sample(rng);
            let v = frand.sample(rng);

            let position = &self.corner + &self.v1 * u + &self.v2 * v;
            let l_not_norm = &position - p;
            let len = l_not_norm.length();
            let l = &(l_not_norm / len);
            let s_ray = Ray::from_values(&p, l);
            if hittables.shadow_hit(&s_ray, 0.01, len) {
                continue;
            }

            let v = &-r.dir.unit_vector();
            color += M::intensity(l, v, n, l_c, m_c, l_p, ior);
        }

        color / self.num_samples as f32
    }
}

pub struct RectangularArea {
    color: Color,
    corner: Point3,
    v1: Vec3,
    v2: Vec3,
    num_steps: usize,
}

impl<M, H, R> LightSource<M, H, R> for RectangularArea
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
        let l_c = &self.color;
        let n = &hit_record.normal;
        let m_c = &hit_record.material.color(r, hit_record);
        let l_p = hit_record.material.phong();
        let ior = hit_record.material.refraction();
        let mut color = Color::new();
        let num_steps_inv = 1. / self.num_steps as f32;

        for u_step in 0..self.num_steps {
            let u = &self.v1 * u_step as f32 * num_steps_inv;

            for v_step in 0..self.num_steps {
                let v = &self.v2 * v_step as f32 * num_steps_inv;

                let position = &self.corner + &u + &v;
                let l_not_norm = &position - p;
                let len = l_not_norm.length();
                let l = &(l_not_norm / len);
                let s_ray = Ray::from_values(&p, l);
                if hittables.shadow_hit(&s_ray, 0.01, len) {
                    continue;
                }

                let v = &-r.dir.unit_vector();
                color += M::intensity(l, v, n, l_c, m_c, l_p, ior);
            }
        }

        color / (self.num_steps * self.num_steps) as f32
    }
}
