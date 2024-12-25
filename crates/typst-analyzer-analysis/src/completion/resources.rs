
use std::path::PathBuf;
use walkdir::{WalkDir, DirEntry};

use crate::typ_logger;

pub fn get_images() -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut images = Vec::new();
    let c_dir = std::env::current_dir()?; // Get the current directory
    typ_logger!("c_dir: {:?}", c_dir);

    if !c_dir.exists() || !c_dir.is_dir() {
        typ_logger!("Directory does not exist or is not a valid directory: {:?}", c_dir);
        return Ok(Vec::new());
    }

    let walker = WalkDir::new(&c_dir).into_iter(); // Initialize the iterator

    for entry in walker.filter_entry(is_hidden) {
        match entry {
            Ok(entry) => {
                typ_logger!("Found entry: {:?}", entry.path());

                if entry.path().is_dir() {
                    continue; // Skip directories
                } else if entry.path().is_file() {
                    let filename = entry.path().file_name();
                    if let Some(ext) = filename {
                        typ_logger!("File name: {:?}", ext);
                        // Normalize extension extraction and match against supported types
                        if let Some(ext_str) = ext.to_str() {
                            typ_logger!("File extension: {:?}", ext_str);
                            let ext_str_lower = ext_str.to_lowercase();

                            // Check if file has a valid image extension
                            if ext_str_lower.ends_with("png") || ext_str_lower.ends_with("jpg") || ext_str_lower.ends_with("jpeg") || ext_str_lower.ends_with("gif") || ext_str_lower.ends_with("svg") {
                                // Convert to an absolute path
                                let r_path =entry.path().strip_prefix(&c_dir)?;
                                typ_logger!("Absolute path found: {:?}", r_path);

                                images.push(r_path.to_path_buf()); // Add to the images vector
                            }
                        }
                    }
                }
            }
            Err(e) => {
                typ_logger!("Error processing entry: {:?}", e);
            }
        }
    }

    Ok(images)
}

// Helper function to filter out hidden files
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/// Finds the project root by searching upward for a marker file/directory (e.g., `.git`)
pub fn find_project_root() -> Option<PathBuf> {
    let mut current_dir = std::env::current_dir().ok()?;
    typ_logger!("Starting search for project root from: {:?}", current_dir);

    while current_dir.parent().is_some() {
        let marker = current_dir.join(".git"); // Change `.git` to another marker if needed
        if marker.exists() {
            typ_logger!("Found project root: {:?}", current_dir);
            return Some(current_dir);
        }

        // Move one directory up
        current_dir = current_dir.parent()?.to_path_buf();
    }

    typ_logger!("No project root found");
    None
}
