# Installing ghk

## Quick Install (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/bymehul/ghk/main/install.sh | bash
```

## Package Managers

### Debian/Ubuntu (.deb)
```bash
# Download latest .deb from releases
wget https://github.com/bymehul/ghk/releases/latest/download/ghk_VERSION_amd64.deb
sudo dpkg -i ghk_VERSION_amd64.deb
```

### Fedora/RHEL (.rpm)
```bash
# Download latest .rpm from releases
wget https://github.com/bymehul/ghk/releases/latest/download/ghk-VERSION-1.x86_64.rpm
sudo rpm -i ghk-VERSION-1.x86_64.rpm
```

### Arch Linux (AUR)
```bash
# If you publish to AUR
yay -S ghk
# or
paru -S ghk
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/bymehul/ghk/main/install.ps1 | iex
```

Or download `ghk-windows-x86_64.exe` from [releases](https://github.com/bymehul/ghk/releases), rename to `ghk.exe`, and add to PATH.

## From Source

```bash
# Requires Rust
cargo install --git https://github.com/bymehul/ghk
```

## Manual Download

1. Go to [Releases](https://github.com/bymehul/ghk/releases)
2. Download binary for your platform:
   - `ghk-linux-x86_64` - Linux (Intel/AMD)
   - `ghk-linux-aarch64` - Linux (ARM64)
   - `ghk-macos-x86_64` - macOS (Intel)
   - `ghk-macos-aarch64` - macOS (Apple Silicon)
   - `ghk-windows-x86_64.exe` - Windows
3. Make executable: `chmod +x ghk-*`
4. Move to PATH: `mv ghk-* ~/.local/bin/ghk`

## Verify Download

```bash
# Check SHA256
sha256sum -c checksums.txt
```

---

# Creating a Release

## Steps

1. **Update version** in `Cargo.toml`

2. **Update CHANGELOG.md**

3. **Tag and push**
   ```bash
   git add -A
   git commit -m "Release v1.0.0"
   git tag v1.0.0
   git push origin main --tags
   ```

4. **GitHub Actions** will automatically:
   - Build binaries for all platforms
   - Create .deb package (Debian/Ubuntu)
   - Create .rpm package (Fedora/RHEL)
   - Upload to GitHub Releases
   - Generate checksums

## Publishing to AUR (Arch Linux)

Create `PKGBUILD`:
```bash
pkgname=ghk
pkgver=1.0.0
pkgrel=1
pkgdesc="Simple GitHub helper"
arch=('x86_64')
url="https://github.com/bymehul/ghk"
license=('MIT')
source=("$url/releases/download/v$pkgver/ghk-linux-x86_64")
sha256sums=('CHECKSUM_HERE')

package() {
    install -Dm755 ghk-linux-x86_64 "$pkgdir/usr/bin/ghk"
}
```

Submit to AUR.

