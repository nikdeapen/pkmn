use file_storage::{FilePath, FolderPath, StoragePath};
use std::collections::HashSet;
use std::error::Error;

/// The R2 bucket base url for static assets. (account + bucket)
const BUCKET: &str =
    "https://4a4bba3e2df525df01c99bae8307cbc5.r2.cloudflarestorage.com/pkmn-static";

/// The set image kinds. (each set should have both)
pub const KINDS: [&str; 2] = ["logo", "symbol"];

/// The set ids that have a `kind` image in R2, from a single bucket listing.
pub fn image_set_ids(kind: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let folder: FolderPath =
        StoragePath::parse(format!("{BUCKET}/cards/sets/{kind}/"))?.to_folder()?;
    let mut ids: HashSet<String> = HashSet::new();
    for file in folder.list_files_as_vec()? {
        if let Some(id) = file.file_name().strip_suffix(".webp") {
            ids.insert(id.to_string());
        }
    }
    Ok(ids)
}

/// Copies the set `kind` image from `old_id` to `new_id`, leaving the original in place.
/// A no-op when there is no source image or the target already exists.
pub fn copy_image(kind: &str, old_id: &str, new_id: &str) -> Result<(), Box<dyn Error>> {
    if let Some(data) = image_path(kind, old_id)?.read_as_vec_if_exists()? {
        image_path(kind, new_id)?.write_data_if_not_exists(data)?;
    }
    Ok(())
}

/// The R2 [FilePath] of a set `kind` image. (ex: `cards/sets/logo/151.webp`)
fn image_path(kind: &str, set_id: &str) -> Result<FilePath, Box<dyn Error>> {
    Ok(StoragePath::parse(format!("{BUCKET}/cards/sets/{kind}/{set_id}.webp"))?.to_file()?)
}
