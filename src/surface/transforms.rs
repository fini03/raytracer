use serde::Deserialize;
use crate::math::{Mat4, Vec3};
use crate::utils::helpers::parse_vec3;

#[derive(Clone)]
pub struct Transform {
    pub world_to_object: Mat4,
    pub normal_matrix: Mat4,
    pub object_to_world: Mat4,
}

impl<'de> Deserialize<'de> for Transform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RotationAngle {
            #[serde(rename = "@theta")]
            pub theta: f32,
        }

        #[derive(Deserialize)]
        enum Operation {
            #[serde(rename = "translate")]
            #[serde(deserialize_with = "parse_vec3")]
            Translate(Vec3),
            #[serde(rename = "scale")]
            #[serde(deserialize_with = "parse_vec3")]
            Scale(Vec3),
            #[serde(rename = "rotateX")]
            RotateX(RotationAngle),
            #[serde(rename = "rotateY")]
            RotateY(RotationAngle),
            #[serde(rename = "rotateZ")]
            RotateZ(RotationAngle),
        }

        #[derive(Deserialize)]
        struct TransformList {
            #[serde(default)]
            #[serde(rename = "$value")]
            pub transforms: Vec<Operation>,
        }

        let TransformList {
            transforms,
        } = TransformList::deserialize(deserializer)?;

        let world_to_object = transforms
            .iter()
            .map(|t| match t {
                Operation::Translate(v) => Mat4::translate(&-v),
                Operation::Scale(v) => Mat4::scale(&(1. / v)),
                Operation::RotateX(t) => Mat4::rotate_x(-t.theta),
                Operation::RotateY(t) => Mat4::rotate_y(-t.theta),
                Operation::RotateZ(t) => Mat4::rotate_z(-t.theta),
            })
            .rev()
            .fold(Mat4::identity(), |acc, e| &acc * &e);

        let object_to_world = transforms
            .into_iter()
            .map(|t| match t {
                Operation::Translate(v) => Mat4::translate(&v),
                Operation::Scale(v) => Mat4::scale(&v),
                Operation::RotateX(t) => Mat4::rotate_x(t.theta),
                Operation::RotateY(t) => Mat4::rotate_y(t.theta),
                Operation::RotateZ(t) => Mat4::rotate_z(t.theta),
            })
            .fold(Mat4::identity(), |acc, e| &acc * &e);

        let normal_matrix = world_to_object.transpose();

        Ok(Transform {
            world_to_object,
            object_to_world,
            normal_matrix,
        })
    }
}
