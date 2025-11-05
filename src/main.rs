mod config;
mod matcher;
mod picker;
mod scanner;

use anyhow::{Context, Result};
use clap::Parser;
use config::Config;
use matcher::Matcher;
use picker::InteractivePicker;
use scanner::scan_projects;
use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pj")]
#[command(about = "Project Jump - Fast project directory jumper", long_about = None)]
struct Args {
    /// Pattern to match against project paths
    pattern: Option<String>,

    /// List all matches without interactive picker
    #[arg(short, long)]
    list: bool,

    /// Generate default config file
    #[arg(long)]
    init_config: bool,

    /// Set the previous directory (used by shell wrapper)
    #[arg(long, hide = true)]
    set_prev: Option<String>,
}

/// Check if we're in an interactive terminal by checking /dev/tty
fn is_interactive() -> bool {
    // Try to open /dev/tty - if successful, we're in an interactive terminal
    File::open("/dev/tty").is_ok()
}

/// Get the state directory for pj
fn state_dir() -> Result<PathBuf> {
    let state_dir = dirs::state_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".local/state")))
        .context("Could not determine state directory")?
        .join("pj");

    fs::create_dir_all(&state_dir)?;
    Ok(state_dir)
}

/// Get the path to the previous directory file
fn prev_dir_path() -> Result<PathBuf> {
    Ok(state_dir()?.join("prev_dir"))
}

/// Read the previous directory
fn read_prev_dir() -> Option<PathBuf> {
    let path = prev_dir_path().ok()?;
    fs::read_to_string(path)
        .ok()
        .map(|s| PathBuf::from(s.trim()))
}

/// Write the previous directory
fn write_prev_dir(dir: &str) -> Result<()> {
    let path = prev_dir_path()?;
    fs::write(path, dir)?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle setting previous directory
    if let Some(prev) = args.set_prev {
        write_prev_dir(&prev)?;
        return Ok(());
    }

    // Handle config initialization
    if args.init_config {
        let config_path = Config::create_default_config()?;
        println!("Created default config at: {}", config_path.display());
        return Ok(());
    }

    // Load configuration
    let config = Config::load()?;

    // Scan for projects
    let projects = scan_projects(&config)?;

    if projects.is_empty() {
        eprintln!("No projects found in configured scan paths");
        std::process::exit(1);
    }

    // Handle pattern matching
    if let Some(pattern) = args.pattern {
        // Special case: "pj -" jumps to previous directory
        if pattern == "-" {
            if let Some(prev) = read_prev_dir() {
                if prev.exists() {
                    println!("{}", prev.display());
                    return Ok(());
                } else {
                    eprintln!("Previous directory no longer exists: {}", prev.display());
                    std::process::exit(1);
                }
            } else {
                eprintln!("No previous directory stored");
                std::process::exit(1);
            }
        }

        let mut matcher = Matcher::new();
        matcher.add_projects(projects);
        let matches = matcher.find_matches(&pattern);

        match matches.len() {
            0 => {
                eprintln!("No matches found for: {}", pattern);
                std::process::exit(1);
            }
            1 => {
                // Single match - print the path
                println!("{}", matches[0].path.display());
            }
            _ => {
                // Multiple matches - show interactive picker or list
                if args.list || !is_interactive() {
                    // List mode or non-interactive - print all matches
                    for m in matches {
                        println!("{}", m.path.display());
                    }
                } else {
                    // Interactive mode - show picker
                    let picker = InteractivePicker::new(matches);
                    match picker.pick()? {
                        Some(project) => println!("{}", project.path.display()),
                        None => std::process::exit(1),
                    }
                }
            }
        }
    } else {
        // No pattern - show interactive picker or list all
        if args.list || !is_interactive() {
            // List mode or non-interactive - print all projects
            for project in projects {
                println!("{}", project.path.display());
            }
        } else {
            // Interactive mode - show picker
            let picker = InteractivePicker::new(projects);
            match picker.pick()? {
                Some(project) => println!("{}", project.path.display()),
                None => std::process::exit(1),
            }
        }
    }

    Ok(())
}
