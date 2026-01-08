# ghk

Simple GitHub helper — push code without the complexity.

<p align="center">
  <a href="https://github.com/bymehul/ghk/releases"><img src="https://img.shields.io/github/v/release/bymehul/ghk?style=flat-square" alt="Release"></a>
  <a href="https://github.com/bymehul/ghk/blob/main/LICENSE"><img src="https://img.shields.io/github/license/bymehul/ghk?style=flat-square" alt="License"></a>
</p>

<p align="center">
  <a href="https://bymehul.github.io/ghk"><strong>📖 Documentation</strong></a> · 
  <a href="https://github.com/bymehul/ghk/releases"><strong>⬇️ Download</strong></a>
</p>

---

## Install

**No Rust required** — download and run!

```bash
# Linux/macOS (recommended)
curl -sSL https://raw.githubusercontent.com/bymehul/ghk/main/install.sh | bash

# Debian/Ubuntu
wget https://github.com/bymehul/ghk/releases/latest/download/ghk_1.0.0_amd64.deb
sudo dpkg -i ghk_*.deb

# Fedora/RHEL
sudo rpm -i https://github.com/bymehul/ghk/releases/latest/download/ghk-1.0.0-1.x86_64.rpm

# Windows (PowerShell)
irm https://raw.githubusercontent.com/bymehul/ghk/main/install.ps1 | iex

# From source (requires Rust)
cargo install --git https://github.com/bymehul/ghk
```

See [INSTALL.md](INSTALL.md) for detailed instructions.

## Quick Start

```bash
ghk setup      # Install git & gh if missing
ghk init       # Start tracking project
ghk create     # Create repo on GitHub
ghk push       # Save changes
```

## Commands

| Command | Alias | Purpose |
|---------|-------|---------|
| `setup` | | Install requirements |
| `init` | | Start tracking folder |
| `login` / `logout` | | GitHub auth |
| `create` | | Create repo on GitHub |
| `push` | `save` | Save changes |
| `pull` | `sync` | Download changes |
| `clone <repo>` | `download` | Download repo |
| `status` | | Show status |
| `diff` | | Preview changes |
| `history` | `log` | Show recent saves |
| `undo` | | Undo last commit |
| `open` | | Open in browser |
| `branch` | | List/switch branches |
| `ignore` | | Add .gitignore |
| `license` | | Add license file |
| `config` | | View/edit settings |
| `user list/switch` | | Manage accounts |
| `completions` | | Shell completions |

## Options

| Flag | Effect |
|------|--------|
| `-q, --quiet` | Suppress output |
| `--nocolor` | Disable colors |
| `-h, --help` | Show help |
| `-V, --version` | Show version |

## Shell Completions

```bash
# Bash
ghk completions bash >> ~/.bashrc

# Zsh  
ghk completions zsh > ~/.zfunc/_ghk

# Fish
ghk completions fish > ~/.config/fish/completions/ghk.fish
```

## Philosophy

- **No jargon** — "save" not "commit"
- **One command, one thing**
- **Always asks before destructive actions**
- **Works everywhere** — Linux, macOS, Windows

## License

MIT
