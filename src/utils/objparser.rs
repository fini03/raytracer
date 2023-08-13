use std::{
    path::Path,
    error::Error,
    fs::File,
    io::{BufReader, BufRead},
    collections::HashMap,
    sync::Arc, vec
};
use crate::{
    math::{Vec3, Point3, Mat4},
    ray::{Hittable, HitRecord, Ray},
    kdtree::AABB,
    surface::{Material, Transform, ColorLookup},
};

const EPSILON: f32 = 0.0000001;

pub type Index = u32;

pub struct Mesh {
    pub vertices: Vec<Point3>,
    pub normals: Vec<Vec3>,
    pub tangents: Vec<Vec3>,
    pub bitangents: Vec<Vec3>,
    pub texcoords: Vec<Vec3>,
    pub material: Arc<dyn Material>,
    pub normal_map: Option<Box<dyn ColorLookup>>,
    pub transform: Option<Transform>,
}

pub struct Triangle {
    vertices: [Index; 3],
    hit_normal: Vec3,
    hit_d: f32,
    hit_edge1: Vec3,
    hit_edge2: Vec3,
}

impl Triangle {
    fn get_intersection(
        &self,
        mesh: &Mesh,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(f32, (usize, usize, usize), (f32, f32))> {
        let Triangle {
            vertices,
            hit_normal,
            hit_d,
            hit_edge1,
            hit_edge2,
        } = &self;

        // Transform ray if we have transforms
        let tr = mesh.transform.as_ref().map_or(r.clone(), |t| {
            let origin = &t.world_to_object * &r.orig;
            let direction = t.world_to_object.mul_dir(&r.dir);
            Ray::from_values(&origin, &direction)
        });

        // Check if ray is parallel to triangle
        let ndir = hit_normal.dot(&tr.dir);
        if ndir.abs() < EPSILON {
            // Dot product is almost 0
            // Triangle is parallel to ray
            return None;
        }

        // Compute t for ray equation
        let t = -(hit_normal.dot(&tr.orig) + hit_d) / ndir;
        if t < t_min || t_max < t {
            // t is not in acceptable range
            return None;
        }

        // Get indices
        let vi0 = vertices[0] as usize;
        let vi1 = vertices[1] as usize;
        let vi2 = vertices[2] as usize;

        // Do inside/outside test for triangle
        let v0 = &mesh.vertices[vi0];
        let h = tr.dir.cross(&hit_edge2);
        let a = hit_edge1.dot(&h);
        let f = 1. / a;
        let s = &tr.orig - v0;

        let u = f * s.dot(&h);
        if u < 0. || u > 1. {
            // TODO: Hmm which case is this?
            return None;
        }

        let q = s.cross(&hit_edge1);
        let v = f * tr.dir.dot(&q);
        if v < 0. || u + v > 1. {
            // TODO: Hmm which case is this?
            return None;
        }

        Some((t, (vi0, vi1, vi2), (u, v)))
    }
}

pub struct MeshTriangle {
    triangle: Triangle,
    mesh: Arc<Mesh>,
}

impl Hittable for MeshTriangle {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord> {
        let isect = match self
            .triangle
            .get_intersection(&self.mesh, r, t_min, t_max)
        {
            None => return None,
            Some(v) => v,
        };
        let (t, (vi0, vi1, vi2), (u, v)) = isect;

        // Interpolate the normal using barycentric coords
        let n0 = &self.mesh.normals[vi0];
        let n1 = &self.mesh.normals[vi1];
        let n2 = &self.mesh.normals[vi2];
        let outward_normal = u * n1 + v * n2 + (1. - u - v) * n0;
        let transformed_normal = self
            .mesh
            .transform
            .as_ref()
            .map_or(outward_normal.clone(), |t| {
                t.normal_matrix.mul_dir(&outward_normal)
            });

        // Interpolate the texture coords using barycentric coords
        let t0 = &self.mesh.texcoords[vi0];
        let t1 = &self.mesh.texcoords[vi1];
        let t2 = &self.mesh.texcoords[vi2];
        let tex_coords = u * t1 + v * t2 + (1. - u - v) * t0;

