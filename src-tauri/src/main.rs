// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::GenericImageView;
use mozjpeg::{ColorSpace, Compress, ScanMode};
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open, compress_image])
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
fn compress_image(path: &str, quality: f32, max_width: u32, max_height: u32) -> Result<(), String> {
    // Load the image
    let img = image::open(path).expect("Failed to open image");

    // Get image dimensions and raw pixels
    let (mut width, mut height) = img.dimensions();
    let raw_pixels = img.to_rgb8().into_raw();

    if max_width > 0 && width > max_width {
        height = (height as f32 * max_width as f32 / width as f32) as u32;
        width = max_width;
    } else if max_height > 0 && height > max_height {
        width = (width as f32 * max_height as f32 / height as f32) as u32;
        height = max_height;
    }

    let res = std::panic::catch_unwind(|| -> std::io::Result<Vec<u8>> {
        let mut comp = Compress::new(ColorSpace::JCS_RGB);

        println!("Image dimensions: {}x{}", width, height);
        println!("Quality: {}", quality);

        comp.set_size(width as usize, height as usize);
        comp.set_quality(quality);
        //comp.set_progressive_mode();
        comp.set_scan_optimization_mode(ScanMode::Auto);

        let mut comp = comp.start_compress(Vec::new())?; // any io::Write will work

        // replace with your image data
        comp.write_scanlines(&raw_pixels)?;

        let writer = comp.finish()?;
        Ok(writer)
    });

    match res {
        Ok(writer) => match writer {
            Ok(writer) => {
                let mut file =
                    BufWriter::new(File::create("output.jpg").expect("Failed to create file"));
                file.write_all(&writer).expect("Failed to write image");
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}
