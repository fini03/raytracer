use crate::{
    math::{Color, Vec3},
    ray::{Hittable, HitRecord, Ray},
    scene::Scene,
    light::{LightModel, Lights},
    utils::config::{Config, SamplingStrategy},
    utils::get_int_color,
    ray_color,
    render
};

use indicatif::ParallelProgressIterator;
use rand::distributions::Uniform;
use rand::prelude::*;
use rayon::prelude::*;

pub fn reflection<H, M, R>(
    r: &Ray,
    scene: &Scene,
    config: &Config,
    hittables: &H,
    lights: &Lights<M, H, R>,
    bounce: usize,
    hit: &HitRecord,
    normal: &Vec3,
    rng: &mut R,
) -> Color
where
    H: Hittable,
    M: LightModel,
    R: Rng,
{
    let direction = r.dir.reflect(&normal).unit_vector();
    let origin = &hit.p + &direction * 0.01;
    let reflect_ray = Ray::from_values(&origin, &direction);
    ray_color(
        &reflect_ray,
        scene,
        config,
        hittables,
        lights,
        bounce + 1,
        rng,
    )
}

pub fn refraction<H, M, R>(
    r: &Ray,
    scene: &Scene,
    config: &Config,
    hittables: &H,
    lights: &Lights<M, H, R>,
    bounce: usize,
    hit: &HitRecord,
    normal: &Vec3,
    rng: &mut R,
) -> Color
where
    H: Hittable,
    M: LightModel,
    R: Rng,
{
    let icd = r.dir.unit_vector();
    let mut n = normal.clone();
    let mut cosi = icd.dot(&n).clamp(-1., 1.);
    let eta = if cosi < 0. {
        // We are outside the surface, we want cos(theta)
        // to be positive
        cosi = -cosi;
        1. / hit.material.refraction()
    } else {
        // We are inside the surface, cos(theta) is
        // already positive but reverse normal direction
        n = -normal;
        hit.material.refraction() / 1.
    };

    let k = 1. - eta * eta * (1. - cosi * cosi);
    let direction = if k < 0. {
        r.dir.reflect(&normal).unit_vector()
    } else {
        let v = eta * &icd + (eta * cosi - k.sqrt()) * n;
        v.unit_vector()
    };

    let origin = &hit.p + &direction * 0.01;
    let refract_ray = Ray::from_values(&origin, &direction);
    ray_color(
        &refract_ray,
        scene,
        config,
        hittables,
        lights,
        bounce + 1,
        rng,
    )
}

pub fn mix_refraction_reflection<H, M, R>(
    r: &Ray,
    scene: &Scene,
    config: &Config,
    hittables: &H,
    lights: &Lights<M, H, R>,
    bounce: usize,
    hit: &HitRecord,
    normal: &Vec3,
    base_color: &Color,
    rng: &mut R,
) -> Color
where
    H: Hittable,
    M: LightModel,
    R: Rng,
{
    // Reflect rays if we need to
    let reflectance = hit.material.reflectance();
    let reflected_color = if reflectance > f32::EPSILON {
        reflectance
            * reflection(
                r, scene, config, hittables, lights, bounce, hit,
                normal, rng,
            )
    } else {
        Color::new()
    };

    // Calculate refraction
    let transmittance = hit.material.transmittance();
    let refracted_color = if transmittance > f32::EPSILON {
        transmittance
            * refraction(
                r, scene, config, hittables, lights, bounce, hit,
                normal, rng,
            )
    } else {
        Color::new()
    };

    base_color * (1. - reflectance - transmittance)
        + reflected_color
        + refracted_color
}