        // Update hitrecord
        let p = r.at(t);
        Some(HitRecord::from_values(
            r,
            p,
            &transformed_normal.unit_vector(),
            t,
            tex_coords,
            self.mesh.material.clone(),
        ))
    }

    fn shadow_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        self.triangle
            .get_intersection(&self.mesh, r, t_min, t_max)
            .is_some()
    }

    fn bound(&self) -> AABB {
        let mut aabb: AABB = Default::default();

        if let Some(t) = self.mesh.transform.as_ref() {
            for i in 0..3 {
                let vi = self.triangle.vertices[i] as usize;
                let v = &t.object_to_world * &self.mesh.vertices[vi];
                aabb.min.assign_min(&v);
                aabb.max.assign_max(&v);
            }
        } else {
            for i in 0..3 {
                let vi = self.triangle.vertices[i] as usize;
                let v = &self.mesh.vertices[vi];
                aabb.min.assign_min(&v);
                aabb.max.assign_max(&v);
            }
        }

        aabb
    }
}

pub struct MeshTangentTriangle {
    triangle: Triangle,
    mesh: Arc<Mesh>,
}

impl Hittable for MeshTangentTriangle {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord> {
        let isect = match self
            .triangle
            .get_intersection(&self.mesh, r, t_min, t_max)
        {
            None => return None,
            Some(v) => v,
        };
        let (t, (vi0, vi1, vi2), (u, v)) = isect;

        // Interpolate the normal using barycentric coords
        let n0 = &self.mesh.normals[vi0];
        let n1 = &self.mesh.normals[vi1];
        let n2 = &self.mesh.normals[vi2];
        let outward_normal = u * n1 + v * n2 + (1. - u - v) * n0;
        let transformed_normal = self
            .mesh
            .transform
            .as_ref()
            .map_or(outward_normal.clone(), |t| {
                t.normal_matrix.mul_dir(&outward_normal).unit_vector()
            });

        // Interpolate the tangent using barycentric coords
        let t0 = &self.mesh.tangents[vi0];
        let t1 = &self.mesh.tangents[vi1];
        let t2 = &self.mesh.tangents[vi2];
        let tangent = u * t1 + v * t2 + (1. - u - v) * t0;
        let transformed_tangent = self
            .mesh
            .transform
            .as_ref()
            .map_or(tangent.clone(), |t| {
                t.normal_matrix.mul_dir(&tangent).unit_vector()
            });

        // Interpolate the bitangent using barycentric coords
        let b0 = &self.mesh.bitangents[vi0];
        let b1 = &self.mesh.bitangents[vi1];
        let b2 = &self.mesh.bitangents[vi2];
        let bitangent = u * b1 + v * b2 + (1. - u - v) * b0;
        let transformed_bitangent = self
            .mesh
            .transform
            .as_ref()
            .map_or(bitangent.clone(), |t| {
                t.normal_matrix.mul_dir(&bitangent).unit_vector()
            });

        // Create TBN matrix
        let tbn = Mat4::tbn(
            &transformed_tangent,
            &transformed_bitangent,
            &transformed_normal,
        );

        // Interpolate the texture coords using barycentric coords
        let t0 = &self.mesh.texcoords[vi0];
        let t1 = &self.mesh.texcoords[vi1];
        let t2 = &self.mesh.texcoords[vi2];
        let tex_coords = u * t1 + v * t2 + (1. - u - v) * t0;

        // Create the hitrecord
        let p = r.at(t);
        let mut hit = HitRecord::from_values(
            r,
            p,
            &transformed_normal,
            t,
            tex_coords,
            self.mesh.material.clone(),
        );

        // Read the normal from the normal map (if there is one)
        if let Some(ref normal_map) = self.mesh.normal_map {
            let nt = normal_map.color(r, &hit) * 2.
                - Vec3::from_values(1., 1., 1.);

            // TODO: We have the front face in the hit record, so
            // if we update the normal, we should probably also think
            // about updating that
            hit.normal = tbn.mul_dir(&nt).unit_vector();
        }

        Some(hit)
    }

