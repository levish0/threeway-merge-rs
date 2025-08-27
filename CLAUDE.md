# threeway-merge-rs

## Project Overview
**threeway-merge-rs** is a Rust library for 3-way string merging using Git's proven merge algorithms.

### Goals
- **Input**: 3 strings (base, ours, theirs)
- **Output**: Merged string + conflict information  
- **Distribution**: Public crates.io library
- **Compatibility**: Full Git merge algorithm and options support

### Key Features
- Pure string-based API (no file I/O)
- All Git merge algorithms: Myers, Patience, Histogram
- All Git merge styles: Normal, Diff3, ZealousDiff3  
- All conflict resolution: Ours, Theirs, Union
- Memory safe Rust wrapper around proven C implementation

## Project Structure
```
src/
├── lib.rs              # Public API exports
├── merge.rs             # High-level string merge functions
├── ffi.rs              # C FFI bindings (internal)
├── types.rs            # Public types and enums
├── error.rs            # Error handling
└── xdiff/              # libgit2/xdiff C source code

build.rs                # Compiles C code with cc crate
Cargo.toml              # Dependencies: cc (build-time only)
```

## Core API Design
```rust
// Main function
pub fn merge_strings(
    base: &str,
    ours: &str, 
    theirs: &str,
    options: &MergeOptions
) -> Result<MergeResult, MergeError>

// Result type
pub struct MergeResult {
    pub content: String,
    pub conflicts: usize,
}

// Configuration
pub struct MergeOptions {
    pub style: MergeStyle,
    pub favor: Option<MergeFavor>, 
    pub algorithm: DiffAlgorithm,
    pub marker_size: usize,
}
```

## Implementation Strategy
1. Use libgit2/xdiff C library internally (src/xdiff/)
2. Compile C code with cc crate in build.rs
3. Create safe FFI bindings (private module)
4. Wrap in high-level Rust API (public interface)
5. Handle all memory management safely
6. Provide Git-compatible results

## Development Commands
- `cargo build` - Compiles C code and Rust wrapper
- `cargo test` - Run tests against Git reference
- `cargo doc` - Generate documentation
- `cargo check` - Fast syntax checking

## Usage Example
```rust
use threeway_merge_rs::{merge_strings, MergeOptions};

let base = "Hello world";  
let ours = "Hello Rust world";
let theirs = "Hello beautiful world";

let result = merge_strings(base, ours, theirs, &MergeOptions::default())?;
println!("{}", result.content);
```

## Technical Notes
- C code is compiled at build time, users only see Rust API
- All Git merge functionality accessible through safe Rust interface
- No runtime C dependencies for end users
- Cross-platform compilation (Windows/Linux/macOS)