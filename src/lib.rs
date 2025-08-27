//! # threeway-merge-rs
//!
//! A Rust library for 3-way string merging using Git's proven merge algorithms.
//!
//! ## Quick Start
//!
//! ```rust
//! use threeway_merge_rs::{merge_strings, MergeOptions, DiffAlgorithm, MergeStyle};
//!
//! let base = "Hello world";
//! let ours = "Hello Rust world";  
//! let theirs = "Hello beautiful world";
//!
//! let mut options = MergeOptions::default();
//! options.algorithm = DiffAlgorithm::Histogram;
//! options.style = MergeStyle::ZealousDiff3;
//! options.ours_label = Some("mine".to_string());
//! options.theirs_label = Some("theirs".to_string());
//!
//! let result = merge_strings(base, ours, theirs, &options)?;
//! println!("Merged content:\n{}", result.content);
//! println!("Conflicts: {}", result.conflicts);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Git Equivalent
//!
//! This is equivalent to running:
//! ```bash
//! git merge-file --diff-algorithm histogram --zdiff3 \
//!   -L "mine" ours_file -L "original" base_file -L "theirs" theirs_file --stdout
//! ```
//!
//! ## Supported Algorithms
//!
//! - **Myers**: Fast, general-purpose algorithm (default)
//! - **Minimal**: Produces smallest possible diff
//! - **Patience**: Good for code with moved blocks  
//! - **Histogram**: Improved version of Patience
//!
//! ## Merge Styles
//!
//! - **Normal**: Standard 2-way conflict markers
//! - **Diff3**: Shows base version in conflicts
//! - **ZealousDiff3**: More aggressive 3-way conflicts

mod ffi;
mod merge;
mod types;

pub use merge::merge_strings;
pub use types::*;
