use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use anyhow::Result;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub lastuser: Option<String>,
    pub quiet: bool,
    pub nocolor: bool,
    pub editor: Option<String>,
}

impl Config {
    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ghk")
            .join("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|s| toml::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "quiet" => Some(self.quiet.to_string()),
            "nocolor" => Some(self.nocolor.to_string()),
            "editor" => self.editor.clone(),
            "lastuser" => self.lastuser.clone(),
            _ => None,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "quiet" => self.quiet = value == "true" || value == "1",
            "nocolor" => self.nocolor = value == "true" || value == "1",
            "editor" => self.editor = Some(value.to_string()),
            _ => anyhow::bail!("Unknown setting: {}", key),
        }
        self.save()
    }
}

// global flags
static mut QUIET: bool = false;
static mut NOCOLOR: bool = false;

pub fn setquiet(q: bool) {
    unsafe { QUIET = q; }
}

pub fn isquiet() -> bool {
    unsafe { QUIET }
}

pub fn setnocolor(c: bool) {
    unsafe { NOCOLOR = c; }
}

pub fn get_nocolor() -> bool {
    unsafe { NOCOLOR }
}

pub fn get_quiet() -> bool {
    unsafe { QUIET }
}

pub fn isnocolor() -> bool {
    unsafe { NOCOLOR }
}

/// Check if this is the first run
pub fn isfirstrun() -> bool {
    !Config::path().exists()
}
