// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod image_processor;

use rayon::prelude::*;
use std::path::Path;
use std::thread::available_parallelism;
use walkdir::WalkDir;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open,
            compress_image,
            get_number_of_threads,
            get_images_from_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Open a website in the default browser
///
/// # Arguments
///
/// * `site` - The website to open
///
/// # Returns
///
/// * `Result<(), String>` - An error message if the operation failed
#[tauri::command]
fn open(site: &str) -> Result<(), String> {
    match open::that(site) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

/// Compress images
///
/// # Arguments
///
/// * `files` - A list of image paths
/// * `quality` - The quality of the compressed images
/// * `max_width` - The maximum width of the compressed images
/// * `max_height` - The maximum height of the compressed images
/// * `num_threads` - The number of threads to use for compression
/// * `delete_original` - Whether to delete the original images
///
/// # Returns
///
/// * `Result<String, String>` - An error message if the operation failed
#[tauri::command]
async fn compress_image(
    files: Vec<String>,
    quality: f32,
    max_width: u32,
    max_height: u32,
    num_threads: usize,
    delete_original: bool,
) -> Result<String, String> {
    // Check if the number of threads is larger than the number of files and if so, replace the number of threads with the amount of files
    let num_threads = if num_threads > files.len() {
        files.len()
    } else {
        num_threads
    };

    // Configure the number of threads for rayon
    let pool = match rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
    {
        Ok(pool) => pool,
        Err(e) => return Err(e.to_string()),
    };

    // Compress images in parallel and collect results
    let results: Vec<_> = pool.install(|| {
        files
            .par_iter()
            .map(|input| {
                image_processor::process_image(
                    input,
                    "",
                    quality,
                    max_width,
                    max_height,
                    delete_original,
                )
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

/// Get a list of images from a directory
///
/// # Arguments
///
/// * `directory` - The directory to search for images
///
/// # Returns
///
/// * `Vec<String>` - A list of image paths
/// * `String` - An error message if the operation failed
#[tauri::command]
async fn get_images_from_directory(directory: &str) -> Result<Vec<String>, String> {
    let mut images = Vec::new();
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() && is_image_file(path) {
            images.push(path.to_str().unwrap().to_string());
        }
    }

    Ok(images)
}

/// Check if a file is an image file
///
/// # Arguments
///
/// * `path` - The path to the file
///
/// # Returns
///
/// * `bool` - Whether the file is a supported image file
fn is_image_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        return match extension.to_str().unwrap_or("").to_lowercase().as_str() {
            "avif" | "bmp" | "dds" | "farbfeld" | "gif" | "hdr" | "ico" | "jpeg" | "jpg"
            | "exr" | "png" | "pnm" | "qoi" | "tga" | "tiff" | "webp" => true,
            _ => false,
        };
    }
    false
}
