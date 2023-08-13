use std::error::Error;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufWriter;
use crate::scene::Scene;

pub struct SceneWriter {
    image_writer: png::Writer<BufWriter<File>>,
}

impl SceneWriter {
    pub fn new(
        scene: &Scene,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let encoder = Self::initialize_encoder(scene)?;
        Ok(Self {
            image_writer: encoder.write_header()?,
        })
    }

    pub fn new_animated(
        scene: &Scene,
        duration: u32,
        frames_per_second: u16,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut encoder = Self::initialize_encoder(scene)?;

        // Let it loop indefinitely
        let num_frames = duration * frames_per_second as u32;
        encoder.set_animated(num_frames, 0)?;
        encoder.set_frame_delay(1, frames_per_second)?;

        Ok(Self {
            image_writer: encoder.write_header()?,
        })
    }

    fn initialize_encoder(
        scene: &Scene,
    ) -> Result<
        png::Encoder<BufWriter<File>>,
        Box<dyn Error + Send + Sync>,
    > {
        let mut path = PathBuf::new();
        path.push(r"../output");
        path.push(&scene.output_file);

        let output_file = File::create(&path)?;
        let w = BufWriter::new(output_file);

        let mut encoder = png::Encoder::new(
            w,
            scene.camera.image_width as u32,
            scene.camera.image_height as u32,
        );
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        // Set gamma to compare with GFX page
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
        let source_chromaticities = png::SourceChromaticities::new(
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);

        Ok(encoder)
    }

    pub fn write_image_data(
        &mut self,
        data: &[u8],
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        self.image_writer.write_image_data(data)?;
        Ok(())
    }
}
