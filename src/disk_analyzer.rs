use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use walkdir::WalkDir;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct FolderEntry {
    pub path: PathBuf,
    pub size: u64,
    pub name: String,
}

/// Disk analyzer for hierarchical folder size scanning (optimized)
pub struct DiskAnalyzer {
    pub current_path: PathBuf,
    pub folders: Vec<FolderEntry>,
    pub parent_path: Option<PathBuf>,
    size_cache: Arc<Mutex<HashMap<PathBuf, u64>>>,
}

impl DiskAnalyzer {
    pub fn new(start_path: &str) -> Self {
        let path = PathBuf::from(start_path);
        let mut analyzer = Self {
            current_path: path.clone(),
            folders: Vec::new(),
            parent_path: None,
            size_cache: Arc::new(Mutex::new(HashMap::new())),
        };
        analyzer.scan();
        analyzer
    }

    /// Fast scan: get folder sizes with caching (parallel WalkDir)
    pub fn scan(&mut self) {
        self.folders.clear();
        
        // Get parent path
        self.parent_path = self.current_path.parent().map(|p| p.to_path_buf());

        // Scan immediate subdirectories
        if let Ok(entries) = fs::read_dir(&self.current_path) {
            let mut temp_folders = Vec::new();
            
            for entry in entries.flatten() {
                let path = entry.path();
                
                // Skip hidden files/folders
                if path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.starts_with('.'))
                    .unwrap_or(false)
                {
                    continue;
                }

                if path.is_dir() {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown")
                        .to_string();

                    // Check cache first
                    let size = {
                        let cache = self.size_cache.lock().unwrap();
                        cache.get(&path).copied()
                    }.unwrap_or_else(|| {
                        // Calculate size with timeout-like behavior (limit depth for speed)
                        let size = Self::calculate_dir_size_fast(&path);
                        
                        // Cache result
                        if let Ok(mut cache) = self.size_cache.lock() {
                            cache.insert(path.clone(), size);
                        }
                        size
                    });

                    temp_folders.push(FolderEntry {
                        path,
                        size,
                        name,
                    });
                }
            }

            self.folders = temp_folders;
        }

        // Sort by size descending
        self.folders.sort_by(|a, b| b.size.cmp(&a.size));
    }

    /// Fast size calculation with early-exit heuristic
    fn calculate_dir_size_fast(path: &Path) -> u64 {
        // Use a simple counter to avoid infinite loops on deep directories
        let mut total: u64 = 0;
        let mut file_count = 0;
        const MAX_FILES: usize = 50000; // Stop after scanning 50k files
        
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .take(MAX_FILES)
        {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    total = total.saturating_add(metadata.len());
                    file_count += 1;
                }
            }
        }
        
        total
    }

    /// Navigate into a subdirectory
    pub fn enter_folder(&mut self, index: usize) {
        if index < self.folders.len() {
            self.current_path = self.folders[index].path.clone();
            self.scan();
        }
    }

    /// Navigate to parent directory
    pub fn go_back(&mut self) {
        if let Some(parent) = &self.parent_path {
            self.current_path = parent.clone();
            self.scan();
        }
    }

    /// Open selected folder in Finder
    pub fn open_in_finder(&self, index: usize) -> bool {
        if index < self.folders.len() {
            let path = &self.folders[index].path;
            let path_str = path.to_string_lossy().to_string();
            let result = std::process::Command::new("open")
                .arg(&path_str)
                .output();
            return result.is_ok();
        }
        false
    }

    /// Get current path as string
    pub fn current_path_str(&self) -> String {
        self.current_path.to_string_lossy().to_string()
    }

    /// Get total size of all folders in current view
    pub fn total_size(&self) -> u64 {
        self.folders.iter().map(|f| f.size).sum()
    }
}

/// Format bytes to human readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{} {}", size as u64, UNITS[unit_idx])
    } else {
        format!("{:.2} {}", size, UNITS[unit_idx])
    }
}

/// Calculate percentage of total
pub fn calc_percentage(value: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (value as f64 / total as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn test_calc_percentage() {
        assert_eq!(calc_percentage(50, 100), 50.0);
        assert_eq!(calc_percentage(0, 0), 0.0);
    }
}
