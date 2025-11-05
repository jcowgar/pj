// Library interface for pj - exposes modules for testing and potential reuse

pub mod config;
pub mod matcher;
pub mod scanner;

// Re-export key types for convenience
pub use config::Config;
pub use matcher::Matcher;
pub use scanner::{Project, scan_projects};
