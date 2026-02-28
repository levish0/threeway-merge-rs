# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.16] - 2026-02-28

### Changed
- Improved merge performance for trivial clean-merge cases by adding early-return fast paths in `merge_strings`:
  - Return `ours` immediately when `ours == theirs`
  - Return `theirs` immediately when `ours == base`
  - Return `ours` immediately when `theirs == base`
- Kept existing input validation behavior unchanged (label NUL checks and `marker_size` range checks still run before merge execution).

## [0.1.15] - 2026-02-13

### Changed
- Improved Git compatibility tests to avoid false positives:
  - Added a Git availability pre-check
  - Interpreted `git merge-file` exit codes as conflict counts (0..=127)
  - Removed pass-on-error branches for Git invocation failures
  - Tightened output normalization to line-ending normalization only
  - Upgraded assertion criteria to require zero failing compatibility cases

### Fixed
- Hardened FFI boundary handling in `merge_strings`:
  - Added checked conversions for input sizes (`usize` -> `c_long`)
  - Added checked conversion for `marker_size` (`usize` -> `c_int`)
  - Added explicit guards for invalid C result buffer states

### Added
- Added regression tests for input validation:
  - Reject labels containing NUL bytes
  - Reject `marker_size` values above `c_int::MAX`

### Metadata
- Removed `license-file` from `Cargo.toml` and kept SPDX `license = "MIT"` to eliminate Cargo license metadata warnings.

## [0.1.10] - 2025-01-11

### Changed
- **License documentation**: Added comprehensive license information to README and Cargo.toml
  - Explicitly documented that this crate statically links xdiff (LGPL-2.1+)
  - Added LGPL compliance section explaining relinking requirements
  - Updated package description to mention LGPL xdiff dependency
  - Added `license-file` field to Cargo.toml pointing to MIT LICENSE

### Documentation
- Added "License" section to README with detailed explanation of xdiff LGPL usage
- Clarified that users can relink against modified xdiff by rebuilding from source
- Referenced `src/xdiff/COPYING` for full LGPL license text
