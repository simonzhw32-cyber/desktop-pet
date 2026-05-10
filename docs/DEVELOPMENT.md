# Development Guide

## Commands

```bash
# Install dependencies
pnpm install

# Type checking
pnpm lint

# Run tests (placeholder)
pnpm test

# Rust tests
cd src-tauri && cargo test

# Rust linter
cd src-tauri && cargo clippy

# Development server with hot reload
pnpm tauri dev

# Build frontend only
pnpm build

# Build Tauri application
pnpm tauri build

# Preview production build
pnpm preview
```

## Project Structure

```
desktop-pet/
├── src/                    # React frontend source
│   ├── App.tsx             # Main application component
│   ├── main.tsx            # Entry point
│   └── ...
├── src-tauri/              # Tauri (Rust) backend
│   ├── src/                # Rust source code
│   ├── tauri.conf.json     # Tauri configuration
│   ├── Cargo.toml          # Rust dependencies
│   └── icons/              # Application icons
├── public/                 # Static assets
├── docs/                   # Documentation
│   ├── agent-playbook.md   # Multi-agent workflow
│   ├── skin-authoring.md   # Skin specification
│   ├── SECURITY.md         # Security policy
│   └── work-items/         # Task tracking
├── .github/
│   └── workflows/
│       └── ci.yml          # CI pipeline
├── package.json            # Node.js dependencies
├── tsconfig.json           # TypeScript configuration
└── vite.config.ts          # Vite configuration
```

## Window Configuration

The main window is configured in `src-tauri/tauri.conf.json`:

- **Size**: 512x512 pixels
- **Transparent**: Yes (for transparent pet rendering)
- **Decorations**: No frameless window
- **Always on Top**: Yes (desktop pet behavior)
- **Skip Taskbar**: Yes (won't appear in taskbar)

## Adding New Dependencies

### Frontend (React/TypeScript)

```bash
pnpm add <package-name>
```

### Backend (Rust)

Edit `src-tauri/Cargo.toml` and add to `[dependencies]`.

## Debugging

- Frontend: Use browser DevTools (auto-opens in dev mode)
- Backend: Use `println!` or `log` crate for console output
- Tauri: Check `src-tauri/` for Rust errors during build

## Project Status

**Current Version:** 0.1.0

**Implemented Features:**
- ✅ Tauri 2 + React + TypeScript scaffold
- ✅ Skin loading and rendering
- ✅ 11 required states (rich-v1)
- ✅ State machine with frame animation
- ✅ Mouse interaction (hover/click/drag)
- ✅ Skin validator with format/size checks
- ✅ Multi-skin support with settings persistence
- ✅ System tray with context menu
- ✅ Unit tests (13 test cases)
- ✅ CI pipeline (lint/test/build/clippy)

**Known Issues:**
- Rust code requires local compilation (VM memory limited)
- CSP disabled (enable before production release)
- Path traversal check is basic (use canonicalize later)