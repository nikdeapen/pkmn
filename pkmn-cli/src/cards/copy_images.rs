use crate::cards::images::{KINDS, copy_image};
use std::error::Error;

/// Copies the logo and symbol images from `source_id` to `target_id` in R2.
/// Existing target images are replaced only when `overwrite` is set; otherwise they are kept.
pub fn copy_images(source_id: &str, target_id: &str, overwrite: bool) -> Result<(), Box<dyn Error>> {
    for kind in KINDS {
        copy_image(kind, source_id, target_id, overwrite)?;
    }
    Ok(())
}
