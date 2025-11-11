# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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