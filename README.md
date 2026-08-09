<div align="center">

# threeway-merge-rs

Git-style 3-way string merging using proven algorithms from libgit2/xdiff.

[![Crates.io](https://img.shields.io/crates/v/threeway_merge.svg)](https://crates.io/crates/threeway_merge)
[![Documentation](https://docs.rs/threeway_merge/badge.svg)](https://docs.rs/threeway_merge)
[![Downloads](https://img.shields.io/crates/d/threeway_merge.svg)](https://crates.io/crates/threeway_merge)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.88.0+-orange.svg?logo=rust)](https://www.rust-lang.org/)

</div>

---

## 🌟 Overview

`threeway-merge-rs` is a Rust library for **string-based 3-way merging** using Git's proven merge algorithms.  
It uses **libgit2/xdiff** via safe FFI bindings, providing the same merge behavior as `git merge-file` but with a pure string API perfect for applications that need to merge text content programmatically.

---

## ✨ Features

- **String-based API**: Works with `&str` inputs, no file I/O required
- **Git-compatible**: Validated against `git merge-file` across 624 test combinations
- **Hardened FFI boundary**: Bounded native inputs and paired xdiff memory management
- **Conflict detection**: Automatic conflict counting and detailed output
- **No external native dependency**: xdiff is compiled and statically linked at build time
- **Comprehensive testing**: Multi-language scenarios with complex merge cases

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

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
threeway_merge = "0.1"
```

### Basic Usage

```rust
use threeway_merge::{merge_strings, MergeOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = "Hello world\nSecond line";
    let ours = "Hello Rust world\nSecond line"; 
    let theirs = "Hello beautiful world\nSecond line";

    let result = merge_strings(base, ours, theirs, &MergeOptions::default())?;
    
    println!("Conflicts: {}", result.conflicts);
    println!("Result:\n{}", result.content);
    
    Ok(())
}
```

### Advanced Configuration

```rust
use threeway_merge::{
    merge_strings, MergeOptions, DiffAlgorithm, MergeStyle, MergeFavor
};

let options = MergeOptions {
    algorithm: DiffAlgorithm::Histogram,
    style: MergeStyle::ZealousDiff3,
    favor: Some(MergeFavor::Union),
    base_label: Some("original".to_string()),
    ours_label: Some("mine".to_string()),
    theirs_label: Some("theirs".to_string()),
    marker_size: 10,
    ..MergeOptions::default()
};

let result = merge_strings(base, ours, theirs, &options)?;
```

### Git Equivalent

This Rust code:
```rust
let result = merge_strings(base, ours, theirs, &options)?;
```

Is equivalent to:
```bash
git merge-file --diff-algorithm histogram --zdiff3 \
  -L "mine" ours.txt -L "original" base.txt -L "theirs" theirs.txt --stdout
```

---

## 🧪 Testing & Compatibility

### Git Compatibility
This library continuously checks compatibility with `git merge-file`:
- **624 test combinations** across multiple scenarios
- **13 complex merge scenarios** including:
  - Multi-language text (Korean, Japanese, French)
  - Programming code (JavaScript, Rust, Python, SQL)
  - Whitespace edge cases and deeply nested conflicts
  - Literature excerpts and legal documents

### Running Tests
```bash
# Run the fast, self-contained test suite
cargo test

# Run the full compatibility matrix (requires Git)
cargo test --test comprehensive_git_comparison -- --ignored

# Run with output visible
cargo test -- --nocapture
```

### Publishing (Maintainers)
```bash
# Validate publish artifacts without uploading
cargo xtask publish-dry

# Publish to crates.io
cargo xtask publish
```

### Performance
- Fast paths avoid native diff work for obvious clean merges
- Native output buffers are released by the same allocator that created them
- Build-time compilation avoids a separately installed xdiff library

---

## 🏗️ Requirements

- **Rust**: 1.88.0 or later (uses 2024 edition)
- **C compiler**: For build-time compilation of xdiff library
- **Git** (optional): Only required for the ignored compatibility test matrix

---

## 🙏 Acknowledgments

### Core Technologies
- [**Rust**](https://www.rust-lang.org/) – Systems programming language with safety and performance
- [**libgit2/xdiff**](https://github.com/libgit2/xdiff) – Standalone version of Git's xdiff library
- [**cc crate**](https://github.com/rust-lang/cc-rs) – Build-time C compilation

### Special Thanks
- **Open Source Community**: For the incredible tools and libraries
- **Contributors**: Everyone who improves `threeway-merge-rs`
- **Davide Libenzi**: Original author of xdiff
- **libgit2 team**: For maintaining the standalone xdiff version

---

## 📜 License

The Rust wrapper is licensed under the **MIT License**.

### Third-Party Code: xdiff Library

This crate statically links the **xdiff library** (located in `src/xdiff/`), which is licensed under the **GNU Lesser General Public License v2.1 or later (LGPL-2.1+)**.

The vendored xdiff snapshot is derived from [libgit2/xdiff](https://github.com/libgit2/xdiff) and carries a small standalone portability patch recorded in [`src/xdiff/git-2.48.1.patch`](src/xdiff/git-2.48.1.patch).

#### LGPL Compliance

The crate includes the xdiff source so users can modify it and rebuild the crate:

1. Modify the C source code in `src/xdiff/`
2. Rebuild the crate: `cargo build`

Distributors of binaries that statically link this crate remain responsible for the applicable LGPL-2.1-or-later requirements, including the relinking provisions in section 6.

See [`src/xdiff/COPYING`](src/xdiff/COPYING) for the full LGPL license text.
