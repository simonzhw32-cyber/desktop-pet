# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-05-10

### Added
- Initial release
- Tauri 2 + React + TypeScript scaffold
- Skin loading with rich-v1 profile (11 required states)
- Frame animation with loop/oneshot
- State machine for state transitions
- Mouse interactions: hover, click, drag
- Click-through transparent window
- Skin validator (format, size, PNG)
- Multi-skin support with settings
- System tray with context menu
- Unit tests (13 test cases)
- CI pipeline (lint/test/build/clippy)

### Known Issues
- CSP disabled (enable before production)
- Path traversal protection is basic
- Rust requires local compilation
