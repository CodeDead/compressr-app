use image::{ExtendedColorType, GenericImageView, ImageFormat};
use resvg::usvg;
use std::fs;
use std::fs::File;
use std::path::Path;
use tiny_skia::Pixmap;

#[derive(Default, Clone)]
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
    ) -> Result<(), String> {
        if input_path.is_empty() || output_path.is_empty() {
            return Err("Input and output paths cannot be empty".to_string());
        }

        // Check if input path is a directory
        let mut files = Vec::new();
        if fs::metadata(input_path)
            .map_err(|e| format!("Failed to read input path metadata: {e}"))?
            .is_dir()
        {
            files = match self.list_files_recursively(Path::new(input_path)) {
                Ok(files) => files,
                Err(e) => {
                    return Err(format!("Failed to list files in directory: {e}"));
                }
            };
        } else {
            files.push(input_path.to_string());
        }

        let is_output_a_directory = fs::metadata(output_path)
            .map_err(|e| format!("Failed to read output path metadata: {e}"))?
            .is_dir();

        for file in files {
            let mut img = if file.ends_with(".svg") {
                match ImageService::load_svg(&file) {
                    Ok(img) => img,
                    Err(e) => {
                        return Err(format!("Failed to load SVG: {e}"));
                    }
                }
            } else {
                match image::open(&file) {
                    Ok(img) => img,
                    Err(e) => {
                        return Err(format!("Failed to load image: {e}"));
                    }
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
            }
        }

        Ok(())
    }

    /// Recursively lists all files in a directory.
    ///
    /// # Arguments
    ///
    /// * `dir` - The path to the directory.
    ///
    /// # Errors
    ///
    /// This function will return an error if the directory cannot be read.
    ///
    /// # Returns
    ///
    /// A vector of file paths as strings.
    fn list_files_recursively(&self, dir: &Path) -> std::io::Result<Vec<String>> {
        let mut files = Vec::new();
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    files.extend(self.list_files_recursively(&path)?);
                } else {
                    files.push(path.to_string_lossy().into_owned());
                }
            }
        }
        Ok(files)
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
