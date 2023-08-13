use std::sync::Arc;
use serde::{Deserialize, Deserializer, de};
use crate::{
    math::{Point3, Vec3},
    ray::{Hittable, HittableList, HitRecord, Ray},
    kdtree::AABB,
    utils::objparser::parse_obj,
    utils::helpers::parse_vec3
};

use super::{
    materials::{parse_material, parse_texture_object, Material, ColorLookup},
    transforms::Transform,
    julia::{parse_julia, Julia}
};

#[derive(Deserialize)]
pub struct Sphere {
    #[serde(rename = "@radius")]
    pub radius: f32,
    #[serde(deserialize_with = "parse_vec3")]
    pub position: Point3,
    #[serde(rename = "$value")]
    #[serde(deserialize_with = "parse_material")]
    material: Arc<dyn Material>,
    transform: Option<Transform>,
}

impl Sphere {
    fn get_intersection_t(
        &self,
        tr: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<f32> {
        // A - C
        let oc = &tr.orig - &self.position;
        // b * b
        let a = tr.dir.length_squared();
        // 2 * b * (A - C), halfed
        let half_b = oc.dot(&tr.dir);
        // (A - C) * (A - C) - r^2
        let c = oc.length_squared() - self.radius * self.radius;

        // Discriminant of quadratic formula
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut t = (-half_b - sqrtd) / a;
        if t < t_min || t_max < t {
            t = (-half_b + sqrtd) / a;
            if t < t_min || t_max < t {
                return None;
            }
        }

        Some(t)
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord> {
        // Transform ray if we have transforms
        let tr = self.transform.as_ref().map_or(r.clone(), |t| {
            let origin = &t.world_to_object * &r.orig;
            let direction = t.world_to_object.mul_dir(&r.dir);
            Ray::from_values(&origin, &direction)
        });

        // Find intersection t value
        let t = match self.get_intersection_t(&tr, t_min, t_max) {
            None => return None,
            Some(t) => t,
        };

        // Intersection point and normal
        let p = tr.at(t);
        let outward_normal = (&p - &self.position) / self.radius;
        let outward_normal = self
            .transform
            .as_ref()
            .map_or(outward_normal.clone(), |t| {
                t.normal_matrix.mul_dir(&outward_normal)
            });

        // Texture coordinates
        let d = (&p - &self.position).unit_vector();
        let u = 0.5 + d.x.atan2(d.z) / std::f32::consts::TAU;
        let v = 0.5 - d.y.asin() / std::f32::consts::PI;

        Some(HitRecord::from_values(
            r,
            r.at(t),
            &outward_normal.unit_vector(),
            t,
            Vec3::from_values(u, v, 1.),
            self.material.clone(),
        ))
    }

    fn shadow_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        // Transform ray if we have transforms
        let tr = self.transform.as_ref().map_or(r.clone(), |t| {
            let origin = &t.world_to_object * &r.orig;
            let direction = t.world_to_object.mul_dir(&r.dir);
            Ray::from_values(&origin, &direction)
        });

        self.get_intersection_t(&tr, t_min, t_max).is_some()
    }

    fn bound(&self) -> AABB {
        if let Some(t) = self.transform.as_ref() {
            spherical_transformed_aabb(&self.position, self.radius, t)
        } else {
            spherical_aabb(&self.position, self.radius)
        }
    }
}

pub fn spherical_aabb(position: &Vec3, radius: f32) -> AABB {
    let offset = Vec3::from_values(radius, radius, radius);

    AABB {
        min: position - &offset,
        max: position + &offset,
    }
}

pub fn spherical_transformed_aabb(
    position: &Vec3,
    radius: f32,
    transform: &Transform,
) -> AABB {
    let m = &transform.object_to_world;
    let pos = m * position;
    let r2 = radius * radius;
    let offset = Vec3::from_values(
        (r2 * (m.e[0] * m.e[0] + m.e[1] * m.e[1] + m.e[2] * m.e[2]))
            .sqrt(),
        (r2 * (m.e[4] * m.e[4] + m.e[5] * m.e[5] + m.e[6] * m.e[6]))
            .sqrt(),
        (r2 * (m.e[8] * m.e[8] + m.e[9] * m.e[9] + m.e[10] * m.e[10]))
            .sqrt(),
    );

    AABB {
        min: &pos - &offset,
        max: &pos + &offset,
    }
}

pub fn parse_world<'de, D>(
    deserializer: D,
) -> Result<HittableList, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Surfaces {
        #[serde(default)]
        #[serde(rename = "$value")]
        pub surfaces: Vec<Surface>,
    }

    #[derive(Deserialize)]
    pub enum Surface {
        #[serde(rename = "sphere")]
        SurfSphere(Sphere),
        #[serde(rename = "julia")]
        #[serde(deserialize_with = "parse_julia")]
        SurfJulia(Julia),
        #[serde(rename = "mesh")]
        #[serde(deserialize_with = "parse_mesh")]
        SurfMesh(Vec<Box<dyn Hittable>>),
    }
    use Surface::*;

    let Surfaces {
        surfaces,
    } = Surfaces::deserialize(deserializer)?;
    let mut hittable_list = HittableList::new();
    surfaces.into_iter().for_each(|s| match s {
        SurfSphere(s) => hittable_list.add(Box::new(s)),
        SurfJulia(j) => hittable_list.add(Box::new(j)),
        SurfMesh(s) => hittable_list.extend(s),
    });

    Ok(hittable_list)
}

pub fn parse_mesh<'de, D>(
    deserializer: D,
) -> Result<Vec<Box<dyn Hittable>>, D::Error>
where
    D: Deserializer<'de>,
{
    use std::path::PathBuf;

    #[derive(Deserialize)]
    pub struct BaseMesh {
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(deserialize_with = "parse_texture_object")]
        // TODO: Why is this default here needed?
        #[serde(default)]
        pub normal_map: Option<Box<dyn ColorLookup>>,
        #[serde(rename = "$value")]
        #[serde(deserialize_with = "parse_material")]
        pub material: Arc<dyn Material>,
        pub transform: Option<Transform>,
    }

    let BaseMesh {
        name,
        material,
        normal_map,
        transform,
    } = BaseMesh::deserialize(deserializer)?;

    // Load obj
    let mut path = PathBuf::new();
    path.push(r"../scenes");
    path.push(&name);
    let data = parse_obj(&path, material, normal_map, transform)
        .map_err(|e| de::Error::custom(e.to_string()))?;

    Ok(data)
}
