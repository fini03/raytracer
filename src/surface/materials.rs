use std::{sync::Arc, path::PathBuf};
use serde::{Deserialize, Deserializer, de};
use crate::utils::helpers::parse_color;
use crate::math::Color;
use crate::ray::{Ray, HitRecord};

pub trait MaterialParameters {
    fn phong(&self) -> &Phong;

    fn reflectance(&self) -> f32;

    fn transmittance(&self) -> f32;

    fn refraction(&self) -> f32;
}

pub trait ColorLookup: Send + Sync {
    fn color(&self, ray: &Ray, hit: &HitRecord) -> Color;
}

pub trait Material:
    MaterialParameters + ColorLookup + Send + Sync
{
}
impl<T: MaterialParameters + ColorLookup> Material for T {}

#[derive(Deserialize)]
pub struct Solid {
    #[serde(deserialize_with = "parse_color")]
    pub color: Color,
    pub phong: Phong,
    pub reflectance: Reflectance,
    pub transmittance: Transmittance,
    pub refraction: Refraction,
}

impl MaterialParameters for Solid {
    fn phong(&self) -> &Phong {
        &self.phong
    }

    fn reflectance(&self) -> f32 {
        self.reflectance.r
    }

    fn transmittance(&self) -> f32 {
        self.transmittance.t
    }

    fn refraction(&self) -> f32 {
        self.refraction.iof
    }
}

impl ColorLookup for Solid {
    fn color(&self, _ray: &Ray, _hit: &HitRecord) -> Color {
        self.color.clone()
    }
}

pub struct Textured {
    pub texture: Box<dyn ColorLookup>,
    pub phong: Phong,
    pub reflectance: Reflectance,
    pub transmittance: Transmittance,
    pub refraction: Refraction,
}

impl MaterialParameters for Textured {
    fn phong(&self) -> &Phong {
        &self.phong
    }

    fn reflectance(&self) -> f32 {
        self.reflectance.r
    }

    fn transmittance(&self) -> f32 {
        self.transmittance.t
    }

    fn refraction(&self) -> f32 {
        self.refraction.iof
    }
}

impl ColorLookup for Textured {
    fn color(&self, ray: &Ray, hit: &HitRecord) -> Color {
        self.texture.color(ray, hit)
    }
}

