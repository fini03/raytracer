use crate::utils::helpers::{parse_color, parse_vec3};
use crate::math::{Color, Point3, Vec3};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Lights {
    #[serde(default)]
    #[serde(rename = "$value")]
    pub lights: Vec<Light>,
}

#[derive(Deserialize)]
pub enum Light {
    #[serde(rename = "ambient_light")]
    Ambient(Ambient),
    #[serde(rename = "point_light")]
    Point(Point),
    #[serde(rename = "parallel_light")]
    Parallel(Parallel),
    #[serde(rename = "spot_light")]
    Spot(Spot),
    #[serde(rename = "rectangular_area_random")]
    RectangularAreaRandom(RectangularAreaRandom),
    #[serde(rename = "rectangular_area")]
    RectangularArea(RectangularArea),
}

#[derive(Deserialize)]
pub struct Ambient {
    #[serde(deserialize_with = "parse_color")]
    pub color: Color,
}

#[derive(Deserialize)]
pub struct Point {
    #[serde(deserialize_with = "parse_color")]
    pub color: Color,
    #[serde(deserialize_with = "parse_vec3")]
    pub position: Point3,
}

#[derive(Deserialize)]
pub struct Parallel {
    #[serde(deserialize_with = "parse_color")]
    pub color: Color,
    #[serde(deserialize_with = "parse_vec3")]
    pub direction: Vec3,
}

#[derive(Deserialize)]
pub struct Spot {
    #[serde(rename = "color")]
    #[serde(deserialize_with = "parse_color")]
    pub color: Color,
    #[serde(rename = "position")]
    #[serde(deserialize_with = "parse_vec3")]
    pub position: Point3,
    #[serde(rename = "direction")]
    #[serde(deserialize_with = "parse_vec3")]
    pub direction: Vec3,
    #[serde(rename = "falloff")]
    pub fall_off: FallOff,
}

/// The angles in FallOff are the angle from the center of cone to the
/// outside, that is, half of the full spread angle
#[derive(Deserialize)]
pub struct FallOff {
    #[serde(rename = "@alpha1")]
    pub alpha1: f32,
    #[serde(rename = "@alpha2")]
    pub alpha2: f32,
}

#[derive(Deserialize)]
pub struct RectangularAreaRandom {
    #[serde(deserialize_with = "parse_color")]
    pub color: Color,
    #[serde(deserialize_with = "parse_vec3")]
    pub corner: Point3,
    #[serde(deserialize_with = "parse_vec3")]
    pub v1: Vec3,
    #[serde(deserialize_with = "parse_vec3")]
    pub v2: Vec3,
    #[serde(rename = "@num_samples")]
    pub num_samples: usize,
}

#[derive(Deserialize)]
pub struct RectangularArea {
    #[serde(deserialize_with = "parse_color")]
    pub color: Color,
    #[serde(deserialize_with = "parse_vec3")]
    pub corner: Point3,
    #[serde(deserialize_with = "parse_vec3")]
    pub v1: Vec3,
    #[serde(deserialize_with = "parse_vec3")]
    pub v2: Vec3,
    #[serde(rename = "@num_steps")]
    pub num_steps: usize,
}
