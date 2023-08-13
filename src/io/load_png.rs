use std::path::Path;
use std::fs::File;
use crate::surface::materials::Texture;
use crate::math::Color;
use std::error::Error;

pub fn load_texture(
    path: &Path,
) -> Result<Texture, Box<dyn Error + Send + Sync>> {
    let file = File::open(path)?;
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;

    let width = info.width as u32;
    let height = info.height as u32;
    let pixels: Vec<_> = buf
        .chunks_exact(3)
        .map(|pixel| Color::from_values(
            pixel[0] as f32 / 255.,
            pixel[1] as f32 / 255.,
            pixel[2] as f32 / 255.
        ))
        .collect();

    Ok(Texture {
        width,
        height,
        pixels
    })
}
