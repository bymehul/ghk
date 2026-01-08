use anyhow::Result;
use crate::cli::Commands;
use crate::config;

pub fn run(cli: crate::cli::Cli) -> Result<()> {
    // Set global flags
    config::setquiet(cli.quiet);
    config::setnocolor(cli.nocolor);

    if !cli.quiet && config::isfirstrun() {
        welcome();

        // Save config to mark first run complete
        let cfg = config::Config::default();
        if let Err(_e) = cfg.save() {
            #[cfg(debug_assertions)]
            eprintln!("Debug: Failed to save config: {}", _e);
        }
    }

    match cli.command {
        Commands::Init => crate::commands::init::run(),
        Commands::Login => crate::commands::login::run(),
        Commands::Logout => crate::commands::logout::run(),
        Commands::User(u) => crate::commands::user::run(u),
        Commands::Create => crate::commands::create::run(),
        Commands::Push | Commands::Save => crate::commands::push::run(),
        Commands::Pull | Commands::Sync => crate::commands::pull::run(),
        Commands::Clone { repo, dir } | Commands::Download { repo, dir } => {
            crate::commands::clone::run(repo, dir)
        }
        Commands::Status => crate::commands::status::run(),
        Commands::Setup => crate::commands::setup::run(),
        Commands::Undo => crate::commands::undo::run(),
        Commands::History { count } | Commands::Log { count } => {
            crate::commands::history::run(count)
        }
        Commands::Open => crate::commands::open::run(),
        Commands::Diff => crate::commands::diff::run(),
        Commands::Config { key, value } => crate::commands::config::run(key, value),
        Commands::Ignore { template } => crate::commands::ignore::run(template),
        Commands::License { kind } => crate::commands::license::run(kind),
        Commands::Branch { name } => crate::commands::branch::run(name),
        Commands::Completions { shell } => crate::commands::completions::run(shell),
    }
}

#[cfg(not(debug_assertions))]
fn welcome() {
    use std::io::{self, Write};

    if !config::get_nocolor() {
        println!("\n  \x1b[1m\x1b[36mWelcome to ghk!\x1b[0m");
        println!("\n  Simple GitHub helper - push code without the complexity");
        println!("\n  Quick start:");
        println!("    \x1b[90mghk setup\x1b[0m    Check requirements");
        println!("    \x1b[90mghk init\x1b[0m     Start tracking a project");
        println!("    \x1b[90mghk create\x1b[0m   Create repo on GitHub");
        println!("    \x1b[90mghk push\x1b[0m     Save your changes");
        println!("\n  Run \x1b[90mghk --help\x1b[0m for all commands\n");
    } else {
        // without ansi colors
        println!("\n  Welcome to ghk!");
        println!("\n  Simple GitHub helper - push code without the complexity");
        println!("\n  Quick start:");
        println!("    ghk setup    Check requirements");
        println!("    ghk init     Start tracking a project");
        println!("    ghk create   Create repo on GitHub");
        println!("    ghk push     Save your changes");
        println!("\n  Run ghk --help for all commands\n");
    }

    let _ = io::stdout().flush();
}

#[cfg(debug_assertions)]
fn welcome() {
    use std::io::{self, Write};

    println!("\n  =========================================");
    println!("  \x1b[1m\x1b[36mWelcome to ghk! (Development Build)\x1b[0m");
    println!("  =========================================");
    println!();
    println!("  Simple GitHub helper - push code without the complexity");
    println!("  Build: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("  Quick start:");
    println!("    \x1b[90mghk setup\x1b[0m    Check requirements");
    println!("    \x1b[90mghk init\x1b[0m     Start tracking a project");
    println!("    \x1b[90mghk create\x1b[0m   Create repo on GitHub");
    println!("    \x1b[90mghk push\x1b[0m     Save your changes");
    println!();
    println!("  Debug commands:");
    println!("    \x1b[90mghk --verbose\x1b[0m     Show detailed logs");
    println!("    \x1b[90mghk --dry-run\x1b[0m     Test commands");
    println!();
    println!("  Run \x1b[90mghk --help\x1b[0m for all commands");
    println!("  =========================================\n");

    // for debug
    let _ = io::stdout().flush();
}
