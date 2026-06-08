use image::{DynamicImage, ExtendedColorType, GenericImageView, ImageEncoder, ImageFormat};
use img_parts::ImageEXIF;
use std::borrow::Cow;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone)]
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
        Self::Jpeg,
        Self::Png,
        Self::Gif,
        Self::WebP,
        Self::Bmp,
        Self::Tiff,
    ];

    /// Returns the canonical file extension for this format.
    ///
    /// # Returns
    ///
    /// A static string slice representing the file extension for the given format.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Gif => "gif",
            Self::WebP => "webp",
            Self::Bmp => "bmp",
            Self::Tiff => "tiff",
        }
    }
}

impl std::fmt::Display for OutputFormat {
    /// Formats the `OutputFormat` enum as a human-readable string for display purposes.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a `Formatter` where the formatted string will be written.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the formatting was successful or if an error occurred.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Jpeg => write!(f, "JPEG"),
            OutputFormat::Png => write!(f, "PNG"),
            OutputFormat::Gif => write!(f, "GIF"),
            OutputFormat::WebP => write!(f, "WebP"),
            OutputFormat::Bmp => write!(f, "BMP"),
            OutputFormat::Tiff => write!(f, "Tiff"),
        }
    }
}

/// Result of a single image compression operation.
#[derive(Debug, Clone)]
pub struct CompressionResult {
    /// The original file name (without directory).
    pub file_name: String,
    /// Original file size in bytes.
    pub original_size: u64,
    /// Compressed file size in bytes.
    pub compressed_size: u64,
}

impl CompressionResult {
    /// Returns the percentage of size changed.
    ///
    /// # Returns
    ///
    /// A floating-point number representing the percentage of size saved compared to the original size.
    pub fn percent_saved(&self) -> f64 {
        if self.original_size == 0 {
            return 0.0;
        }
        let diff = self.original_size as f64 - self.compressed_size as f64;
        (diff / self.original_size as f64) * 100.0
    }
}

/// Parameters for a single image compression operation.
#[derive(Debug, Clone)]
pub struct CompressionParams {
    /// Base output path (directory or file). Shared across batch items via `Arc`.
    pub output_path: Arc<str>,
    /// Whether `output_path` refers to a directory.
    pub is_output_a_directory: bool,
    /// Scale factor to apply in percentage (values < 100 activate scaling).
    pub scale: u32,
    /// Desired output width in pixels (optional).
    pub width: Option<u32>,
    /// Desired output height in pixels (optional).
    pub height: Option<u32>,
    /// JPEG/WEBP Image quality (0–100); ignored for other formats.
    pub quality: u8,
    /// Output container format.
    pub format: OutputFormat,
    /// Whether to copy EXIF metadata from the source file.
    pub preserve_exif: bool,
    /// When `Some`, overrides the path that `resolve_output_path` would normally derive.
    /// Used by the caller to pass a pre-deduplicated output path.
    pub output_path_override: Option<String>,
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

