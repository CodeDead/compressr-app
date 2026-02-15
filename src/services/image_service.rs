use image::{ExtendedColorType, GenericImageView, ImageFormat};
use resvg::usvg;
use std::fs;
use std::fs::File;
use tiny_skia::Pixmap;

#[derive(Default)]
pub struct ImageService;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Jpeg,
    Png,
    Gif,
    WebP,
}

impl OutputFormat {
    pub const ALL: [OutputFormat; 4] = [
        OutputFormat::Jpeg,
        OutputFormat::Png,
        OutputFormat::Gif,
        OutputFormat::WebP,
    ];
}

impl ImageService {
    /// Compresses an image with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `input_path` - The path to the input image.
    /// * `output_path` - The path to the output image.
    /// * `scale` - The scale factor to apply to the image (in percentage).
    /// * `width` - The desired width of the output image (optional).
    /// * `height` - The desired height of the output image (optional).
    /// * `quality` - The quality of the output image (0-100).
    /// * `format` - The output format of the image.
    ///
    /// # Errors
    ///
    /// This function will return an error if the input or output paths are empty, if the input image cannot be loaded, or if the output image cannot be saved.
    #[allow(clippy::too_many_arguments)]
    pub fn compress_image(
        &self,
        input_path: &str,
        output_path: &str,
        scale: u32,
        width: Option<u32>,
        height: Option<u32>,
        quality: u8,
        format: OutputFormat,
    ) -> anyhow::Result<()> {
        if input_path.is_empty() || output_path.is_empty() {
            anyhow::bail!("Input or output missing");
        }

        let mut img = if input_path.ends_with(".svg") {
            ImageService::load_svg(input_path)?
        } else {
            image::open(input_path)?
        };

        // Scale
        let (w, h) = img.dimensions();
        img = img.resize(
            w * scale / 100,
            h * scale / 100,
            image::imageops::FilterType::Lanczos3,
        );

        // Resize
        if let Some(w) = width
            && let Some(h) = height
        {
            img = img.resize(w, h, image::imageops::FilterType::Lanczos3);
        }

        let mut output = File::create(output_path)?;

        match format {
            OutputFormat::Jpeg => {
                let mut encoder =
                    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, quality);
                encoder.encode_image(&img)?;
            }
            OutputFormat::WebP => {
                let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut output);
                encoder.encode(
                    &img.to_rgba8(),
                    img.width(),
                    img.height(),
                    ExtendedColorType::Rgba8,
                )?;
            }
            OutputFormat::Png => img.write_to(&mut output, ImageFormat::Png)?,
            OutputFormat::Gif => img.write_to(&mut output, ImageFormat::Gif)?,
        }

        Ok(())
    }

    /// Loads an SVG file and converts it to a `DynamicImage`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the SVG file.
    ///
    /// # Errors
    ///
    /// This function will return an error if the SVG file cannot be read, if the SVG cannot be parsed, or if the SVG cannot be rendered.
    pub fn load_svg(path: &str) -> anyhow::Result<image::DynamicImage> {
        let data = fs::read(path)?;

        // Parse options
        let mut opt = usvg::Options {
            // Get file's absolute directory.
            resources_dir: fs::canonicalize(path)
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf())),
            ..usvg::Options::default()
        };
        opt.fontdb_mut().load_system_fonts();

        let tree = usvg::Tree::from_data(&data, &opt)?;

        // Create pixmap
        let size = tree.size().to_int_size();
        let mut pixmap = Pixmap::new(size.width(), size.height())
            .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;

        // Render SVG to pixmap
        resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

        // Convert to image::DynamicImage
        let img = image::RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
            .ok_or_else(|| anyhow::anyhow!("Failed to convert SVG"))?;

        Ok(image::DynamicImage::ImageRgba8(img))
    }
}
