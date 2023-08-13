use serde::Deserialize;

use crate::{
    utils::helpers::parse_color,
    camera::Camera,
    light::structs::Lights,
    surface::surfaces::parse_world,
    math::Color,
    ray::HittableList
};

#[derive(Deserialize)]
pub struct Scene {
    #[serde(rename = "@output_file")]
    pub output_file: String,
    #[serde(deserialize_with = "parse_color")]
    pub background_color: Color,
    pub camera: Camera,
    pub lights: Lights,
    #[serde(default)]
    #[serde(rename = "surfaces")]
    #[serde(deserialize_with = "parse_world")]
    pub world: HittableList,
}