    fn shadow_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        self.triangle
            .get_intersection(&self.mesh, r, t_min, t_max)
            .is_some()
    }

    fn bound(&self) -> AABB {
        let mut aabb: AABB = Default::default();

        if let Some(t) = self.mesh.transform.as_ref() {
            for i in 0..3 {
                let vi = self.triangle.vertices[i] as usize;
                let v = &t.object_to_world * &self.mesh.vertices[vi];
                aabb.min.assign_min(&v);
                aabb.max.assign_max(&v);
            }
        } else {
            for i in 0..3 {
                let vi = self.triangle.vertices[i] as usize;
                let v = &self.mesh.vertices[vi];
                aabb.min.assign_min(&v);
                aabb.max.assign_max(&v);
            }
        }

        aabb
    }
}

pub fn parse_obj(
    filepath: &Path,
    material: Arc<dyn Material>,
    normal_map: Option<Box<dyn ColorLookup>>,
    transform: Option<Transform>,
) -> Result<Vec<Box<dyn Hittable>>, Box<dyn Error + Send + Sync>> {
    let has_normal_map = normal_map.is_some();
    let mut index_mapping = HashMap::new();
    let mut base_vertices = vec![];
    let mut base_normals = vec![];
    let mut base_texcoords = vec![];
    let mut mesh = Mesh {
        vertices: vec![],
        normals: vec![],
        texcoords: vec![],
        tangents: vec![],
        bitangents: vec![],
        normal_map,
        material,
        transform,
    };
    let mut triangles: Vec<Triangle> = vec![];
    let mut next_index = 0;

    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    'lines: for line in reader.lines() {
        let line = line?;
        let mut iter = line.split_whitespace();
        let keyword = match iter.next() {
            Some(s) => s,
            None => continue,
        };

        // Vertices and normals
        if keyword == "v" || keyword == "vn" {
            let x = match iter.next() {
                Some(s) => s.parse()?,
                None => continue,
            };
            let y = match iter.next() {
                Some(s) => s.parse()?,
                None => continue,
            };
            let z = match iter.next() {
                Some(s) => s.parse()?,
                None => continue,
            };

            if keyword == "v" {
                let v = Point3::from_values(x, y, z);
                base_vertices.push(v);
            } else {
                base_normals.push(Vec3::from_values(x, y, z));
            }
            continue;
        }

        // Texture coordinates
        if keyword == "vt" {
            let u = match iter.next() {
                Some(s) => s.parse()?,
                None => continue,
            };
            let v = match iter.next() {
                Some(s) => s.parse()?,
                None => continue,
            };

            base_texcoords.push(Vec3::from_values(u, v, 0.));
            continue;
        }

        // Faces
        if keyword == "f" {
            let mut triangle = Triangle {
                vertices: [0; 3],
                hit_normal: Vec3::new(),
                hit_d: 0.,
                hit_edge1: Vec3::new(),
                hit_edge2: Vec3::new(),
            };

            for i in 0..3 {
                let indices = match iter.next() {
                    Some(s) => s,
                    None => continue 'lines,
                };

                // Check if we already know the index_group
                if let Some(index) = index_mapping.get(indices) {
                    triangle.vertices[i] = *index;
                    continue;
                }

                // Otherwise, parse everything
                let mut vertex_iter = indices.split('/');
                let coords = match vertex_iter.next() {
                    Some(s) => s.parse::<usize>()? - 1,
                    None => continue 'lines,
                };
                let tex = match vertex_iter.next() {
                    Some(s) => s.parse::<usize>().unwrap_or(1) - 1,
                    None => continue 'lines,
                };
                let normal = match vertex_iter.next() {
                    Some(s) => s.parse::<usize>()? - 1,
                    None => continue 'lines,
                };

                // Only add texcoords if we have them
                let texcoord = if base_texcoords.len() > 0 {
                    base_texcoords[tex].clone()
                } else {
                    Vec3::new()
                };

                mesh.vertices.push(base_vertices[coords].clone());
                mesh.normals.push(base_normals[normal].clone());
                mesh.texcoords.push(texcoord);
                index_mapping.insert(indices.to_owned(), next_index);
                triangle.vertices[i] = next_index;
                next_index += 1;
            }

            // Pre-compute plane normal for hition test
            let v0 = &mesh.vertices[triangle.vertices[0] as usize];
            let v1 = &mesh.vertices[triangle.vertices[1] as usize];
            let v2 = &mesh.vertices[triangle.vertices[2] as usize];
            let v0v1 = v1 - v0;
            let v0v2 = v2 - v0;
            let n = v0v1.cross(&v0v2);
            triangle.hit_d = -n.dot(v0);
            triangle.hit_normal = n;
            triangle.hit_edge1 = v0v1;
            triangle.hit_edge2 = v0v2;

            triangles.push(triangle);
            continue;
        }
    }

    if has_normal_map {
        let num_vertices = mesh.vertices.len();
        let mut num_tangents = vec![0; num_vertices];
        let mut tangents = vec![Vec3::new(); num_vertices];
        let mut bitangents = vec![Vec3::new(); num_vertices];

        for triangle in &triangles {
            let v0i = triangle.vertices[0] as usize;
            let v1i = triangle.vertices[1] as usize;
            let v2i = triangle.vertices[2] as usize;

            let uv0 = &mesh.texcoords[v0i];
            let uv1 = &mesh.texcoords[v1i];
            let uv2 = &mesh.texcoords[v2i];

            let delta_pos1 = &triangle.hit_edge1;
            let delta_pos2 = &triangle.hit_edge2;

            let delta_uv1 = uv1 - uv0;
            let delta_uv2 = uv2 - uv0;

            let r = 1.
                / (delta_uv1.x * delta_uv2.y
                    - delta_uv1.y * delta_uv2.x);
            let tangent = r
                * (delta_pos1 * delta_uv2.y
                    - delta_pos2 * delta_uv1.y);
            let bitangent = r
                * (delta_pos2 * delta_uv1.x
                    - delta_pos1 * delta_uv2.x);

            // TODO: this might be REALLY sussy
            //let n0 = &mesh.normals[v0i];
            //let n1 = &mesh.normals[v1i];
            //let n2 = &mesh.normals[v2i];

            //tangents[v0i] += &tangent - n0 * n0.dot(&tangent);
            //tangents[v1i] += &tangent - n1 * n1.dot(&tangent);
            //tangents[v2i] += &tangent - n2 * n2.dot(&tangent);

            //bitangents[v0i] += &bitangent - n0 * n0.dot(&bitangent);
            //bitangents[v1i] += &bitangent - n1 * n1.dot(&bitangent);
            //bitangents[v2i] += &bitangent - n2 * n2.dot(&bitangent);

            tangents[v0i] += &tangent;
            tangents[v1i] += &tangent;
            tangents[v2i] += &tangent;

            bitangents[v0i] += &bitangent;
            bitangents[v1i] += &bitangent;
            bitangents[v2i] += &bitangent;

            num_tangents[v0i] += 1;
            num_tangents[v1i] += 1;
            num_tangents[v2i] += 1;
        }

        // Average the tangents for all vertices
        let it = num_tangents
            .into_iter()
            .zip(tangents.iter_mut().zip(bitangents.iter_mut()));
        for (n, (t, b)) in it {
            if n == 0 {
                continue;
            }

            let factor = 1. / n as f32;
            *t *= factor;
            *b *= factor;
        }

        mesh.tangents = tangents;
        mesh.bitangents = bitangents;

        let mesh_pointer = Arc::new(mesh);
        let hittables = triangles
            .into_iter()
            .map(|triangle| {
                Box::new(MeshTangentTriangle {
                    triangle,
                    mesh: mesh_pointer.clone(),
                }) as Box<dyn Hittable>
            })
            .collect();
        return Ok(hittables);
    }

    let mesh_pointer = Arc::new(mesh);
    let hittables = triangles
        .into_iter()
        .map(|triangle| {
            Box::new(MeshTriangle {
                triangle,
                mesh: mesh_pointer.clone(),
            }) as Box<dyn Hittable>
        })
        .collect();
    Ok(hittables)
}
