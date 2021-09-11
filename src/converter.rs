use std::path::Path;

use crate::errors::VSMPError;
use image::FilterType;

pub fn convert(path: &Path, width: u32, height: u32) -> Result<Vec<u8>, VSMPError> {
    let img = image::open(&path)?;
    let resized_image = img.resize_exact(width / 2, height, FilterType::Lanczos3);
    let mut gray_image = resized_image.grayscale().adjust_contrast(50.0).to_luma();
    image::imageops::dither(&mut gray_image, &image::imageops::colorops::BiLevel);
    let buffer = gray_image
        .to_vec()
        .into_iter()
        .map(|e| if e == 255 { 0x03 } else { e })
        .collect();
    Ok(buffer)
}
