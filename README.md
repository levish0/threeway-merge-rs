<div align="center">
  <img src="./assets/banner.png" alt="3way merge"/>

# threeway-merge-rs

Git-style 3-way merge library in Rust with LibXDiff FFI.

[![License](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.86.0+-orange.svg?logo=rust)](https://www.rust-lang.org/)

</div>

---

## 🌟 Overview

`threeway-merge-rs` is a Rust library for performing **Git-style 3-way merges**.  
It leverages **LibXDiff** (LGPL) via FFI for diffing functionality while providing a safe and ergonomic Rust wrapper.  
The library can detect conflicts, generate conflict hunks, and supports multiple diffing and merging algorithms.

---

## ✨ Features

- Perform 3-way merges on text files
- Detect conflicts and generate conflict hunks
- Provides a **Rust FFI interface** to [LibXDiff](http://xdiff.sourceforge.net/)
- Lightweight and fast
- Fully MIT-licensed Rust wrapper

### Configurable Merge Options

#### Diff Algorithm
- `Myers` – Default, standard diff
- `Minimal` – Minimal changes
- `Patience` – Patience diff algorithm
- `Histogram` – Histogram-based diff

#### Merge Level
- `Minimal` – Conservative merging
- `Eager` – Slightly more aggressive
- `Zealous` – Aggressive merge
- `ZealousAlnum` – Aggressive with alphanumeric heuristics

#### Merge Favor
- `None` – No automatic favor
- `Ours` – Prefer "ours" changes
- `Theirs` – Prefer "theirs" changes
- `Union` – Combine changes when possible

#### Merge Style
- `Normal` – Default 3-way merge
- `Diff3` – Include base lines in conflicts
- `ZealousDiff3` – Diff3 style with aggressive merging

#### Conflict Markers
- Customize marker labels and sizes (`<<<<<<<`, `=======`, `>>>>>>>`)

---

## 🙏 Acknowledgments

### Core Technologies
- [**Rust**](https://www.rust-lang.org/) – Systems programming language with safety and performance
- [**LibXDiff**](http://xdiff.sourceforge.net/) – C library for file diffing (LGPL 2.1+), used via FFI

### Special Thanks
- **Open Source Community** – For the incredible tools and libraries
- **Contributors** – Everyone who improves `threeway-merge-rs`
- **Davide Libenzi** – Original author of LibXDiff

---

<div align="center">
**Made with ♥️ and lots of ☕ by levish (Levi Lim) & Community**
</div>
