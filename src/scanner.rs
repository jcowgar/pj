use crate::config::Config;
use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Project {
    pub path: PathBuf,
    /// Relative path from scan root for display and matching
    pub display_path: String,
}

impl Project {
    pub fn new(path: PathBuf, scan_root: &Path) -> Self {
        let display_path = path
            .strip_prefix(scan_root)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

        Self { path, display_path }
    }

    /// Get the display path for matching (e.g., "ai/decree-ng/main")
    pub fn display_path(&self) -> &str {
        &self.display_path
    }
}

/// Check if a directory is a project based on the markers
fn is_project(dir: &Path, markers: &[String]) -> bool {
    markers.iter().any(|marker| dir.join(marker).exists())
}

/// Scan directories for project roots only
pub fn scan_projects(config: &Config) -> Result<Vec<Project>> {
    let mut projects = Vec::new();

    for scan_path in &config.scan_paths {
        // Expand tilde in path
        let scan_path = shellexpand::tilde(&scan_path.to_string_lossy()).to_string();
        let scan_path = PathBuf::from(scan_path);

        if !scan_path.exists() {
            eprintln!("Warning: Scan path does not exist: {}", scan_path.display());
            continue;
        }

        // Find project roots (directories containing project markers)
        for entry in WalkDir::new(&scan_path)
            .max_depth(config.max_depth)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_dir() && is_project(path, &config.project_markers) {
                projects.push(Project::new(path.to_path_buf(), &scan_path));
            }
        }
    }

    Ok(projects)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_project_new() {
        let scan_root = PathBuf::from("/home/user/projects");
        let project_path = PathBuf::from("/home/user/projects/myapp");

        let project = Project::new(project_path.clone(), &scan_root);

        assert_eq!(project.path, project_path);
        assert_eq!(project.display_path, "myapp");
    }

    #[test]
    fn test_project_new_nested() {
        let scan_root = PathBuf::from("/home/user/projects");
        let project_path = PathBuf::from("/home/user/projects/ai/decree-ng");

        let project = Project::new(project_path.clone(), &scan_root);

        assert_eq!(project.path, project_path);
        assert_eq!(project.display_path, "ai/decree-ng");
    }

    #[test]
    fn test_project_display_path() {
        let scan_root = PathBuf::from("/home/user");
        let project_path = PathBuf::from("/home/user/code/rust/myproject");

        let project = Project::new(project_path, &scan_root);

        assert_eq!(project.display_path(), "code/rust/myproject");
    }

    #[test]
    fn test_is_project_with_git() {
        let temp_dir = TempDir::new().unwrap();
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(&git_dir).unwrap();

        let markers = vec![".git".to_string(), ".jj".to_string()];
        assert!(is_project(temp_dir.path(), &markers));
    }

    #[test]
    fn test_is_project_with_jj() {
        let temp_dir = TempDir::new().unwrap();
        let jj_dir = temp_dir.path().join(".jj");
        fs::create_dir(&jj_dir).unwrap();

        let markers = vec![".git".to_string(), ".jj".to_string()];
        assert!(is_project(temp_dir.path(), &markers));
    }

    #[test]
    fn test_is_project_without_markers() {
        let temp_dir = TempDir::new().unwrap();

        let markers = vec![".git".to_string(), ".jj".to_string()];
        assert!(!is_project(temp_dir.path(), &markers));
    }

    #[test]
    fn test_is_project_with_file_marker() {
        let temp_dir = TempDir::new().unwrap();
        let marker_file = temp_dir.path().join("Cargo.toml");
        fs::write(&marker_file, "").unwrap();

        let markers = vec!["Cargo.toml".to_string()];
        assert!(is_project(temp_dir.path(), &markers));
    }

    #[test]
    fn test_scan_projects_finds_git_repos() {
        let temp_dir = TempDir::new().unwrap();

        // Create a project with .git marker
        let project1 = temp_dir.path().join("project1");
        fs::create_dir(&project1).unwrap();
        fs::create_dir(project1.join(".git")).unwrap();

        // Create another project
        let project2 = temp_dir.path().join("project2");
        fs::create_dir(&project2).unwrap();
        fs::create_dir(project2.join(".jj")).unwrap();

        // Create a non-project directory
        let not_project = temp_dir.path().join("not-project");
        fs::create_dir(&not_project).unwrap();

        let config = Config {
            scan_paths: vec![temp_dir.path().to_path_buf()],
            project_markers: vec![".git".to_string(), ".jj".to_string()],
            max_depth: 2,
        };

        let projects = scan_projects(&config).unwrap();

        assert_eq!(projects.len(), 2);
        assert!(projects.iter().any(|p| p.display_path == "project1"));
        assert!(projects.iter().any(|p| p.display_path == "project2"));
    }

    #[test]
    fn test_scan_projects_respects_max_depth() {
        let temp_dir = TempDir::new().unwrap();

        // Create nested structure
        let level1 = temp_dir.path().join("level1");
        let level2 = level1.join("level2");
        let level3 = level2.join("level3");
        let level4 = level3.join("level4");

        fs::create_dir_all(&level4).unwrap();
        fs::create_dir(level4.join(".git")).unwrap();

        // Config with max_depth=3 should not find level4 project
        let config = Config {
            scan_paths: vec![temp_dir.path().to_path_buf()],
            project_markers: vec![".git".to_string()],
            max_depth: 3,
        };

        let projects = scan_projects(&config).unwrap();
        assert_eq!(projects.len(), 0);

        // Config with max_depth=4 should find it
        let config = Config {
            scan_paths: vec![temp_dir.path().to_path_buf()],
            project_markers: vec![".git".to_string()],
            max_depth: 4,
        };

        let projects = scan_projects(&config).unwrap();
        assert_eq!(projects.len(), 1);
    }

    #[test]
    fn test_scan_projects_empty_when_no_markers() {
        let temp_dir = TempDir::new().unwrap();

        // Create directories without markers
        fs::create_dir(temp_dir.path().join("dir1")).unwrap();
        fs::create_dir(temp_dir.path().join("dir2")).unwrap();

        let config = Config {
            scan_paths: vec![temp_dir.path().to_path_buf()],
            project_markers: vec![".git".to_string()],
            max_depth: 2,
        };

        let projects = scan_projects(&config).unwrap();
        assert_eq!(projects.len(), 0);
    }
}
