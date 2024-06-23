use image::GenericImageView;
use mozjpeg::{ColorSpace, Compress};
use std::fs::File;
use std::io::{BufWriter, Write};

/// Compress an image
/// 
/// # Arguments
/// 
/// * `input_path` - The path to the input image
/// * `output_path` - The path to the output image
/// * `quality` - The quality of the compressed image
/// * `max_width` - The maximum width of the compressed image
/// * `max_height` - The maximum height of the compressed image
/// * `delete_original` - Whether to delete the original image
/// 
/// # Returns
/// 
/// * `Result<(), String>` - An error message if the operation failed
pub fn process_image(
    input_path: &str,
    output_path: &str,
    quality: f32,
    max_width: u32,
    max_height: u32,
    delete_original: bool,
) -> Result<(), String> {
    // Load the image
    let mut img = match image::open(input_path) {
        Ok(img) => img,
        Err(e) => return Err(e.to_string()),
    };

    // Get image dimensions and raw pixels
    let (width, height) = img.dimensions();
    if max_width > 0 && width > max_width {
        img = img.resize(
            max_width,
            (height as f32 * max_width as f32 / width as f32) as u32,
            image::imageops::FilterType::Lanczos3,
        );
    } else if max_height > 0 && height > max_height {
        img = img.resize(
            (width as f32 * max_height as f32 / height as f32) as u32,
            max_height,
            image::imageops::FilterType::Lanczos3,
        );
    }
    let (width, height) = img.dimensions();
    let raw_pixels = img.to_rgb8().into_raw();

    let res = std::panic::catch_unwind(|| -> std::io::Result<Vec<u8>> {
        let mut comp = Compress::new(ColorSpace::JCS_RGB);

        comp.set_size(width as usize, height as usize);
        comp.set_quality(quality);
        comp.set_optimize_scans(true);
        comp.set_progressive_mode();

        let mut comp = comp.start_compress(Vec::new())?;

        // replace with your image data
        comp.write_scanlines(&raw_pixels)?;

        let writer = comp.finish()?;
        Ok(writer)
    });

    match res {
        Ok(writer) => match writer {
            Ok(writer) => {
                let mut output_path = output_path.to_string();
                if output_path.is_empty() {
                    // Get the file name from the input path without the extension
                    let file_name = std::path::Path::new(input_path)
                        .file_stem()
                        .expect("Failed to get file name")
                        .to_str()
                        .expect("Failed to convert file name to string");

                    let original_file_directory = std::path::Path::new(input_path)
                        .parent()
                        .expect("Failed to get parent directory")
                        .to_str()
                        .expect("Failed to convert parent directory to string");

                    output_path =
                        format!("{}/{}_compressed.jpg", original_file_directory, file_name);
                }

                let mut file =
                    BufWriter::new(File::create(output_path).expect("Failed to create file"));
                file.write_all(&writer).expect("Failed to write image");

                if delete_original {
                    std::fs::remove_file(input_path).expect("Failed to delete original image");
                }
                Ok(())
            }
            Err(e) => return Err(e.to_string()),
        },
        Err(_) => {
            return Err(String::from("Unknown mozjpeg error"));
        }
    }
}