fn fresnel(ior: f32, normal: &Vec3, icd: &Vec3) -> f32 {
    let eta_i;
    let eta_t;

    let cos_i = icd.dot(&normal).clamp(-1., 1.);
    if cos_i > 0. {
        eta_i = ior;
        eta_t = 1.;
    } else {
        eta_i = 1.;
        eta_t = ior;
    }

    // Use snell's law to get sin_t
    let sin_t = eta_i / eta_t * (1. - cos_i * cos_i).max(0.).sqrt();
    if sin_t >= 1. {
        return 1.;
    }

    let cos_t = (1. - sin_t * sin_t).max(0.).sqrt();
    let cos_i = cos_i.abs();

    let r_s = (eta_t * cos_i - eta_i * cos_t)
        / (eta_t * cos_i + eta_i * cos_t);
    let r_p = (eta_i * cos_i - eta_t * cos_t)
        / (eta_i * cos_i + eta_t * cos_t);

    (r_s * r_s + r_p * r_p) / 2.
}


pub fn mix_fresnel<H, M, R>(
    r: &Ray,
    scene: &Scene,
    config: &Config,
    hittables: &H,
    lights: &Lights<M, H, R>,
    bounce: usize,
    hit: &HitRecord,
    normal: &Vec3,
    base_color: &Color,
    rng: &mut R,
) -> Color
where
    H: Hittable,
    M: LightModel,
    R: Rng,
{
    // Check if we need to mix at all
    let m_reflectance = hit.material.reflectance();
    let m_transmittance = hit.material.transmittance();
    if m_reflectance + m_transmittance <= f32::EPSILON {
        return base_color.clone();
    }

    // Find the contribution values
    let contrib_reflect;
    let contrib_refract;
    // TODO: hmm?
    //let contrib_base = (1. - m_reflectance) * (1. - m_transmittance);
    let contrib_base = 1. - m_reflectance - m_transmittance;
    if m_reflectance > f32::EPSILON && m_transmittance > f32::EPSILON {
        let fr = fresnel(
            hit.material.refraction(),
            normal,
            &r.dir.unit_vector(),
        );

        // TODO: Hmm?
        contrib_reflect = m_reflectance + m_transmittance * fr;
        contrib_refract = m_transmittance * (1. - fr);
    } else {
        contrib_reflect = m_reflectance;
        contrib_refract = m_transmittance;
    }

    // Gather reflection color
    let reflected_color = if contrib_reflect > f32::EPSILON {
        reflection(
            r, scene, config, hittables, lights, bounce, hit, normal,
            rng,
        )
    } else {
        Color::new()
    };

    // Gather refracted color
    let refracted_color = if contrib_refract > f32::EPSILON {
        refraction(
            r, scene, config, hittables, lights, bounce, hit, normal,
            rng,
        )
    } else {
        Color::new()
    };

    contrib_base * base_color
        + contrib_reflect * reflected_color
        + contrib_refract * refracted_color
}

