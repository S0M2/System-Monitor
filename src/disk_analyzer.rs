use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub name: String,
    pub is_dir: bool,
}

/// Disk analyzer - shows immediate items (files + dirs) in current folder
pub struct DiskAnalyzer {
    pub current_path: PathBuf,
    pub items: Vec<FileEntry>,
    pub parent_path: Option<PathBuf>,
}

impl DiskAnalyzer {
    pub fn new(start_path: &str) -> Self {
        let path = PathBuf::from(start_path);
        let mut analyzer = Self {
            current_path: path.clone(),
            items: Vec::new(),
            parent_path: None,
        };
        analyzer.scan();
        analyzer
    }

    /// Ultra-fast scan: load current directory only (no recursion)
    pub fn scan(&mut self) {
        self.items.clear();

        // Get parent path
        self.parent_path = self.current_path.parent().map(|p| p.to_path_buf());

        // Scan immediate items in current directory only
        if let Ok(entries) = fs::read_dir(&self.current_path) {
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

                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();

                // Get metadata WITHOUT recursion
                let (size, is_dir) = if let Ok(metadata) = entry.metadata() {
                    let sz = metadata.len();
                    let is_d = metadata.is_dir();
                    (sz, is_d)
                } else {
                    (0, path.is_dir())
                };

                self.items.push(FileEntry {
                    path,
                    size,
                    name,
                    is_dir,
                });
            }
        }

        // Sort by size descending, but directories first
        self.items.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less, // dirs first
                (false, true) => std::cmp::Ordering::Greater,
                _ => b.size.cmp(&a.size), // then by size
            }
        });
    }

    /// Navigate into a subdirectory (only if selected item is a directory)
    pub fn enter_folder(&mut self, index: usize) {
        if index < self.items.len() && self.items[index].is_dir {
            self.current_path = self.items[index].path.clone();
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

    /// Open selected item in Finder
    pub fn open_in_finder(&self, index: usize) -> bool {
        if index < self.items.len() {
            let path = &self.items[index].path;
            let path_str = path.to_string_lossy().to_string();

            // Use 'open -R' to reveal the file/folder in Finder
            let result = std::process::Command::new("open")
                .arg("-R")
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

    /// Get total size of all items in current view
    pub fn total_size(&self) -> u64 {
        self.items.iter().map(|f| f.size).sum()
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
