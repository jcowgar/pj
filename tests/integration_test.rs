use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Note: These tests focus on the library components rather than the CLI
// since the CLI requires TTY interaction which is difficult to test

#[test]
fn test_end_to_end_project_scanning_and_matching() {
    // Create a temporary directory structure
    let temp_dir = TempDir::new().unwrap();

    // Create several projects
    let project1 = temp_dir.path().join("rust-project");
    fs::create_dir(&project1).unwrap();
    fs::create_dir(project1.join(".git")).unwrap();

    let project2 = temp_dir.path().join("go-project");
    fs::create_dir(&project2).unwrap();
    fs::create_dir(project2.join(".git")).unwrap();

    let nested_dir = temp_dir.path().join("category");
    fs::create_dir(&nested_dir).unwrap();

    let project3 = nested_dir.join("nested-project");
    fs::create_dir(&project3).unwrap();
    fs::create_dir(project3.join(".jj")).unwrap();

    // Create a non-project directory
    let non_project = temp_dir.path().join("not-a-project");
    fs::create_dir(&non_project).unwrap();

    // Create config
    let config = pj::config::Config {
        scan_paths: vec![temp_dir.path().to_path_buf()],
        project_markers: vec![".git".to_string(), ".jj".to_string()],
        max_depth: 3,
    };

    // Scan for projects
    let projects = pj::scanner::scan_projects(&config).unwrap();

    // Verify we found all projects
    assert_eq!(projects.len(), 3);

    // Test matcher with exact match
    let mut matcher = pj::matcher::Matcher::new();
    matcher.add_projects(projects.clone());
    let matches = matcher.find_matches("rust-project");
    assert_eq!(matches.len(), 1);
    assert!(matches[0].path.ends_with("rust-project"));

    // Test matcher with fuzzy match
    let mut matcher = pj::matcher::Matcher::new();
    matcher.add_projects(projects.clone());
    let matches = matcher.find_matches("rust");
    assert_eq!(matches.len(), 1);

    // Test matcher with partial nested path
    let mut matcher = pj::matcher::Matcher::new();
    matcher.add_projects(projects.clone());
    let matches = matcher.find_matches("cat/nest");
    assert_eq!(matches.len(), 1);
    assert!(matches[0].display_path.contains("nested-project"));

    // Test matcher with pattern matching multiple projects
    let mut matcher = pj::matcher::Matcher::new();
    matcher.add_projects(projects);
    let matches = matcher.find_matches("project");
    assert_eq!(matches.len(), 3);
}

#[test]
fn test_config_integration() {
    // Test that config can be serialized and deserialized
    let config = pj::config::Config {
        scan_paths: vec![PathBuf::from("/test/path1"), PathBuf::from("/test/path2")],
        project_markers: vec![".git".to_string(), "Cargo.toml".to_string()],
        max_depth: 4,
    };

    let toml_str = toml::to_string(&config).unwrap();
    let loaded_config: pj::config::Config = toml::from_str(&toml_str).unwrap();

    assert_eq!(config.scan_paths, loaded_config.scan_paths);
    assert_eq!(config.project_markers, loaded_config.project_markers);
    assert_eq!(config.max_depth, loaded_config.max_depth);
}

#[test]
fn test_deep_nested_project_scanning() {
    let temp_dir = TempDir::new().unwrap();

    // Create deeply nested structure
    let level1 = temp_dir.path().join("level1");
    let level2 = level1.join("level2");
    let level3 = level2.join("level3");
    let deep_project = level3.join("deep-project");

    fs::create_dir_all(&deep_project).unwrap();
    fs::create_dir(deep_project.join(".git")).unwrap();

    // Test with sufficient max_depth
    let config = pj::config::Config {
        scan_paths: vec![temp_dir.path().to_path_buf()],
        project_markers: vec![".git".to_string()],
        max_depth: 5,
    };

    let projects = pj::scanner::scan_projects(&config).unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(
        projects[0].display_path,
        "level1/level2/level3/deep-project"
    );

    // Test with insufficient max_depth
    let config = pj::config::Config {
        scan_paths: vec![temp_dir.path().to_path_buf()],
        project_markers: vec![".git".to_string()],
        max_depth: 3,
    };

    let projects = pj::scanner::scan_projects(&config).unwrap();
    assert_eq!(projects.len(), 0);
}

#[test]
fn test_multiple_marker_types() {
    let temp_dir = TempDir::new().unwrap();

    // Create projects with different markers
    let git_project = temp_dir.path().join("git-project");
    fs::create_dir(&git_project).unwrap();
    fs::create_dir(git_project.join(".git")).unwrap();

    let jj_project = temp_dir.path().join("jj-project");
    fs::create_dir(&jj_project).unwrap();
    fs::create_dir(jj_project.join(".jj")).unwrap();

    let cargo_project = temp_dir.path().join("cargo-project");
    fs::create_dir(&cargo_project).unwrap();
    fs::write(
        cargo_project.join("Cargo.toml"),
        "[package]\nname = \"test\"",
    )
    .unwrap();

    let npm_project = temp_dir.path().join("npm-project");
    fs::create_dir(&npm_project).unwrap();
    fs::write(npm_project.join("package.json"), "{}").unwrap();

    // Config with all marker types
    let config = pj::config::Config {
        scan_paths: vec![temp_dir.path().to_path_buf()],
        project_markers: vec![
            ".git".to_string(),
            ".jj".to_string(),
            "Cargo.toml".to_string(),
            "package.json".to_string(),
        ],
        max_depth: 2,
    };

    let projects = pj::scanner::scan_projects(&config).unwrap();
    assert_eq!(projects.len(), 4);

    // Verify all project types are found
    let display_paths: Vec<&str> = projects.iter().map(|p| p.display_path.as_str()).collect();
    assert!(display_paths.contains(&"git-project"));
    assert!(display_paths.contains(&"jj-project"));
    assert!(display_paths.contains(&"cargo-project"));
    assert!(display_paths.contains(&"npm-project"));
}

#[test]
fn test_matcher_with_many_projects() {
    let temp_dir = TempDir::new().unwrap();

    // Create many projects to test matching performance and correctness
    for i in 0..50 {
        let project = temp_dir.path().join(format!("project-{}", i));
        fs::create_dir(&project).unwrap();
        fs::create_dir(project.join(".git")).unwrap();
    }

    // Create a few special projects
    let special1 = temp_dir.path().join("my-awesome-app");
    fs::create_dir(&special1).unwrap();
    fs::create_dir(special1.join(".git")).unwrap();

    let special2 = temp_dir.path().join("my-other-app");
    fs::create_dir(&special2).unwrap();
    fs::create_dir(special2.join(".git")).unwrap();

    let config = pj::config::Config {
        scan_paths: vec![temp_dir.path().to_path_buf()],
        project_markers: vec![".git".to_string()],
        max_depth: 2,
    };

    let projects = pj::scanner::scan_projects(&config).unwrap();
    assert_eq!(projects.len(), 52);

    // Test specific fuzzy match
    let mut matcher = pj::matcher::Matcher::new();
    matcher.add_projects(projects);
    let matches = matcher.find_matches("awesome");

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].display_path, "my-awesome-app");
}

#[test]
fn test_nonexistent_scan_path_handling() {
    let config = pj::config::Config {
        scan_paths: vec![
            PathBuf::from("/this/path/does/not/exist"),
            PathBuf::from("/another/fake/path"),
        ],
        project_markers: vec![".git".to_string()],
        max_depth: 3,
    };

    // Should not panic, just return empty results
    let projects = pj::scanner::scan_projects(&config).unwrap();
    assert_eq!(projects.len(), 0);
}
