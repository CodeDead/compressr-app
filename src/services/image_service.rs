use image::{ExtendedColorType, GenericImageView, ImageEncoder, ImageFormat};
use std::fs;
use std::fs::File;
use std::path::Path;

#[derive(Default, Clone)]
pub struct ImageService;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Jpeg,
    Png,
    Gif,
    WebP,
    Bmp,
    Tiff,
}

impl OutputFormat {
    pub const ALL: [OutputFormat; 6] = [
        OutputFormat::Jpeg,
        OutputFormat::Png,
        OutputFormat::Gif,
        OutputFormat::WebP,
        OutputFormat::Bmp,
        OutputFormat::Tiff,
    ];
}

impl ImageService {
    /// Initialize a new ImageService
    ///
    /// # Returns
    ///
    /// A new instance of ImageService
    pub fn new() -> Self {
        ImageService
    }

    /// Compresses an image with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `input_path` - The path(s) to the input image(s).
    /// * `output_path` - The path to the output image.
    /// * `scale` - The scale factor to apply to the image (in percentage).
    /// * `width` - The desired width of the output image (optional).
    /// * `height` - The desired height of the output image (optional).
    /// * `quality` - The quality of the output image (0-100).
    /// * `format` - The output format of the image.
    /// * `delete_original` - Whether to delete the original image after compression.
    ///
    /// # Errors
    ///
    /// This function will return an error if the input or output paths are empty, if the input image cannot be loaded, or if the output image cannot be saved.
    #[allow(clippy::too_many_arguments)]
    pub fn compress_image(
        &self,
        input_path: Vec<String>,
        output_path: &str,
        scale: u32,
        width: Option<u32>,
        height: Option<u32>,
        quality: u8,
        format: OutputFormat,
        delete_original: bool,
    ) -> Result<(), String> {
        if input_path.is_empty() {
            return Err("Input path cannot be empty".to_string());
        }

        if output_path.is_empty() {
            return Err("Output path cannot be empty".to_string());
        }

        let mut is_output_a_directory = false;

        // Check if output path already exists
        if fs::metadata(output_path).is_ok() {
            is_output_a_directory = fs::metadata(output_path)
                .map_err(|e| format!("Failed to read output path metadata: {e}"))?
                .is_dir();
        }

        for file in input_path {
            let mut img = match image::open(&file) {
                Ok(img) => img,
                Err(e) => {
                    return Err(format!("Failed to load image: {e}"));
                }
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

            let output_path = if is_output_a_directory {
                let file_name_without_path_and_extension = Path::new(&file)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output");

                let file_name_without_path_and_extension =
                    format!("{}_compressed", file_name_without_path_and_extension);

                let extension = match format {
                    OutputFormat::Jpeg => "jpg",
                    OutputFormat::Png => "png",
                    OutputFormat::Gif => "gif",
                    OutputFormat::WebP => "webp",
                    OutputFormat::Bmp => "bmp",
                    OutputFormat::Tiff => "tiff",
                };

                format!(
                    "{}/{}.{}",
                    output_path, file_name_without_path_and_extension, extension
                )
            } else {
                output_path.to_string()
            };

            let mut output = match File::create(output_path) {
                Ok(f) => f,
                Err(e) => {
                    return Err(format!("Failed to create output file: {e}"));
                }
            };

            match format {
                OutputFormat::Jpeg => {
                    let mut encoder =
                        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, quality);
                    match encoder.encode_image(&img) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(format!("Failed to encode JPEG: {e}"));
                        }
                    };
                }
                OutputFormat::WebP => {
                    let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut output);
                    match encoder.encode(
                        &img.to_rgba8(),
                        img.width(),
                        img.height(),
                        ExtendedColorType::Rgba8,
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(format!("Failed to encode WebP: {e}"));
                        }
                    };
                }
                OutputFormat::Png => {
                    match img.write_to(&mut output, ImageFormat::Png) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(format!("Failed to encode PNG: {e}"));
                        }
                    };
                }
                OutputFormat::Gif => {
                    match img.write_to(&mut output, ImageFormat::Gif) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(format!("Failed to encode GIF: {e}"));
                        }
                    };
                }
                OutputFormat::Bmp => {
                    let mut encoder = image::codecs::bmp::BmpEncoder::new(&mut output);
                    match encoder.encode(
                        &img.to_rgba8(),
                        img.width(),
                        img.height(),
                        ExtendedColorType::Rgba8,
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(format!("Failed to encode BMP: {e}"));
                        }
                    };
                }
                OutputFormat::Tiff => {
                    let encoder = image::codecs::tiff::TiffEncoder::new(&mut output);
                    match encoder.write_image(
                        &img.to_rgba8(),
                        img.width(),
                        img.height(),
                        ExtendedColorType::Rgba8,
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(format!("Failed to write TIFF header: {e}"));
                        }
                    };
                }
            }

            if delete_original && let Err(e) = fs::remove_file(&file) {
                return Err(format!("Failed to delete original file: {e}"));
            }
        }

        Ok(())
    }
}
