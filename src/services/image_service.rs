use image::{ExtendedColorType, GenericImageView, ImageEncoder, ImageFormat};
use img_parts::ImageEXIF;
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
    /// * `preserve_exif` - Whether to preserve EXIF metadata from the source image.
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
        preserve_exif: bool,
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

        for file in input_path.clone() {
            // Extract EXIF bytes from the source file before re-encoding strips them.
            // Try each supported container format in order; most source images are JPEG.
            let source_exif: Option<img_parts::Bytes> = if preserve_exif {
                fs::read(&file).ok().and_then(|raw| {
                    let b: img_parts::Bytes = raw.into();
                    img_parts::jpeg::Jpeg::from_bytes(b.clone())
                        .ok()
                        .and_then(|j| j.exif())
                        .or_else(|| {
                            img_parts::png::Png::from_bytes(b.clone())
                                .ok()
                                .and_then(|p| p.exif())
                        })
                        .or_else(|| {
                            img_parts::webp::WebP::from_bytes(b)
                                .ok()
                                .and_then(|w| w.exif())
                        })
                })
            } else {
                None
            };

            let mut img = match image::open(&file) {
                Ok(img) => img,
                Err(e) => {
                    return Err(format!("Failed to load image: {e}"));
                }
            };

            // Scale
            if width.is_none() && height.is_none() && scale < 100 {
                let (w, h) = img.dimensions();
                img = img.resize(
                    w * scale / 100,
                    h * scale / 100,
                    image::imageops::FilterType::Lanczos3,
                );
            }

            // Resize
            if let Some(w) = width
                && let Some(h) = height
            {
                img = img.resize(w, h, image::imageops::FilterType::Lanczos3);
            }

            let output_path = if is_output_a_directory {
                let file_name_with_extension = Path::new(&file)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("output");

                let file_name_without_path_and_extension =
                    format!("{}_compressed", file_name_with_extension);

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

            let mut output = match File::create(&output_path) {
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
            // Drop the file handle before re-reading the file for EXIF injection.
            drop(output);

            // Re-inject EXIF bytes into the output file for supported formats.
            if let Some(exif) = source_exif {
                let raw = fs::read(&output_path)
                    .map_err(|e| format!("Failed to read output for EXIF injection: {e}"))?;
                let b: img_parts::Bytes = raw.into();

                let injected: Option<Vec<u8>> = match format {
                    OutputFormat::Jpeg => {
                        let mut jpeg = img_parts::jpeg::Jpeg::from_bytes(b)
                            .map_err(|e| format!("Failed to parse output JPEG: {e}"))?;
                        jpeg.set_exif(Some(exif));
                        let mut buf = Vec::new();
                        jpeg.encoder()
                            .write_to(&mut buf)
                            .map_err(|e| format!("Failed to write EXIF to JPEG: {e}"))?;
                        Some(buf)
                    }
                    OutputFormat::Png => {
                        let mut png = img_parts::png::Png::from_bytes(b)
                            .map_err(|e| format!("Failed to parse output PNG: {e}"))?;
                        png.set_exif(Some(exif));
                        let mut buf = Vec::new();
                        png.encoder()
                            .write_to(&mut buf)
                            .map_err(|e| format!("Failed to write EXIF to PNG: {e}"))?;
                        Some(buf)
                    }
                    OutputFormat::WebP => {
                        let mut webp = img_parts::webp::WebP::from_bytes(b)
                            .map_err(|e| format!("Failed to parse output WebP: {e}"))?;
                        webp.set_exif(Some(exif));
                        let mut buf = Vec::new();
                        webp.encoder()
                            .write_to(&mut buf)
                            .map_err(|e| format!("Failed to write EXIF to WebP: {e}"))?;
                        Some(buf)
                    }
                    // GIF, BMP, and TIFF do not support EXIF via img-parts.
                    _ => None,
                };

                if let Some(buf) = injected {
                    fs::write(&output_path, buf)
                        .map_err(|e| format!("Failed to save EXIF-injected output: {e}"))?;
                }
            }
        }

        // Delete original files after compression
        if delete_original {
            for file in input_path {
                if let Err(e) = fs::remove_file(&file) {
                    return Err(format!("Failed to delete original file: {e}"));
                }
            }
        }

        Ok(())
    }
}
