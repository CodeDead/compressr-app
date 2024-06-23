// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::GenericImageView;
use mozjpeg::{ColorSpace, Compress};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::thread::available_parallelism;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open,
            compress_image,
            get_number_of_threads
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open(site: &str) -> Result<(), String> {
    match open::that(site) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn compress_image(
    files: Vec<String>,
    quality: f32,
    max_width: u32,
    max_height: u32,
    num_threads: usize,
) -> Result<String, String> {
    // Check if the number of threads is larger than the number of files and if so, replace the number of threads with the amount of files
    let num_threads = if num_threads > files.len() {
        files.len()
    } else {
        num_threads
    };

    // Configure the number of threads for rayon
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .expect("Failed to set number of threads");

    // Compress images in parallel and collect results
    let results: Vec<_> = pool.install(|| {
        files
            .par_iter()
            .map(|input| {
                process_image(input, "", quality, max_width, max_height, false)
                    .map_err(|e| format!("Error processing {}: {}", input, e))
            })
            .collect()
    });

    // Handle results
    let mut errors = String::new();
    for result in results {
        match result {
            Ok(()) => {}
            Err(e) => {
                errors = format!("{}\n{}", errors, e);
                eprintln!("{}", e)
            }
        }
    }

    Ok(errors)
}

/// Get the number of threads that can be used for port scanning
///
/// # Returns
///
/// * `u32` - The number of threads that can be used for port scanning
#[tauri::command]
fn get_number_of_threads() -> usize {
    let default_parallelism_approx = available_parallelism().unwrap().get();
    default_parallelism_approx
}

fn process_image(
    input_path: &str,
    output_path: &str,
    quality: f32,
    max_width: u32,
    max_height: u32,
    delete_original: bool,
) -> Result<(), String> {
    // Load the image
    let mut img = image::open(input_path).expect("Failed to open image");

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
        Err(e) => {
            return Err(String::from("Unknown mozjpeg error"));
        }
    }
}
