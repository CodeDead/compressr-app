use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub(crate) const IMAGE_EXTENSIONS: &[&str] =
    &["png", "jpg", "jpeg", "bmp", "gif", "webp", "tiff", "tif"];

/// Returns `true` when `path` has a supported image extension (case-insensitive).
///
/// # Arguments
///
/// * `path` - The path whose extension is checked.
///
/// # Returns
///
/// `true` if the extension matches one of [`IMAGE_EXTENSIONS`], `false` otherwise.
fn is_image_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
}

/// Scans a folder to find image files based on a predefined set of valid image extensions.
///
/// # Arguments
/// * `folder`: A `PathBuf` representing the folder to scan.
/// * `recursive`: A `bool` indicating whether the scan should include subdirectories recursively.
///
/// # Returns
/// * `Ok(Vec<String>)`: A vector of file paths (as strings) pointing to image files found
///   in the folder (and subdirectories, if `recursive` is `true`).
/// * `Err(String)`: An error message indicating why the scan could not be completed. This could
///   include issues such as failure to read a folder, access directory entries, or fetch metadata
///   for entries.
pub fn scan_folder(folder: PathBuf, recursive: bool) -> Result<Vec<String>, String> {
    let mut entry_errors: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    if recursive {
        let mut dirs: Vec<PathBuf> = vec![folder.clone()];
        // Seed with the root itself so a symlinked subdirectory pointing back
        // to the root cannot cause it to be scanned a second time.
        let mut visited: HashSet<PathBuf> = HashSet::new();
        visited.insert(folder.canonicalize().unwrap_or_else(|_| folder.clone()));

        while let Some(dir) = dirs.pop() {
            match std::fs::read_dir(&dir) {
                Err(e) => entry_errors.push(format!("Could not read '{}': {e}", dir.display())),
                Ok(entries) => {
                    for entry in entries {
                        let entry = match entry {
                            Err(e) => {
                                entry_errors.push(format!("Directory entry error: {e}"));
                                continue;
                            }
                            Ok(e) => e,
                        };
                        let path = entry.path();
                        let metadata = match path.metadata() {
                            Ok(m) => m,
                            Err(e) => {
                                entry_errors.push(format!(
                                    "Could not read metadata for '{}': {e}",
                                    path.display()
                                ));
                                continue;
                            }
                        };

                        if metadata.is_dir() {
                            let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
                            if visited.insert(canonical) {
                                dirs.push(path);
                            }
                        } else if metadata.is_file() && is_image_file(&path) {
                            files.push(path.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
    } else {
        match std::fs::read_dir(&folder) {
            Err(e) => {
                return Err(format!("Could not read folder '{}': {e}", folder.display()));
            }
            Ok(entries) => {
                for entry in entries {
                    let entry = match entry {
                        Err(e) => {
                            entry_errors.push(format!("Directory entry error: {e}"));
                            continue;
                        }
                        Ok(e) => e,
                    };
                    let path = entry.path();
                    if path.is_file() && is_image_file(&path) {
                        files.push(path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    if !entry_errors.is_empty() {
        return Err(entry_errors.join("\n"));
    }

    Ok(files)
}
