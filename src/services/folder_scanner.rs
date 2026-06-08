use std::collections::HashSet;
use std::path::PathBuf;

pub(crate) const IMAGE_EXTENSIONS: &[&str] =
    &["png", "jpg", "jpeg", "bmp", "gif", "webp", "tiff", "tif"];

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
        let mut dirs: Vec<PathBuf> = vec![folder];
        let mut visited: HashSet<PathBuf> = HashSet::new();

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
                        } else if metadata.is_file()
                            && let Some(ext) = path.extension().and_then(|e| e.to_str())
                            && IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str())
                        {
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
                    if !path.is_file() {
                        continue;
                    }
                    if let Some(ext) = path.extension()
                        && let Some(ext_str) = ext.to_str()
                        && IMAGE_EXTENSIONS.contains(&ext_str.to_lowercase().as_str())
                    {
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
