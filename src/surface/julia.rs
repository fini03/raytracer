use serde::{Deserialize, Deserializer};
use std::sync::Arc;
use crate::{
    utils::helpers::{parse_vec3, parse_vec4},
    math::{Mat4, Point3, Vec3, Vec4},
    ray::{Hittable, HitRecord, Ray},
    kdtree::AABB
};
use super::{
    materials::{parse_material, Material},
    surfaces::spherical_transformed_aabb,
    transforms::Transform
};

const ESCAPE_THRESHOLD: f32 = 1e1;
const DEL: f32 = 1e-4;
const BOUNDING_RADIUS2: f32 = 3f32;
const BOUNDING_RADIUS: f32 = 716_035f32 / 413_403f32; // sqrt(3)

pub struct Julia {
    epsilon: f32,
    max_iterations: usize,
    mu: Vec4,
    material: Arc<dyn Material>,
    transform: Transform,
}

impl Julia {
    pub fn from_values(
        epsilon: f32,
        max_iterations: usize,
        mu: Vec4,
        material: Arc<dyn Material>,
        transform: Transform,
    ) -> Self {
        Self {
            epsilon,
            max_iterations,
            mu,
            material,
            transform,
        }
    }

    fn get_intersect_values(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(f32, Point3)> {
        // ------------------------------------------------------------
        // BOUNDING SPHERE
        // ------------------------------------------------------------

        // Transform ray
        let tr_origin = &self.transform.world_to_object * &r.orig;
        let tr_direction =
            self.transform.world_to_object.mul_dir(&r.dir);
        let tr = Ray::from_values(&tr_origin, &tr_direction);

        // Discriminant of quadratic formula
        let a = tr.dir.length_squared();
        let half_b = tr.orig.dot(&tr.dir);
        let c = tr.orig.length_squared() - BOUNDING_RADIUS2;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        // Check if any of the intersection points are in acceptable
        // range (at least some overlaps with the range)
        let sqrtd = discriminant.sqrt();
        let mut t1 = (-half_b - sqrtd) / a;
        let mut t2 = (-half_b + sqrtd) / a;
        if (t_max - t1) * (t2 - t_min) < 0. {
            return None;
        }

        // Make sure both our t values are sensible
        // t1 should always be the smaller one, so that we can check
        // for t2 if we're leaving the scene
        t1 = t1.clamp(t_min, t_max);
        t2 = t2.clamp(t_min, t_max);

        // ------------------------------------------------------------
        // JULIA SET
        // ------------------------------------------------------------

        let mut t = t1;
        let mut origin = tr.at(t);

        let dist = loop {
            let mut z = Vec4::from_vec3(&origin);
            let mut zp = Vec4::from_values(1., 0., 0., 0.);

            for _ in 0..self.max_iterations {
                zp = 2. * z.quat_mult(&zp);
                z = z.quat_sq() + &self.mu;
                if z.length_squared() > ESCAPE_THRESHOLD {
                    break;
                }
            }

            // Find distance lower bound
            let norm_z = z.length();
            let dist = 0.5 * norm_z * norm_z.ln() / zp.length();

            // Step this far along the ray
            t += dist;
            origin = tr.at(t);

            // Are we close enough to the surface?
            // Have we left the bounding sphere?
            if dist < self.epsilon || t > t2 {
                break dist;
            }
        };

        // Check if we have a hit
        if dist >= self.epsilon {
            return None;
        }

        Some((t, origin))
    }
}

impl Hittable for Julia {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord> {
        let isect = match self.get_intersect_values(r, t_min, t_max) {
            None => return None,
            Some(v) => v,
        };
        let (t, origin) = isect;

        // Estimate normal
        let p = Vec4::from_vec3(&origin);
        let mut gx1 = &p - Vec4::from_values(DEL, 0., 0., 0.);
        let mut gx2 = &p + Vec4::from_values(DEL, 0., 0., 0.);
        let mut gy1 = &p - Vec4::from_values(0., DEL, 0., 0.);
        let mut gy2 = &p + Vec4::from_values(0., DEL, 0., 0.);
        let mut gz1 = &p - Vec4::from_values(0., 0., DEL, 0.);
        let mut gz2 = &p + Vec4::from_values(0., 0., DEL, 0.);

        for _ in 0..self.max_iterations {
            gx1 = gx1.quat_sq() + &self.mu;
            gx2 = gx2.quat_sq() + &self.mu;
            gy1 = gy1.quat_sq() + &self.mu;
            gy2 = gy2.quat_sq() + &self.mu;
            gz1 = gz1.quat_sq() + &self.mu;
            gz2 = gz2.quat_sq() + &self.mu;
        }

        let grad_x = gx2.length() - gx1.length();
        let grad_y = gy2.length() - gy1.length();
        let grad_z = gz2.length() - gz1.length();
        let normal = self
            .transform
            .normal_matrix
            .mul_dir(&Vec3::from_values(grad_x, grad_y, grad_z));

        Some(HitRecord::from_values(
            r,
            r.at(t),
            &normal.unit_vector(),
            t,
            Vec3::from_values(0., 0., 1.),
            self.material.clone(),
        ))
    }

    fn shadow_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        self.get_intersect_values(r, t_min, t_max).is_some()
    }

    fn bound(&self) -> AABB {
        spherical_transformed_aabb(
            &Vec3::new(),
            BOUNDING_RADIUS,
            &self.transform,
        )
    }
}

pub fn parse_julia<'de, D>(deserializer: D) -> Result<Julia, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    pub struct BaseJulia {
        #[serde(rename = "@radius")]
        pub radius: f32,
        #[serde(rename = "@epsilon")]
        pub epsilon: f32,
        #[serde(rename = "@max_iterations")]
        pub max_iterations: usize,
        #[serde(deserialize_with = "parse_vec3")]
        pub position: Point3,
        #[serde(deserialize_with = "parse_vec4")]
        pub mu: Vec4,
        #[serde(rename = "$value")]
        #[serde(deserialize_with = "parse_material")]
        pub material: Arc<dyn Material>,
        pub transform: Option<Transform>,
    }

    let BaseJulia {
        radius,
        epsilon,
        max_iterations,
        position,
        mu,
        material,
        transform,
    } = BaseJulia::deserialize(deserializer)?;

    // We're going to encode radius and position into transformation
    // matrices directly, because this is easier to handle for us.
    // Matrix for scaling according to the radius:
    let radius_scale_vec = Vec3::from_values(
        radius / BOUNDING_RADIUS,
        radius / BOUNDING_RADIUS,
        radius / BOUNDING_RADIUS,
    );
    let radius_scale = Mat4::scale(&radius_scale_vec);
    let radius_scale_inv = Mat4::scale(&(1. / radius_scale_vec));
    let translate = Mat4::translate(&position);
    let translate_inv = Mat4::translate(&-&position);

    // Combine matrices
    let mut world_to_object = &radius_scale_inv * &translate_inv;
    let mut object_to_world = &translate * &radius_scale;
    if let Some(t) = transform {
        world_to_object = &world_to_object * &t.world_to_object;
        object_to_world = &t.object_to_world * &object_to_world;
    }
    let normal_matrix = world_to_object.transpose();

    Ok(Julia::from_values(
        epsilon,
        max_iterations,
        mu,
        material,
        Transform {
            world_to_object,
            normal_matrix,
            object_to_world,
        },
    ))
}
