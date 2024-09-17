use std::path::{Path, PathBuf};
use std::time::Instant;
use clap::{Command, Arg, value_parser};
use walkdir::WalkDir;
use image::{GenericImageView, imageops::FilterType};
use rfd::FileDialog;
use oxipng::{optimize, Options, InFile, OutFile};
use rayon::prelude::*;

fn main() {
    let total_start_time = Instant::now();

    let matches = define_cli_args();
    let preserve = matches.get_flag("preserve");
    let batch_size = *matches.get_one::<usize>("batch_size").expect("Invalid batch size");

    let dir_path = select_directory();
    let image_paths = collect_image_paths(&dir_path);

    process_images_in_batches(&image_paths, batch_size, preserve);

    let total_duration = total_start_time.elapsed();
    println!("Total processing time: {:?}", total_duration);
}

fn define_cli_args() -> clap::ArgMatches {
    Command::new("optimized-hd2sd")
        .version("1.0")
        .author("breadles5")
        .about("Downscales and optimizes @2x images")
        .arg(
            Arg::new("preserve")
                .short('p')
                .long("preserve")
                .action(clap::ArgAction::SetTrue)
                .help("Preserve existing downscaled images"),
        )
        .arg(
            Arg::new("batch_size")
                .short('b')
                .long("batch-size")
                .value_name("SIZE")
                .help("Set the batch size for processing")
                .value_parser(value_parser!(usize))
                .default_value("20"),
        )
        .get_matches()
}

fn select_directory() -> PathBuf {
    FileDialog::new()
        .set_directory(".")
        .pick_folder()
        .unwrap_or_else(|| {
            eprintln!("No directory selected.");
            std::process::exit(1);
        })
}

fn collect_image_paths(dir_path: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().to_string_lossy().contains("@2x"))
        .map(|e| e.path().to_owned())
        .collect()
}

fn process_images_in_batches(image_paths: &[PathBuf], batch_size: usize, preserve: bool) {
    for (i, batch) in image_paths.chunks(batch_size).enumerate() {
        let batch_start_time = Instant::now();
        println!("Processing batch {} of {}", i + 1, (image_paths.len() + batch_size - 1) / batch_size);
        
        batch.par_iter().for_each(|path| {
            process_single_image(path, preserve);
        });

        let batch_duration = batch_start_time.elapsed();
        println!("Time to process batch {}: {:?}", i + 1, batch_duration);
    }
}

fn process_single_image(path: &Path, preserve: bool) {
    let image_start_time = Instant::now();
    println!("Processing file: {:?}", path);

    optimize_original_image(path);
    process_and_save_downscaled_image(path, preserve);

    let image_duration = image_start_time.elapsed();
    println!("Time to process {:?}: {:?}", path, image_duration);
}

fn optimize_original_image(path: &Path) {
    let options = Options::from_preset(5);
    if let Err(e) = optimize(&InFile::Path(path.to_path_buf()), &OutFile::Path { path: None, preserve_attrs: false }, &options) {
        eprintln!("Failed to optimize original image {:?}: {}", path, e);
    } else {
        println!("Optimized original image at {:?}", path);
    }
}

fn process_and_save_downscaled_image(path: &Path, preserve: bool) {
    match image::open(path) {
        Ok(img) => {
            let (width, height) = img.dimensions();
            let scaled_img = img.resize(width / 2, height / 2, FilterType::CatmullRom);

            if let Some(new_filename) = path.file_name().and_then(|name| name.to_str()) {
                let new_filename = new_filename.replace("@2x", "");
                let new_path = path.with_file_name(new_filename);

                if new_path.exists() && preserve {
                    println!("Preserving existing image: {:?}", new_path);
                } else {
                    save_and_optimize_downscaled_image(&scaled_img, &new_path);
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to open image {:?}: {}", path, e);
        }
    }
}

fn save_and_optimize_downscaled_image(scaled_img: &image::DynamicImage, new_path: &Path) {
    if let Err(e) = scaled_img.save(new_path) {
        eprintln!("Failed to save image {:?}: {}", new_path, e);
    } else {
        let options = Options::from_preset(5);
        if let Err(e) = optimize(&InFile::Path(new_path.to_path_buf()), &OutFile::Path { path: None, preserve_attrs: false }, &options) {
            eprintln!("Failed to optimize downscaled image {:?}: {}", new_path, e);
        } else {
            println!("Optimized downscaled image saved at {:?}", new_path);
        }
    }
}