pub struct TextureNearest {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl ColorLookup for TextureNearest {
    fn color(&self, _ray: &Ray, hit: &HitRecord) -> Color {
        let tex_coords = &hit.tex_coords;

        let w_int = self.width as isize;
        let h_int = self.height as isize;

        let u = ((tex_coords.x * w_int as f32).floor() as isize)
            .rem_euclid(w_int);
        let v = ((tex_coords.y * h_int as f32).floor() as isize)
            .rem_euclid(h_int);

        self.pixels[(v * w_int + u) as usize].clone()
    }
}

pub struct TextureLinear {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl ColorLookup for TextureLinear {
    fn color(&self, _ray: &Ray, hit: &HitRecord) -> Color {
        let tex_coords = &hit.tex_coords;
        let w_int = self.width as isize;
        let h_int = self.height as isize;

        // Subtracting half a pixel from the coordinates, so that the
        // textures line up with the output produced by nearest
        // neighbour, otherwise it's one pixel off when compared to
        // nearest
        let u = tex_coords.x * w_int as f32 - 0.5;
        let v = tex_coords.y * h_int as f32 - 0.5;
        let s = u.fract();
        let t = v.fract();

        let c0_u = (u.floor() as isize).rem_euclid(w_int);
        let c0_v = (v.floor() as isize).rem_euclid(h_int);
        let c0 = &self.pixels[(c0_v * w_int + c0_u) as usize];

        let c1_u = (u.ceil() as isize).rem_euclid(w_int);
        let c1_v = (v.floor() as isize).rem_euclid(h_int);
        let c1 = &self.pixels[(c1_v * w_int + c1_u) as usize];

        let c2_u = (u.floor() as isize).rem_euclid(w_int);
        let c2_v = (v.ceil() as isize).rem_euclid(h_int);
        let c2 = &self.pixels[(c2_v * w_int + c2_u) as usize];

        let c3_u = (u.ceil() as isize).rem_euclid(w_int);
        let c3_v = (v.ceil() as isize).rem_euclid(h_int);
        let c3 = &self.pixels[(c3_v * w_int + c3_u) as usize];

        let i_0 = (1. - s) * c0 + s * c1;
        let i_1 = (1. - s) * c2 + s * c3;

        (1. - t) * i_0 + t * i_1
    }
}

pub struct TextureSphere {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl ColorLookup for TextureSphere {
    fn color(&self, ray: &Ray, hit: &HitRecord) -> Color {
        let w_int = self.width as isize;
        let h_int = self.height as isize;

        // Calculate the sphere mapping
        let v = -ray.dir.unit_vector();
        let n = &hit.normal;
        let r = v.reflect(n);
        let r_scale = 1.
            / (r.x * r.x + r.y * r.y + (r.z + 1.) * (r.z + 1.)).sqrt();
        let s_s = (r.x * r_scale + 1.) * 0.5;
        let s_t = (r.y * r_scale + 1.) * 0.5;

        // Subtracting half a pixel from the coordinates, so that the
        // textures line up with the output produced by nearest
        // neighbour, otherwise it's one pixel off when compared to
        // nearest
        let u = s_s * w_int as f32 - 0.5;
        let v = s_t * h_int as f32 - 0.5;
        let s = u.fract();
        let t = v.fract();

        let c0_u = (u.floor() as isize).rem_euclid(w_int);
        let c0_v = (v.floor() as isize).rem_euclid(h_int);
        let c0 = &self.pixels[(c0_v * w_int + c0_u) as usize];

        let c1_u = (u.ceil() as isize).rem_euclid(w_int);
        let c1_v = (v.floor() as isize).rem_euclid(h_int);
        let c1 = &self.pixels[(c1_v * w_int + c1_u) as usize];

        let c2_u = (u.floor() as isize).rem_euclid(w_int);
        let c2_v = (v.ceil() as isize).rem_euclid(h_int);
        let c2 = &self.pixels[(c2_v * w_int + c2_u) as usize];

        let c3_u = (u.ceil() as isize).rem_euclid(w_int);
        let c3_v = (v.ceil() as isize).rem_euclid(h_int);
        let c3 = &self.pixels[(c3_v * w_int + c3_u) as usize];

        let i_0 = (1. - s) * c0 + s * c1;
        let i_1 = (1. - s) * c2 + s * c3;

        (1. - t) * i_0 + t * i_1
    }
}

#[derive(Deserialize, Clone)]
pub struct Phong {
    #[serde(rename = "@ka")]
    pub ka: f32,
    #[serde(rename = "@kd")]
    pub kd: f32,
    #[serde(rename = "@ks")]
    pub ks: f32,
    #[serde(rename = "@exponent")]
    pub exponent: f32,
}

#[derive(Deserialize)]
pub struct Reflectance {
    #[serde(rename = "@r")]
    pub r: f32,
}

#[derive(Deserialize)]
pub struct Transmittance {
    #[serde(rename = "@t")]
    pub t: f32,
}

#[derive(Deserialize)]
pub struct Refraction {
    #[serde(rename = "@iof")]
    pub iof: f32,
}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

pub fn parse_material<'de, D>(
    deserializer: D,
) -> Result<Arc<dyn Material>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    enum BaseMaterial {
        #[serde(rename = "material_solid")]
        BaseSolid(Solid),
        #[serde(rename = "material_textured")]
        #[serde(deserialize_with = "parse_material_textured")]
        BaseTextured(Textured),
        #[serde(rename = "material_spheremap")]
        #[serde(deserialize_with = "parse_material_spheremap")]
        BaseSphereMap(Textured),
    }
    use BaseMaterial::*;

