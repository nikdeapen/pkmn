use file_storage::{FolderPath, StoragePath};
use web_scrape::cache::WebCache;

/// Builds the shared on-disk [WebCache] rooted at `$HOME/.web-cache/`.
#[must_use]
pub fn local_cache() -> WebCache {
    let home: String = std::env::var("HOME").expect("$HOME not set");
    let path: StoragePath =
        StoragePath::parse(format!("{home}/.web-cache/")).expect("invalid web-cache path");
    let folder: FolderPath = unsafe { FolderPath::new(path) };
    WebCache::new(Some(folder), None)
}
