use std::fs;
use std::path::Path;

use clap::Parser;
use colored::*;
use rayon::prelude::*;

use crate::black::*;
mod black;

fn process_and_print(path: &Path, parallel: bool) {
    match black(path, parallel) {
        Ok(percentage) => {
            let filename = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");
            println!("{}: {:.2}%", filename.green(), percentage);
        }
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
        }
    }
}

#[derive(Parser)]
#[command(name = "black")]
#[command(about = "Determine the percentage of black pixels in an image")]
#[command(version = "0.3.0")]
#[command(author = "Luke Hannan")]
struct Args {
    // File path of the image
    path: String,

    /// Process a whole directory
    #[arg(short, long, default_value_t = false)]
    directory: bool,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);

    if args.directory {
        if !path.is_dir() {
            eprintln!("{}: The provided path is not a directory.", "Error".red());
            return;
        } else {
            let entries = match fs::read_dir(path) {
                Ok(entries) => entries,
                Err(e) => {
                    eprintln!("{}: Failed to read directory: {}", "Error".red(), e);
                    return;
                }
            };

            let image_files: Vec<_> = entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| is_image_file(path))
                .collect();

            if image_files.is_empty() {
                println!("No image files found in the directory.");
                return;
            }

            image_files.par_iter().for_each(|path| {
                process_and_print(path, false);
            });
            return;
        }
    }

    if !is_image_file(path) {
        eprintln!(
            "{}: The provided path does not point to a valid image file.",
            "Error".red()
        );
        return;
    }

    process_and_print(path, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_image_file() {
        assert!(is_image_file(Path::new("test.jpg")));
        assert!(is_image_file(Path::new("test.png")));
        assert!(is_image_file(Path::new("test.jpeg")));
        assert!(is_image_file(Path::new("test.gif")));
        assert!(is_image_file(Path::new("test.bmp")));
        assert!(is_image_file(Path::new("test.tiff")));
        assert!(is_image_file(Path::new("test.webp")));

        assert!(!is_image_file(Path::new("test.txt")));
        assert!(!is_image_file(Path::new("test.rs")));
        assert!(!is_image_file(Path::new("test")));
    }

    #[test]
    fn test_process_and_print_error_handling() {
        // Test with non-existent file
        let fake_path = Path::new("non_existent_file.jpg");
        // This should not panic, just print an error
        process_and_print(fake_path, false);
    }

    #[test]
    fn test_path_filename_extraction() {
        let path = Path::new("/some/path/image.jpg");
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        assert_eq!(filename, "image.jpg");

        let path_no_extension = Path::new("/some/path/file");
        let filename = path_no_extension
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        assert_eq!(filename, "file");
    }
}