    /// Compresses a single image file using the provided parameters.
    ///
    /// # Errors
    ///
    /// Returns an error string if the image cannot be loaded, encoded, or saved.
    pub fn compress_single(
        &self,
        file: String,
        params: &CompressionParams,
        cancelled: Arc<AtomicBool>,
    ) -> Result<CompressionResult, String> {
        if cancelled.load(Ordering::Relaxed) {
            return Err("COMPRESSION_ABORTED".to_string());
        }

        // Validate explicit dimensions before doing any I/O.
        if let Some(w) = params.width
            && w == 0
        {
            return Err("Width cannot be equal to 0".to_string());
        }
        if let Some(h) = params.height
            && h == 0
        {
            return Err("Height cannot be equal to 0".to_string());
        }

        let raw = fs::read(&file).map_err(|e| format!("Failed to read '{file}': {e}"))?;

        if cancelled.load(Ordering::Relaxed) {
            return Err("COMPRESSION_ABORTED".to_string());
        }

        let original_size = raw.len() as u64;

        let source_exif = if params.preserve_exif {
            self.read_exif(&raw)
        } else {
            None
        };
        let img = image::load_from_memory(&raw)
            .map_err(|e| format!("Failed to load image '{file}': {e}"))?;

        if cancelled.load(Ordering::Relaxed) {
            return Err("COMPRESSION_ABORTED".to_string());
        }

        let img = self.apply_geometry(img, params);

        // Encode to an in-memory buffer — no intermediate file write needed.
        let encoded = self.encode(&img, params)?;

        if cancelled.load(Ordering::Relaxed) {
            return Err("COMPRESSION_ABORTED".to_string());
        }

        // Optionally, inject EXIF into the in-memory buffer before the single
        // disk write, eliminating the previous read-back-from-disk round-trip.
        let final_bytes = if let Some(exif) = source_exif {
            self.inject_exif(encoded, exif, params.format)?
        } else {
            encoded
        };

        let compressed_size = final_bytes.len() as u64;

        let output_path = params
            .output_path_override
            .clone()
            .unwrap_or_else(|| self.resolve_output_path(&file, params));

        if cancelled.load(Ordering::Relaxed) {
            return Err("COMPRESSION_ABORTED".to_string());
        }

        fs::write(&output_path, final_bytes)
            .map_err(|e| format!("Failed to write output file: {e}"))?;

        let file_name = Path::new(&file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&file)
            .to_string();

        Ok(CompressionResult {
            file_name,
            original_size,
            compressed_size,
        })
    }

    /// Reads EXIF bytes from the raw file bytes.
    ///
    /// Uses `image::guess_format` (magic bytes) to detect the image format
    /// first, so the buffer is cloned at most once and only the correct parser
    /// is invoked.
    ///
    /// # Arguments
    ///
    /// * `bytes`: The raw file bytes of the source image.
    ///
    /// # Returns
    ///
    /// An `Option` containing the EXIF bytes if successfully read and preserved, or `None` otherwise.
    fn read_exif(&self, bytes: &[u8]) -> Option<img_parts::Bytes> {
        let format = image::guess_format(bytes).ok()?;
        match format {
            ImageFormat::Jpeg => {
                let b: img_parts::Bytes = bytes.to_vec().into();
                img_parts::jpeg::Jpeg::from_bytes(b)
                    .ok()
                    .and_then(|j| j.exif())
            }
            ImageFormat::Png => {
                let b: img_parts::Bytes = bytes.to_vec().into();
                img_parts::png::Png::from_bytes(b)
                    .ok()
                    .and_then(|p| p.exif())
            }
            ImageFormat::WebP => {
                let b: img_parts::Bytes = bytes.to_vec().into();
                img_parts::webp::WebP::from_bytes(b)
                    .ok()
                    .and_then(|w| w.exif())
            }
            _ => None,
        }
    }

    /// Applies scale and/or explicit dimensions to the image.
    ///
    /// # Arguments
    ///
    /// * `img`: The original image to be transformed.
    /// * `params`: The compression parameters containing scaling and dimension info.
    ///
    /// # Returns
    ///
    /// The transformed image with applied geometry.
    fn apply_geometry(&self, mut img: DynamicImage, params: &CompressionParams) -> DynamicImage {
        // Scale takes effect only when no explicit dimensions are set.
        if params.width.is_none() && params.height.is_none() && params.scale < 100 {
            let (w, h) = img.dimensions();
            let new_w = ((w as u64 * params.scale as u64) / 100).max(1) as u32;
            let new_h = ((h as u64 * params.scale as u64) / 100).max(1) as u32;
            img = img.resize(new_w, new_h, image::imageops::FilterType::Lanczos3);
        }

        match (params.width, params.height) {
            (Some(w), Some(h)) => img.resize(w, h, image::imageops::FilterType::Lanczos3),
            (Some(w), None) => {
                let (ow, oh) = img.dimensions();
                let h = ((oh as f32 * (w as f32 / ow as f32)).round() as u32).max(1);
                img.resize(w, h, image::imageops::FilterType::Lanczos3)
            }
            (None, Some(h)) => {
                let (ow, oh) = img.dimensions();
                let w = ((ow as f32 * (h as f32 / oh as f32)).round() as u32).max(1);
                img.resize(w, h, image::imageops::FilterType::Lanczos3)
            }
            _ => img,
        }
    }

