use anyhow::{Result, bail};
use dialoguer::Input;
use crate::{git, gh, util};

pub fn run() -> Result<()> {
    // Check prerequisites
    if !git::isrepo() {
        util::err("Not a git repository");
        util::dim("Run 'ghk init' first");
        bail!("Not a git repository");
    }

    if !gh::loggedin() {
        util::err("Not logged in to GitHub");
        util::dim("Run 'ghk login' first");
        bail!("Not logged in");
    }

    if !git::hasremote() {
        util::err("Not connected to GitHub");
        util::dim("Run 'ghk create' first to create a repository");
        bail!("No remote configured");
    }

    // Check for changes
    let changes = git::haschanges()?;
    if !changes {
        util::ok("Already up to date");
        util::dim("No changes to save");
        return Ok(());
    }

    // Safety check for scary files (sensitive or large temp files)
    let files = git::changedfiles()?;
    let scary_patterns = [
        ".env", "node_modules", "target", "dist", "venv", ".venv",
        "vendor", ".DS_Store", "thumbs.db", "__pycache__"
    ];

    let mut found_scary = Vec::new();
    for file in &files {
        for pattern in &scary_patterns {
            if file.contains(pattern) {
                found_scary.push(file.clone());
                break;
            }
        }
    }

    if !found_scary.is_empty() {
        util::warn("Wait! Potential sensitive or temporary files detected:");
        for file in found_scary.iter().take(5) {
            util::dim(&format!("  {}", file));
        }
        if found_scary.len() > 5 {
            util::dim(&format!("  ... and {} more", found_scary.len() - 5));
        }
        
        if !std::path::Path::new(".gitignore").exists() {
            util::info("Tip: You don't have a .gitignore file.");
            util::dim("Run 'ghk ignore' to add a template for your project.");
        }

        let proceed = dialoguer::Confirm::new()
            .with_prompt("Are you sure you want to save these files?")
            .default(false)
            .interact()?;
        
        if !proceed {
            util::info("Cancelled. Clean up your files or add them to .gitignore.");
            return Ok(());
        }
        println!(); // Add space before showing other changes
    }

    // Show what will be saved
    util::info("Changes to save:");
    for file in files.iter().take(10) {
        util::dim(&format!("  {}", file));
    }
    if files.len() > 10 {
        util::dim(&format!("  ... and {} more", files.len() - 10));
    }

    // Get commit message
    let msg: String = Input::new()
        .with_prompt("What did you change?")
        .default("Update".to_string())
        .interact_text()?;

    // Stage, commit, push
    util::info("Saving...");
    git::addall()?;
    git::commit(&msg)?;
    git::push()?;

    util::ok("Saved to GitHub!");
    Ok(())
}
