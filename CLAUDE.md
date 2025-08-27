# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# threeway-merge-rs

## Project Overview
**threeway-merge-rs** is a Rust library for Git-style 3-way string merging using libgit2/xdiff C library via FFI.

### Core Architecture
- **String-based API**: Works with `&str` inputs, not files
- **FFI Wrapper**: Safe Rust interface around libgit2/xdiff C library
- **Memory Management**: Proper cleanup of C-allocated memory using `libc::free`
- **Build-time Compilation**: C library compiled via `build.rs` using `cc` crate

### Key Components
- `src/merge.rs` - Main `merge_strings()` function and high-level API
- `src/ffi.rs` - C FFI bindings and struct definitions for xdiff
- `src/types.rs` - Public Rust types (`MergeOptions`, `MergeResult`, enums)
- `src/xdiff/` - libgit2/xdiff C source code (compiled at build time)
- `build.rs` - Compiles C source files into static library

### Critical FFI Memory Safety
- CString lifetimes must extend through entire `xdl_merge` call
- All C-allocated memory must be freed with `libc::free()`
- String pointers passed to C must remain valid during function execution

## Development Commands

### Building and Testing
```bash
cargo build              # Build library
cargo test               # Run all tests  
cargo test --test basic_merge  # Run specific test file
cargo test test_name -- --nocapture  # Run single test with output
cargo run --example basic_usage      # Run file-based example
```

### Documentation
```bash
cargo doc                # Generate documentation
cargo test --doc         # Test documentation examples
```

### Git Comparison Testing
To verify our implementation matches Git's behavior:
```bash
cd examples/
git merge-file --diff-algorithm histogram --zdiff3 \
  -L "mine" ours.txt -L "original" base.txt -L "theirs" theirs.txt \
  --stdout > git_result.txt
```
Then compare with our `result.txt` output.

## Supported Git merge-file Options

### Algorithms (--diff-algorithm)
- `Myers` - Default algorithm (no flags)
- `Minimal` - Uses `XDF_NEED_MINIMAL` flag
- `Patience` - Uses `XDF_PATIENCE_DIFF` flag  
- `Histogram` - Uses `XDF_HISTOGRAM_DIFF` flag

### Styles
- `Normal` - Standard conflict markers
- `Diff3` - Shows base content (`XDL_MERGE_DIFF3`)
- `ZealousDiff3` - Aggressive diff3 (`XDL_MERGE_ZEALOUS_DIFF3`)

### Conflict Resolution (favor)
- `Ours` - `XDL_MERGE_FAVOR_OURS`
- `Theirs` - `XDL_MERGE_FAVOR_THEIRS`  
- `Union` - `XDL_MERGE_FAVOR_UNION`

## Implementation Notes

### libgit2/xdiff vs Git's xdiff
We use the standalone libgit2/xdiff instead of Git's xdiff to avoid Git-specific dependencies like regex and ignore patterns. Files in `src/xdiff/` are from https://github.com/libgit2/xdiff.

### Conflict Counting
Conflicts are counted by finding sets of conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`) and dividing by 3.

### Label Support
Custom labels work but may have formatting differences from Git due to xdiff implementation differences.