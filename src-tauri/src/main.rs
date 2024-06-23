// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod image_processor;

use rayon::prelude::*;
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
