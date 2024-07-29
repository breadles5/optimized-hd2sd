use std::path::PathBuf;
use clap::{Command, Arg};
use walkdir::WalkDir;
use image::{GenericImageView, imageops::FilterType};
use rfd::FileDialog;
use oxipng::{optimize, Options, InFile, OutFile};

fn main() {
    // Define command-line arguments
    let matches = Command::new("optimized-hd2sd")
        .version("1.0")
        .author("breadles5")
        .about("Downscales and optimizes @2x images")
        .arg(
            Arg::new("preserve")
                .short('p')
                .long("preserve")
                .takes_value(false)
                .help("Preserve existing downscaled images"),
        )
        .get_matches();

    // Determine if we should preserve existing downscaled images
    let preserve = matches.is_present("preserve");

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

        // Optimize the original image file using oxipng
        let options = Options::from_preset(3); // You can adjust the preset level
        if let Err(e) = optimize(&InFile::Path(path.to_path_buf()), &OutFile::Path(None), &options) {
            eprintln!("Failed to optimize original image {:?}: {}", path, e);
        } else {
            println!("Optimized original image at {:?}", path);
        }

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

                    // Check if the downscaled image already exists
                    if new_path.exists() && preserve {
                        println!("Preserving existing image: {:?}", new_path);
                    } else {
                        // Save the downscaled image to the new path
                        if let Err(e) = scaled_img.save(&new_path) {
                            eprintln!("Failed to save image {:?}: {}", new_path, e);
                        } else {
                            // Optimize the new image file using oxipng
                            if let Err(e) = optimize(&InFile::Path(new_path.clone()), &OutFile::Path(None), &options) {
                                eprintln!("Failed to optimize downscaled image {:?}: {}", new_path, e);
                            } else {
                                println!("Optimized downscaled image saved at {:?}", new_path);
                            }
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to open image {:?}: {}", path, e);
            }
        }
    }
}
