use image::ImageReader;
use rayon::prelude::*;
use std::path::Path;

pub fn black(path: &Path, parallel: bool) -> Result<f64, image::ImageError> {
    let image = ImageReader::open(path)?.decode()?.into_rgb8();

    let (width, height) = image.dimensions();
    let total_pixels = (width * height) as f64;

    let black_pixels = if parallel {
        image
            .pixels()
            .collect::<Vec<_>>()
            .par_iter()
            .filter(|p| p.0[0] == 0 && p.0[1] == 0 && p.0[2] == 0)
            .count() as f64
    } else {
        image
            .pixels()
            .filter(|p| p.0[0] == 0 && p.0[1] == 0 && p.0[2] == 0)
            .count() as f64
    };

    if total_pixels == 0.0 {
        return Ok(0.0);
    }

    Ok((black_pixels / total_pixels) * 100.0)
}

pub fn is_image_file(path: &Path) -> bool {
    let image_extensions = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];

    match path.extension().and_then(|ext| ext.to_str()) {
        Some(extension) => image_extensions.contains(&extension.to_lowercase().as_str()),
        None => false,
    }
}
