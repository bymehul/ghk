use anyhow::Result;
use dialoguer::Confirm;
use which::which;
use std::process::Command;

use crate::{util, gh, git};

pub fn run() -> Result<()> {
    println!();
    util::info("Checking your setup...");
    println!();

    // Check git
    if which("git").is_ok() {
        util::ok("Git installed");
        if let Some(v) = git::version() {
            util::dim(&format!("  {}", v));
        }
    } else {
        util::warn("Git not found");
        installtool("git")?;
    }

    // Check gh
    if which("gh").is_ok() {
        util::ok("GitHub CLI installed");
        if let Some(v) = gh::version() {
            util::dim(&format!("  {}", v));
        }
    } else {
        util::warn("GitHub CLI not found");
        installtool("gh")?;
    }

    // Check login
    if gh::loggedin() {
        let user = gh::whoami().unwrap_or_else(|_| "unknown".to_string());
        util::ok(&format!("Logged in as {}", user));
    } else {
        util::warn("Not logged in to GitHub");
        if Confirm::new()
            .with_prompt("Login now?")
            .default(true)
            .interact()? 
        {
            gh::login()?;
            if gh::loggedin() {
                util::ok("Connected!");
            }
        } else {
            util::dim("Skipped - run 'ghk login' later");
        }
    }

    // Check SSH
    if gh::loggedin() {
        if gh::hassshkey() {
            util::ok("SSH key configured");
        } else {
            util::warn("No SSH key found");
            util::dim("You can still use HTTPS, but SSH is recommended");
            util::dim("Run 'gh ssh-key add' to add your SSH key");
        }
    }

    // Check online
    if gh::isonline() {
        util::ok("Can reach GitHub");
    } else {
        util::warn("Cannot reach GitHub");
        util::dim("Check your internet connection");
    }

    println!();
    util::ok("All set!");
    util::dim("Run 'ghk --help' to see available commands");
    println!();
    Ok(())
}

/* ---------- helpers ---------- */

fn installtool(tool: &str) -> Result<()> {
    if !Confirm::new()
        .with_prompt(&format!("Install {} now?", tool))
        .default(true)
        .interact()? 
    {
        util::dim("Skipped");
        return Ok(());
    }

    let os = std::env::consts::OS;

    match os {
        "linux" => installonlinux(tool),
        "macos" => runpkg("brew", &["install", tool]),
        "windows" => {
            let id = match tool {
                "git" => "Git.Git",
                "gh" => "GitHub.cli",
                _ => tool,
            };
            runpkg("winget", &["install", "--id", id, "-e"])
        }
        _ => {
            util::warn(&format!("Please install {} manually", tool));
            Ok(())
        }
    }
}

fn installonlinux(tool: &str) -> Result<()> {
    let pm = detectpm();
    
    match pm.as_deref() {
        Some("apt") => runsudo(&["apt", "install", "-y", tool]),
        Some("dnf") => runsudo(&["dnf", "install", "-y", tool]),
        Some("pacman") => runsudo(&["pacman", "-S", "--noconfirm", tool]),
        Some("zypper") => runsudo(&["zypper", "install", "-y", tool]),
        _ => {
            util::warn("Could not detect package manager");
            util::dim("Please install manually:");
            util::dim(&format!("  sudo apt install {}", tool));
            util::dim(&format!("  sudo dnf install {}", tool));
            util::dim(&format!("  sudo pacman -S {}", tool));
            Ok(())
        }
    }
}

fn detectpm() -> Option<String> {
    let managers = ["apt", "dnf", "pacman", "zypper", "apk"];
    for pm in managers {
        if which(pm).is_ok() {
            return Some(pm.to_string());
        }
    }
    None
}

fn runsudo(args: &[&str]) -> Result<()> {
    if which("sudo").is_err() {
        util::warn("sudo not found");
        util::dim("Please run as root or install sudo");
        util::dim(&format!("  {}", args.join(" ")));
        return Ok(());
    }

    // Check if running as root (Unix only)
    #[cfg(unix)]
    let is_root = unsafe { libc::geteuid() } == 0;
    #[cfg(not(unix))]
    let is_root = false;
    
    let status = if is_root {
        Command::new(args[0]).args(&args[1..]).status()
    } else {
        util::dim("This requires admin access...");
        Command::new("sudo").args(args).status()
    };

    match status {
        Ok(s) if s.success() => {
            util::ok(&format!("{} installed", args.last().unwrap_or(&"")));
            Ok(())
        }
        Ok(_) => {
            util::warn("Install failed - try running manually");
            Ok(())
        }
        Err(e) => {
            util::warn(&format!("Could not run command: {}", e));
            Ok(())
        }
    }
}

fn runpkg(cmd: &str, args: &[&str]) -> Result<()> {
    if which(cmd).is_err() {
        util::warn(&format!("{} not found", cmd));
        match cmd {
            "brew" => util::dim("Install Homebrew from https://brew.sh"),
            "winget" => util::dim("winget should be pre-installed on Windows 10+"),
            _ => {}
        }
        return Ok(());
    }

    let status = Command::new(cmd).args(args).status();
    
    match status {
        Ok(s) if s.success() => {
            util::ok("Installed");
            Ok(())
        }
        Ok(_) => {
            util::warn("Install failed");
            Ok(())
        }
        Err(e) => {
            util::warn(&format!("Error: {}", e));
            Ok(())
        }
    }
}
