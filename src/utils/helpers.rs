use serde::{Deserialize, Deserializer};
use crate::math::{Vec3, Vec4, Color};

pub fn parse_vec3<'de, D>(deserializer: D) -> Result<Vec3, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    pub struct Position {
        #[serde(rename = "@x")]
        pub x: f32,
        #[serde(rename = "@y")]
        pub y: f32,
        #[serde(rename = "@z")]
        pub z: f32,
    }

    let pos = Position::deserialize(deserializer)?;
    Ok(Vec3::from_values(pos.x, pos.y, pos.z))
}

pub fn parse_vec4<'de, D>(deserializer: D) -> Result<Vec4, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    pub struct Position {
        #[serde(rename = "@x")]
        pub x: f32,
        #[serde(rename = "@y")]
        pub y: f32,
        #[serde(rename = "@z")]
        pub z: f32,
        #[serde(rename = "@w")]
        pub w: f32,
    }

    let pos = Position::deserialize(deserializer)?;
    Ok(Vec4::from_values(pos.x, pos.y, pos.z, pos.w))
}

pub fn parse_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    pub struct Col {
        #[serde(rename = "@r")]
        pub r: f32,
        #[serde(rename = "@g")]
        pub g: f32,
        #[serde(rename = "@b")]
        pub b: f32,
    }

    let color = Col::deserialize(deserializer)?;
    Ok(Color::from_values(color.r, color.g, color.b))
}
