use crate::scanner::Project;
use nucleo::{Config as NucleoConfig, Nucleo, Utf32String};
use std::sync::Arc;

pub struct Matcher {
    nucleo: Nucleo<Project>,
}

impl Default for Matcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Matcher {
    pub fn new() -> Self {
        let nucleo = Nucleo::new(
            NucleoConfig::DEFAULT,
            Arc::new(|| {}),
            None,
            1, // number of threads
        );

        Self { nucleo }
    }

    /// Add projects to the matcher
    pub fn add_projects(&mut self, projects: Vec<Project>) {
        let injector = self.nucleo.injector();

        for project in projects {
            injector.push(project, |proj, cols| {
                cols[0] = Utf32String::from(proj.display_path());
            });
        }
    }

    /// Perform fuzzy matching and return sorted results
    pub fn find_matches(&mut self, pattern: &str) -> Vec<Project> {
        // Set the pattern
        self.nucleo.pattern.reparse(
            0,
            pattern,
            nucleo::pattern::CaseMatching::Smart,
            nucleo::pattern::Normalization::Smart,
            false,
        );

        // Tick to process matches
        self.nucleo.tick(10);

        // Get the snapshot of matches
        let snapshot = self.nucleo.snapshot();

        // Collect matched items
        snapshot
            .matched_items(..snapshot.matched_item_count())
            .map(|item| item.data.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_project(path: &str, display_path: &str) -> Project {
        Project {
            path: PathBuf::from(path),
            display_path: display_path.to_string(),
        }
    }

    #[test]
    fn test_matcher_exact_match() {
        let mut matcher = Matcher::new();
        let projects = vec![
            create_test_project("/home/user/projects/myapp", "myapp"),
            create_test_project("/home/user/projects/otherapp", "otherapp"),
        ];

        matcher.add_projects(projects);
        let matches = matcher.find_matches("myapp");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].display_path, "myapp");
    }

    #[test]
    fn test_matcher_fuzzy_match() {
        let mut matcher = Matcher::new();
        let projects = vec![
            create_test_project("/home/user/projects/my-awesome-app", "my-awesome-app"),
            create_test_project("/home/user/projects/other-app", "other-app"),
        ];

        matcher.add_projects(projects);
        let matches = matcher.find_matches("maa");

        // Should match "my-awesome-app" with fuzzy matching
        assert!(matches.iter().any(|p| p.display_path == "my-awesome-app"));
    }

    #[test]
    fn test_matcher_case_insensitive() {
        let mut matcher = Matcher::new();
        let projects = vec![
            create_test_project("/home/user/projects/MyApp", "MyApp"),
            create_test_project("/home/user/projects/other", "other"),
        ];

        matcher.add_projects(projects);
        let matches = matcher.find_matches("myapp");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].display_path, "MyApp");
    }

    #[test]
    fn test_matcher_nested_path_match() {
        let mut matcher = Matcher::new();
        let projects = vec![
            create_test_project("/home/user/projects/ai/decree-ng", "ai/decree-ng"),
            create_test_project("/home/user/projects/web/app", "web/app"),
        ];

        matcher.add_projects(projects);
        let matches = matcher.find_matches("ai/dec");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].display_path, "ai/decree-ng");
    }

    #[test]
    fn test_matcher_multiple_matches() {
        let mut matcher = Matcher::new();
        let projects = vec![
            create_test_project("/home/user/projects/app1", "app1"),
            create_test_project("/home/user/projects/app2", "app2"),
            create_test_project("/home/user/projects/app3", "app3"),
            create_test_project("/home/user/projects/other", "other"),
        ];

        matcher.add_projects(projects);
        let matches = matcher.find_matches("app");

        assert_eq!(matches.len(), 3);
        assert!(matches.iter().all(|p| p.display_path.contains("app")));
    }

    #[test]
    fn test_matcher_no_matches() {
        let mut matcher = Matcher::new();
        let projects = vec![
            create_test_project("/home/user/projects/app1", "app1"),
            create_test_project("/home/user/projects/app2", "app2"),
        ];

        matcher.add_projects(projects);
        let matches = matcher.find_matches("nonexistent");

        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_matcher_empty_pattern() {
        let mut matcher = Matcher::new();
        let projects = vec![
            create_test_project("/home/user/projects/app1", "app1"),
            create_test_project("/home/user/projects/app2", "app2"),
        ];

        matcher.add_projects(projects);
        let matches = matcher.find_matches("");

        // Empty pattern should match all projects
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_matcher_partial_path_match() {
        let mut matcher = Matcher::new();
        let projects = vec![
            create_test_project("/home/user/projects/rust/my-project", "rust/my-project"),
            create_test_project("/home/user/projects/go/my-project", "go/my-project"),
            create_test_project("/home/user/projects/js/other", "js/other"),
        ];

        matcher.add_projects(projects);
        let matches = matcher.find_matches("rust");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].display_path, "rust/my-project");
    }

    #[test]
    fn test_matcher_acronym_match() {
        let mut matcher = Matcher::new();
        let projects = vec![
            create_test_project(
                "/home/user/projects/my-awesome-project",
                "my-awesome-project",
            ),
            create_test_project("/home/user/projects/other", "other"),
        ];

        matcher.add_projects(projects);
        let matches = matcher.find_matches("map");

        // Should match "my-awesome-project" via acronym fuzzy matching
        assert!(
            matches
                .iter()
                .any(|p| p.display_path == "my-awesome-project")
        );
    }
}