    /// Extracts raw pixel data and color type without unnecessary cloning.
    ///
    /// # Arguments
    ///
    /// * `img`: The `DynamicImage` from which to extract pixel data.
    ///
    /// # Returns
    ///
    /// A tuple containing a `Cow<'a, [u8]>` with the raw pixel data and an `ExtendedColorType` representing the color type.
    fn pixel_data_slice<'a>(&self, img: &'a DynamicImage) -> (Cow<'a, [u8]>, ExtendedColorType) {
        match img {
            DynamicImage::ImageLuma8(buf) => (Cow::Borrowed(buf.as_raw()), ExtendedColorType::L8),
            DynamicImage::ImageLumaA8(buf) => (Cow::Borrowed(buf.as_raw()), ExtendedColorType::La8),
            DynamicImage::ImageRgb8(buf) => (Cow::Borrowed(buf.as_raw()), ExtendedColorType::Rgb8),
            DynamicImage::ImageRgba8(buf) => {
                (Cow::Borrowed(buf.as_raw()), ExtendedColorType::Rgba8)
            }
            _ => {
                let rgba = img.to_rgba8();
                (Cow::Owned(rgba.into_raw()), ExtendedColorType::Rgba8)
            }
        }
    }

    /// Resolves the final output file path from the source file path and params.
    ///
    /// # Arguments
    ///
    /// * `file`: The original source file path.
    /// * `params`: The compression parameters containing output path info.
    ///
    /// # Returns
    ///
    /// A string representing the resolved output file path.
    pub fn resolve_output_path(&self, file: &str, params: &CompressionParams) -> String {
        if params.is_output_a_directory {
            let file_stem = Path::new(file)
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("output");

            let stem = format!("{}_compressed", file_stem);

            Path::new(&*params.output_path)
                .join(format!("{}.{}", stem, params.format.extension()))
                .to_string_lossy()
                .into_owned()
        } else {
            params.output_path.to_string()
        }
    }

    /// Resolves output paths for a batch of input files, disambiguating any collisions.
    ///
    /// Each input is mapped through [`resolve_output_path`](Self::resolve_output_path). When two
    /// inputs would resolve to the same output path, subsequent paths gain a numeric suffix
    /// (`name_2.ext`, `name_3.ext`, ...) so no file is silently overwritten by another item in
    /// the same batch.
    ///
    /// # Arguments
    ///
    /// * `files`: The input file paths to resolve.
    /// * `params`: The compression parameters used to derive each output path.
    ///
    /// # Returns
    ///
    /// A vector of resolved, collision-free output paths, parallel to `files`.
    pub fn resolve_unique_output_paths(
        &self,
        files: &[String],
        params: &CompressionParams,
    ) -> Vec<String> {
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        files
            .iter()
            .map(|file| {
                let candidate = self.resolve_output_path(file, params);
                if seen.insert(candidate.clone()) {
                    return candidate;
                }

                let path = Path::new(&candidate);
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or_default()
                    .to_owned();
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output")
                    .to_owned();
                let mut n: u32 = 2;
                loop {
                    let new_name = if ext.is_empty() {
                        format!("{}_{}", stem, n)
                    } else {
                        format!("{}_{}.{}", stem, n, ext)
                    };
                    let disambiguated = path
                        .with_file_name(&new_name)
                        .to_string_lossy()
                        .into_owned();
                    if seen.insert(disambiguated.clone()) {
                        break disambiguated;
                    }
                    n += 1;
                }
            })
            .collect()
    }

    /// Encodes `img` into a heap-allocated byte buffer in the requested format.
    ///
    /// # Arguments
    ///
    /// * `img`: The image to be encoded.
    /// * `params`: The compression parameters containing format and quality info.
    ///
    /// # Returns
    ///
    /// A `Result` containing the encoded image bytes on success, or an error string on failure.
    fn encode(&self, img: &DynamicImage, params: &CompressionParams) -> Result<Vec<u8>, String> {
        let mut cursor = Cursor::new(Vec::new());

        match params.format {
            OutputFormat::Jpeg => {
                let mut encoder =
                    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, params.quality);
                encoder
                    .encode_image(img)
                    .map_err(|e| format!("Failed to encode JPEG: {e}"))?;
            }
            OutputFormat::WebP => {
                let encoder = webp::Encoder::from_image(img)
                    .map_err(|e| format!("Failed to create WebP encoder: {e}"))?;

                let webp_memory = if params.quality == 100 {
                    encoder.encode_lossless()
                } else {
                    encoder.encode(params.quality as f32)
                };

                cursor.get_mut().extend_from_slice(&webp_memory);
            }
            OutputFormat::Png => {
                img.write_to(&mut cursor, ImageFormat::Png)
                    .map_err(|e| format!("Failed to encode PNG: {e}"))?;
            }
            OutputFormat::Gif => {
                img.write_to(&mut cursor, ImageFormat::Gif)
                    .map_err(|e| format!("Failed to encode GIF: {e}"))?;
            }
            OutputFormat::Bmp => {
                let mut encoder = image::codecs::bmp::BmpEncoder::new(&mut cursor);
                let (bytes, color_type) = self.pixel_data_slice(img);
                encoder
                    .encode(&bytes, img.width(), img.height(), color_type)
                    .map_err(|e| format!("Failed to encode BMP: {e}"))?;
            }
            OutputFormat::Tiff => {
                let encoder = image::codecs::tiff::TiffEncoder::new(&mut cursor);
                let (bytes, color_type) = self.pixel_data_slice(img);
                encoder
                    .write_image(&bytes, img.width(), img.height(), color_type)
                    .map_err(|e| format!("Failed to write TIFF: {e}"))?;
            }
        }

        Ok(cursor.into_inner())
    }

    /// Re-injects EXIF bytes into an already-encoded in-memory image buffer.
    ///
    /// GIF, BMP, and TIFF do not support EXIF via `img-parts`; for those the
    /// original `bytes` are returned unchanged without an extra copy.
    ///
    /// # Arguments
    ///
    /// * `bytes`: The encoded image bytes to inject EXIF into.
    /// * `exif`: The EXIF bytes to inject.
    /// * `format`: The output format of the image, used to determine how to inject EXIF.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new image bytes with EXIF injected on success, or an error string on failure.
    fn inject_exif(
        &self,
        bytes: Vec<u8>,
        exif: img_parts::Bytes,
        format: OutputFormat,
    ) -> Result<Vec<u8>, String> {
        // The img-parts container types (Jpeg/Png/WebP) don't share a trait, but
        // they expose the same `from_bytes`/`set_exif`/`encoder().write_to` shape,
        // so a macro collapses the otherwise-identical arms.
        macro_rules! inject {
            ($ty:path, $label:literal, $bytes:expr) => {{
                let mut container = <$ty>::from_bytes($bytes)
                    .map_err(|e| format!(concat!("Failed to parse output ", $label, ": {}"), e))?;
                container.set_exif(Some(exif));
                let mut buf = Vec::new();
                container
                    .encoder()
                    .write_to(&mut buf)
                    .map_err(|e| format!(concat!("Failed to write EXIF to ", $label, ": {}"), e))?;
                Ok(buf)
            }};
        }

        match format {
            // GIF/BMP/TIFF cannot carry EXIF via img-parts; return bytes untouched.
            OutputFormat::Gif | OutputFormat::Bmp | OutputFormat::Tiff => Ok(bytes),
            OutputFormat::Jpeg => inject!(img_parts::jpeg::Jpeg, "JPEG", bytes.into()),
            OutputFormat::Png => inject!(img_parts::png::Png, "PNG", bytes.into()),
            OutputFormat::WebP => inject!(img_parts::webp::WebP, "WebP", bytes.into()),
        }
    }
}
