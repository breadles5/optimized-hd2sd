use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use image::{GenericImageView, ImageBuffer, DynamicImage, imageops::FilterType};
use rfd::FileDialog;

fn main() {
    // Open a file explorer window to select the directory
    let dir_to_process = FileDialog::new()
        .set_directory(".")
        .pick_folder();

    let dir_path = match dir_to_process {
        Some(path) => path,
        None => {
            eprintln!("No directory selected.");
            std::process::exit(1);
        }
    };

    // Traverse the directory and subdirectories
    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().to_string_lossy().contains("@2x"))
    {
        let path = entry.path();
        println!("Processing file: {:?}", path);

        // Open the image file
        match image::open(&path) {
            Ok(img) => {
                // Downscale the image by a factor of 2 using a bicubic filter
                let (width, height) = img.dimensions();
                let scaled_img = img.resize(width / 2, height / 2, FilterType::CatmullRom);

                // Create a new filename by removing the "@2x" part
                if let Some(new_filename) = path.file_name().and_then(|name| name.to_str()) {
                    let new_filename = new_filename.replace("@2x", "");
                    let mut new_path = PathBuf::from(path.parent().unwrap());
                    new_path.push(new_filename);

                    // Save the downscaled image to the new path
                    if let Err(e) = scaled_img.save(&new_path) {
                        eprintln!("Failed to save image {:?}: {}", new_path, e);
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to open image {:?}: {}", path, e);
            }
        }
    }
}