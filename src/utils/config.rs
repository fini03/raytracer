use serde::Deserialize;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default)]
    pub random_seed: u64,
    #[serde(default)]
    pub cook_torrance: bool,
    #[serde(default)]
    pub fresnel: bool,
    #[serde(default)]
    pub kdtree: bool,
    #[serde(default)]
    pub num_threads: usize,
    pub super_sampling: Option<SamplingStrategy>,
    pub dof: Option<DepthOfField>,
    pub anim: Option<Animation>,
    #[serde(default)]
    pub texture_interpolation: TextureInterpolation,
}

#[derive(Deserialize, Debug, Default)]
pub enum SamplingStrategy {
    #[default]
    Grid4x4,
    RandomSampling {
        sample_count: usize,
    },
}

#[derive(Deserialize, Debug, Default)]
pub enum TextureInterpolation {
    #[default]
    Nearest,
    Linear,
}

#[derive(Deserialize, Debug)]
pub struct DepthOfField {
    pub focal_length: f32,
    pub aperture: f32,
}

#[derive(Deserialize, Debug)]
pub struct Animation {
    pub duration: u32,
    pub frames_per_second: u16,
}

pub fn get_config() -> Result<Config, Box<dyn Error + Send + Sync>> {
    if let Some(file_path) = args().nth(2) {
        let mut contents = String::new();
        File::open(file_path)?.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents)?)
    } else {
        Ok(Config::default())
    }
}