pub fn render_supersampled<H, M, R>(
    width: usize,
    height: usize,
    scene: &Scene,
    hittables: &H,
    lights: &Lights<M, H, R>,
    data: &mut [u8],
    config: &Config,
    super_sampling: &SamplingStrategy,
    rng: &mut R,
) where
    H: Hittable,
    M: LightModel,
    R: Rng + Send + Sync + SeedableRng,
{
    let frand = Uniform::new(-0.5, 0.5);
    let base_seed = rng.gen::<u64>();
    data.par_chunks_exact_mut(3)
        .enumerate()
        .progress_count((width * height) as u64)
        .for_each(|(i, slice)| {
            let y = height - 1 - (i / width);
            let x = i % width;
            let rng = &mut R::seed_from_u64(base_seed + i as u64);
            let mut color = Color::new();

            if let Some(ref dof) = config.dof {
                let primary = scene.camera.get_ray(x as f32, y as f32);
                let focal_point = primary.at(dof.focal_length);
                let aperture = dof.aperture;

                match super_sampling {
                    SamplingStrategy::RandomSampling {
                        sample_count,
                    } => {
                        for _ in 0..*sample_count {
                            let x_offset =
                                frand.sample(rng) * aperture;
                            let y_offset =
                                frand.sample(rng) * aperture;
                            let r = scene.camera.get_aperture_ray(
                                x_offset,
                                y_offset,
                                &focal_point,
                            );

                            color += ray_color(
                                &r, scene, config, hittables, lights,
                                0, rng,
                            );
                        }

                        color = color / *sample_count as f32;
                    }
                    SamplingStrategy::Grid4x4 => {
                        let mut r;

                        color += ray_color(
                            &primary, scene, config, hittables,
                            lights, 0, rng,
                        );

                        r = scene.camera.get_aperture_ray(
                            -0.1,
                            -0.1,
                            &focal_point,
                        );
                        color += ray_color(
                            &r, scene, config, hittables, lights, 0,
                            rng,
                        );

                        r = scene.camera.get_aperture_ray(
                            0.1,
                            -0.1,
                            &focal_point,
                        );
                        color += ray_color(
                            &r, scene, config, hittables, lights, 0,
                            rng,
                        );

                        r = scene.camera.get_aperture_ray(
                            -0.1,
                            0.1,
                            &focal_point,
                        );
                        color += ray_color(
                            &r, scene, config, hittables, lights, 0,
                            rng,
                        );

                        r = scene.camera.get_aperture_ray(
                            0.1,
                            0.1,
                            &focal_point,
                        );
                        color += ray_color(
                            &r, scene, config, hittables, lights, 0,
                            rng,
                        );

                        color = color / 5.;
                    }
                }
            } else {
                match super_sampling {
                    SamplingStrategy::RandomSampling {
                        sample_count,
                    } => {
                        for _ in 0..*sample_count {
                            let x_offset = frand.sample(rng);
                            let y_offset = frand.sample(rng);
                            let r = scene.camera.get_ray(
                                x as f32 + x_offset,
                                y as f32 + y_offset,
                            );

                            color += ray_color(
                                &r, scene, config, hittables, lights,
                                0, rng,
                            );
                        }

                        color = color / *sample_count as f32;
                    }
                    SamplingStrategy::Grid4x4 => {
                        let mut r;
                        let fx = x as f32;
                        let fy = y as f32;

                        r = scene.camera.get_ray(fx, fy);
                        color += ray_color(
                            &r, scene, config, hittables, lights, 0,
                            rng,
                        );

                        r = scene.camera.get_ray(fx - 0.25, fy - 0.25);
                        color += ray_color(
                            &r, scene, config, hittables, lights, 0,
                            rng,
                        );

                        r = scene.camera.get_ray(fx + 0.25, fy - 0.25);
                        color += ray_color(
                            &r, scene, config, hittables, lights, 0,
                            rng,
                        );

                        r = scene.camera.get_ray(fx - 0.25, fy + 0.25);
                        color += ray_color(
                            &r, scene, config, hittables, lights, 0,
                            rng,
                        );

                        r = scene.camera.get_ray(fx + 0.25, fy + 0.25);
                        color += ray_color(
                            &r, scene, config, hittables, lights, 0,
                            rng,
                        );

                        color = color / 5.;
                    }
                }
            }

            // Convert color to image color
            let mut int_color = [0u8; 3];
            get_int_color(&mut int_color, &color);
            slice.copy_from_slice(&int_color);
        });
}

pub fn render_frame<H, M, R>(
    width: usize,
    height: usize,
    scene: &Scene,
    hittables: &H,
    lights: &Lights<M, H, R>,
    data: &mut [u8],
    config: &Config,
    rng: &mut R,
) where
    H: Hittable,
    M: LightModel,
    R: Rng + Send + Sync + SeedableRng,
{
    if let Some(ref super_sampling) = config.super_sampling {
        render_supersampled(
            width,
            height,
            scene,
            hittables,
            lights,
            data,
            config,
            super_sampling,
            rng,
        );
    } else {
        render(
            width, height, &scene, hittables, lights, data, config,
            rng,
        );
    }
}
