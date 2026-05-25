# zstbund – Offline Arch Package Bundler: Dependencies and Keys in One Go
 English | [中文](README.md) 

[![AUR version](https://img.shields.io/aur/version/zstbund)](https://aur.archlinux.org/packages/zstbund)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

Note: Using zstbund to bundle packages **does not** install any software on your system.

> For completely offline Arch Linux machines: one command to bundle the software you need, along with all its dependencies and verification signatures. Install everything with dependencies in one go even in an offline environment.

---

## Why zstbund?

When you want to install software on a **fully offline** Arch Linux machine:

- `pacman -Spw` only downloads packages, **without GPG keys** → offline verification fails
- Manually resolving the dependency chain? Too much hassle
- Downloading packages one by one and transferring them to the offline machine? Possible, but still requires manual packaging

**zstbund** = automatic dependency resolution + bundling all packages + collecting required GPG keys → produces a single zip file. Take it anywhere, you can either extract and install manually or let it install automatically:
```bash
zstbund install <zsts.zip> # automatic installation
```

---

## Installation

### Install from AUR (recommended)
```bash
yay -S zstbund
# or
git clone https://aur.archlinux.org/zstbund.git
cd zstbund
makepkg -si
```

---

## Usage

### Bundle dependencies into a package

```bash
zstbund bundle <single-or-multiple-packages-to-bundle>
```

### Install from a bundle

```bash
zstbund install <bundled-zip-file>
# or
unzip <bundled-zip-file> && pacman -U *.pkg.tar.zst
```

---

# To-Do

## Unfinished (priority from high to low)

- [ ] Bilingual help (Chinese & English)
- [ ] Add AUR support

## Completed

- [x] Bilingual README (Chinese & English)


This README is translated into English by AI and may not be up to date.