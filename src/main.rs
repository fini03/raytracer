use std::{error::Error, f32::consts, sync::OnceLock};
use rand::prelude::*;
use rand_xoshiro::Xoshiro256StarStar;
use rayon::prelude::*;

mod camera;
mod light;
mod surface;
mod kdtree;
mod scene;
mod math;
mod utils;
mod ray;
mod io;
mod raytracer;

use math::{Color, Vec3};
use ray::{Hittable, HittableList, Ray};
use scene::Scene;
use indicatif::ParallelProgressIterator;
use kdtree::KDTree;
use light::{LightModel, Lights, Phong, CookTorrance};
use utils::config::Config;
use crate::io::SceneWriter;
use crate::raytracer::*;

static CONFIG: OnceLock<Config> = OnceLock::new();

fn ray_color<H, M, R>(
    r: &Ray,
    scene: &Scene,
    config: &Config,
    hittables: &H,
    lights: &Lights<M, H, R>,
    bounce: usize,
    rng: &mut R,
) -> Color
where
    H: Hittable,
    M: LightModel,
    R: Rng,
{
    hittables
        .hit(r, 0., f32::INFINITY)
        .map(|hit| {
            let unit_normal = hit.normal.unit_vector();
            let color = lights.intensity(r, &hit, hittables, rng);

            // Reached max bounces, return the color
            if bounce > scene.camera.max_bounces {
                return color;
            }

            if config.fresnel {
                mix_fresnel(
                    r,
                    scene,
                    config,
                    hittables,
                    lights,
                    bounce,
                    &hit,
                    &unit_normal,
                    &color,
                    rng,
                )
            } else {
                mix_refraction_reflection(
                    r,
                    scene,
                    config,
                    hittables,
                    lights,
                    bounce,
                    &hit,
                    &unit_normal,
                    &color,
                    rng,
                )
            }
        })
        .unwrap_or(scene.background_color.clone())
}

fn render<H, M, R>(
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
    let base_seed = rng.gen::<u64>();
    data.par_chunks_exact_mut(3)
        .enumerate()
        .progress_count((width * height) as u64)
        .for_each(|(i, slice)| {
            let y = height - 1 - (i / width);
            let x = i % width;

            let r = scene.camera.get_ray(x as f32, y as f32);
            let mut chunk_rng = R::seed_from_u64(base_seed + i as u64);
            let color = ray_color(
                &r,
                scene,
                config,
                hittables,
                lights,
                0,
                &mut chunk_rng,
            );

            let mut int_color = [0u8; 3];
            utils::get_int_color(&mut int_color, &color);
            slice.copy_from_slice(&int_color);
        });
}

fn render_main<H, M, R>(
    mut scene: Scene,
    config: &Config,
    hittables: &H,
    lights: &Lights<M, H, R>,
    mut rng: R,
) -> Result<(), Box<dyn Error + Send + Sync>>
where
    H: Hittable,
    M: LightModel,
    R: Rng + Send + Sync + SeedableRng,
{
    let width = scene.camera.image_width;
    let height = scene.camera.image_height;
    let mut data = vec![0; width * height * 3];

    if let Some(ref anim) = config.anim {
        let duration = anim.duration;
        let fps = anim.frames_per_second;
        let mut scene_writer =
            SceneWriter::new_animated(&scene, duration, fps)?;

        let total_frames = duration as usize * fps as usize - 1;
        let camera_base = scene.camera.position.clone();

        for frame in 0..=total_frames {
            let camera_offset = 1f32
                * ((frame as f32 / total_frames as f32) * consts::PI)
                    .sin();
            scene.camera.position = &camera_base
                + Vec3::from_values(camera_offset, 0., camera_offset);

            println!("Rendering frame {}/{}", frame, total_frames);
            render_frame(
                width, height, &scene, hittables, lights, &mut data,
                &config, &mut rng,
            );
            scene_writer.write_image_data(&data)?;
        }
    } else {
        let mut scene_writer = SceneWriter::new(&scene)?;
        render_frame(
            width, height, &scene, hittables, lights, &mut data,
            &config, &mut rng,
        );
        scene_writer.write_image_data(&data)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    CONFIG.set(utils::config::get_config()?).unwrap();

    let mut scene = io::load_scene()?;
    let config = CONFIG.get().unwrap();
    rayon::ThreadPoolBuilder::new()
        .num_threads(config.num_threads)
        .build_global()
        .unwrap();

    let seed = config.random_seed;
    let rng = Xoshiro256StarStar::seed_from_u64(seed);

    if config.cook_torrance {
        if config.kdtree {
            let lights: Lights<
                CookTorrance,
                KDTree,
                Xoshiro256StarStar,
            >;
            lights = Lights::from_scene(&scene.lights.lights);

            println!("Building kdtree...");
            let mut hittables = HittableList::new();
            hittables.extend(scene.world.objects.drain(..).collect());
            let kdtree = KDTree::build(hittables);
            println!("Done building kdtree.");
            render_main(scene, config, &kdtree, &lights, rng)
        } else {
            let lights: Lights<
                CookTorrance,
                HittableList,
                Xoshiro256StarStar,
            >;
            lights = Lights::from_scene(&scene.lights.lights);

            let mut hittables = HittableList::new();
            hittables.extend(scene.world.objects.drain(..).collect());
            render_main(scene, config, &hittables, &lights, rng)
        }
    } else {
        if config.kdtree {
            let lights: Lights<Phong, KDTree, Xoshiro256StarStar>;
            lights = Lights::from_scene(&scene.lights.lights);

            println!("Building kdtree...");
            let mut hittables = HittableList::new();
            hittables.extend(scene.world.objects.drain(..).collect());
            let kdtree = KDTree::build(hittables);
            println!("Done building kdtree.");
            render_main(scene, config, &kdtree, &lights, rng)
        } else {
            let lights: Lights<
                Phong,
                HittableList,
                Xoshiro256StarStar,
            >;
            lights = Lights::from_scene(&scene.lights.lights);

            let mut hittables = HittableList::new();
            hittables.extend(scene.world.objects.drain(..).collect());
            render_main(scene, config, &hittables, &lights, rng)
        }
    }
}
