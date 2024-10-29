# optimized-hd2sd
A Rust application designed to process osu skin HD textures and create optimized SD textures.

## Features
- Recursively finds all HD textures (denoted with `@2x` in file name) in the selected directory and its subdirectories.
- Creates new SD textures using Catmull-Rom filtering for high-quality downscaling.
- Losslessly optimizes both input (HD) and output (SD) file sizes using oxipng.
- Processes images in batches for improved performance.
- Provides options to preserve existing SD images and customize batch size.

## Usage

### GUI
1. Run the executable.
2. Select the directory containing the skin files when prompted.
3. The application will process all `@2x` images in the selected directory and its subdirectories.

### Command Line Arguments
- `-b, --batch-size <SIZE>`: Set the batch size for processing images (default:20)
- `-p, --preserve`: Preserve existing SD images
- `-h, --help`: Display help information
