# zstbund – 离线打包 Arch 包，依赖和密钥一键带走
[English](README-en.md) | 中文

[![AUR version](https://img.shields.io/aur/version/zstbund)](https://aur.archlinux.org/packages/zstbund)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

注：使用zstbund打包软件包**不会**在系统中安装任何软件

> 为无网络的 Arch Linux 机器而生：一条命令打包你需要安装的软件，连同它的所有依赖和验证签名，离线环境也能一键带依赖安装。

---

## 为什么需要 zstbund？

当你在**完全离线**的 Arch Linux 机器上安装软件时：

- `pacman -Spw` 只能下载包，**不带 GPG 密钥** → 离线验证失败
- 手动找依赖链？超麻烦
- 逐一下载再传给离线机？能装，但是还得手动压缩

**zstbund** = 自动解析依赖 + 同时打包所有包 + 打包所需 GPG 密钥 → 生成一个 zip，拿走即可，可解压手动安装也可自动安装
```bash
zstbund install <zsts.zip> # 自动安装
```

---

## 安装

### 从 AUR 安装（推荐）
```bash
yay -S zstbund
# 或
git clone https://aur.archlinux.org/zstbund.git
cd zstbund
makepkg -si
```

---

## 使用

### 保存依赖成压缩包

```bash
zstbund bundle 需要打包的单个（或多个）包
```

### 安装压缩包

```bash
zstbund install 打包出来的压缩包
# 或
unzip 打包出来的压缩包 && pacman -U *.pkg.tar.zst
```

---

# 代办

## 未完成（优先级从高到低排序）

- [ ] 中英双语help
- [ ] 增加aur支持

## 已完成

- [x] 中英双语README