    let m = BaseMaterial::deserialize(deserializer)?;
    Ok(match m {
        BaseSolid(m) => Arc::new(m),
        BaseTextured(m) => Arc::new(m),
        BaseSphereMap(m) => Arc::new(m),
    })
}

pub fn parse_material_textured<'de, D>(
    deserializer: D,
) -> Result<Textured, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    pub struct BaseTextured {
        #[serde(deserialize_with = "parse_texture")]
        pub texture: Texture,
        pub phong: Phong,
        pub reflectance: Reflectance,
        pub transmittance: Transmittance,
        pub refraction: Refraction,
    }

    let BaseTextured {
        texture,
        phong,
        reflectance,
        transmittance,
        refraction,
    } = BaseTextured::deserialize(deserializer)?;

    use crate::utils::config::TextureInterpolation::*;
    let config = crate::CONFIG.get().unwrap();
    let t: Box<dyn ColorLookup> = match config.texture_interpolation {
        Nearest => {
            let Texture {
                width,
                height,
                pixels,
            } = texture;
            Box::new(TextureNearest {
                width,
                height,
                pixels,
            })
        }
        Linear => {
            let Texture {
                width,
                height,
                pixels,
            } = texture;
            Box::new(TextureLinear {
                width,
                height,
                pixels,
            })
        }
    };

    Ok(Textured {
        texture: t,
        phong,
        reflectance,
        transmittance,
        refraction,
    })
}

pub fn parse_texture_object<'de, D>(
    deserializer: D,
) -> Result<Option<Box<dyn ColorLookup>>, D::Error>
where
    D: Deserializer<'de>,
{
    use crate::io::load_texture;

    #[derive(Deserialize)]
    pub struct BaseTexture {
        #[serde(rename = "@name")]
        pub name: String,
    }

    let name;
    let t = Option::<BaseTexture>::deserialize(deserializer)?;
    match t {
        Some(t) => name = t.name,
        None => return Ok(None),
    }

    let mut path = PathBuf::new();
    path.push(r"../scenes");
    path.push(&name);

    let Texture {
        width,
        height,
        pixels,
    } = load_texture(&path)
        .map_err(|e| de::Error::custom(e.to_string()))?;

    use crate::utils::config::TextureInterpolation::*;
    let config = crate::CONFIG.get().unwrap();
    match config.texture_interpolation {
        Nearest => Ok(Some(Box::new(TextureNearest {
            width,
            height,
            pixels,
        }))),
        Linear => Ok(Some(Box::new(TextureLinear {
            width,
            height,
            pixels,
        }))),
    }
}

pub fn parse_material_spheremap<'de, D>(
    deserializer: D,
) -> Result<Textured, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    pub struct BaseTextured {
        #[serde(deserialize_with = "parse_texture")]
        pub texture: Texture,
        pub phong: Phong,
        pub reflectance: Reflectance,
        pub transmittance: Transmittance,
        pub refraction: Refraction,
    }

    let BaseTextured {
        texture,
        phong,
        reflectance,
        transmittance,
        refraction,
    } = BaseTextured::deserialize(deserializer)?;
    let Texture {
        width,
        height,
        pixels,
    } = texture;
    let t: Box<dyn ColorLookup> = Box::new(TextureSphere {
        width,
        height,
        pixels,
    });
    Ok(Textured {
        texture: t,
        phong,
        reflectance,
        transmittance,
        refraction,
    })
}

pub fn parse_texture<'de, D>(
    deserializer: D,
) -> Result<Texture, D::Error>
where
    D: Deserializer<'de>,
{
    use crate::io::load_texture;

    #[derive(Deserialize)]
    pub struct BaseTexture {
        #[serde(rename = "@name")]
        pub name: String,
    }

    let t = BaseTexture::deserialize(deserializer)?;
    let mut path = PathBuf::new();
    path.push(r"../scenes");
    path.push(&t.name);
    let texture = load_texture(&path)
        .map_err(|e| de::Error::custom(e.to_string()))?;

    Ok(texture)
}
