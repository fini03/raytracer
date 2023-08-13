use serde::Deserialize;
use crate::utils::helpers::parse_vec3;
use crate::math::{Point3, Vec3, Mat4};
use crate::ray::Ray;

pub struct Camera {
    pub image_width: usize,
    pub image_height: usize,
    pub max_bounces: usize,
    pub position: Point3,
    fwidth: f32,
    fheight: f32,

    look_at: Mat4,
    fov: Vec3,
}

impl Camera {
    pub fn from_values(
        position: Point3,
        look_at: Point3,
        up: Vec3,
        horizontal_fov: f32,
        image_width: usize,
        image_height: usize,
        max_bounces: usize,
    ) -> Self {
        // TODO: invert ratio when necessary
        let aspect_ratio = image_height as f32 / image_width as f32;
        let fov_x = horizontal_fov.to_radians();
        let fov_y = fov_x * aspect_ratio;
        let fov = Vec3::from_values(fov_x.tan(), fov_y.tan(), 1.);
        let look_at = Mat4::look_at(&position, &look_at, &up);

        Self {
            image_width,
            image_height,
            fwidth: image_width as f32,
            fheight: image_height as f32,
            max_bounces,

            position,
            look_at,
            fov,
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let x_n = (x + 0.5) / self.fwidth;
        let y_n = (y + 0.5) / self.fheight;

        let plane_point: Point3 = &self.look_at
            * &(Vec3::from_values(2. * x_n - 1., 2. * y_n - 1., -1.)
                * &self.fov);

        Ray::from_values(
            &self.position,
            &(plane_point - &self.position).unit_vector(),
        )
    }

    pub fn get_aperture_ray(
        &self,
        x_offset: f32,
        y_offset: f32,
        focal_point: &Point3,
    ) -> Ray {
        let aperture_position = &self.position
            + Point3::from_values(x_offset, y_offset, 0.);

        Ray::from_values(
            &aperture_position,
            &(focal_point - &aperture_position).unit_vector(),
        )
    }
}

impl<'de> Deserialize<'de> for Camera {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Angle {
            #[serde(rename = "@angle")]
            degrees: f32,
        }

        #[derive(Deserialize)]
        struct Resolution {
            #[serde(rename = "@horizontal")]
            width: usize,
            #[serde(rename = "@vertical")]
            height: usize,
        }

        #[derive(Deserialize)]
        struct MaxBounces {
            #[serde(rename = "@n")]
            n: usize,
        }

        #[derive(Deserialize)]
        struct BaseCamera {
            #[serde(deserialize_with = "parse_vec3")]
            position: Point3,
            #[serde(rename = "lookat")]
            #[serde(deserialize_with = "parse_vec3")]
            look_at: Point3,
            #[serde(deserialize_with = "parse_vec3")]
            up: Vec3,
            horizontal_fov: Angle,
            resolution: Resolution,
            max_bounces: MaxBounces,
        }

        let BaseCamera {
            position,
            look_at,
            up,
            horizontal_fov,
            resolution,
            max_bounces,
        } = BaseCamera::deserialize(deserializer)?;

        Ok(Camera::from_values(
            position,
            look_at,
            up,
            horizontal_fov.degrees,
            resolution.width,
            resolution.height,
            max_bounces.n,
        ))
    }
}
