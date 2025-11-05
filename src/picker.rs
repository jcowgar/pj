use crate::scanner::Project;
use anyhow::Result;
use nucleo_picker::{Picker, render::StrRenderer};

pub struct InteractivePicker {
    projects: Vec<Project>,
}

impl InteractivePicker {
    pub fn new(projects: Vec<Project>) -> Self {
        Self { projects }
    }

    /// Show interactive picker and return selected project
    pub fn pick(&self) -> Result<Option<Project>> {
        if self.projects.is_empty() {
            return Ok(None);
        }

        let mut picker = Picker::new(StrRenderer);
        let injector = picker.injector();

        // Push all project paths to the picker
        for project in &self.projects {
            injector.push(project.display_path.clone());
        }

        // Show picker and get selection
        match picker.pick()? {
            Some(selected_path) => {
                // Find the project with matching display path
                let project = self
                    .projects
                    .iter()
                    .find(|p| p.display_path == *selected_path)
                    .cloned();
                Ok(project)
            }
            None => Ok(None),
        }
    }
}
