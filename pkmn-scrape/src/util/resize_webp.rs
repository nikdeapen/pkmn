use image::imageops::FilterType;
use image::{DynamicImage, RgbaImage};
use std::error::Error;
use webp::{Encoder, PixelLayout, WebPMemory};

/// Resizes the WebP `data` to exactly `width`×`height`, re-encoding as lossy WebP at `quality`.
/// (quality is `0.0..=100.0`)
pub fn resize_webp(
    data: &[u8],
    width: u32,
    height: u32,
    quality: f32,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let image: DynamicImage = image::load_from_memory(data)?;
    let resized: RgbaImage = image
        .resize_exact(width, height, FilterType::Lanczos3)
        .to_rgba8();
    let memory: WebPMemory =
        Encoder::new(resized.as_raw(), PixelLayout::Rgba, width, height).encode(quality);
    Ok(memory.to_vec())
}
