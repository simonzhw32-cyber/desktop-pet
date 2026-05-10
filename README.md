# Desktop Pet

A cross-platform desktop pet application built with Tauri 2, React, and TypeScript.

## Features

- 🐱 Animated desktop pet with transparent, frameless, always-on-top window
- 🎨 Extensible skin system (see [Skin Authoring Guide](docs/skin-authoring.md))
- 🔒 Privacy-first design (see [Security Policy](docs/SECURITY.md))

## Tech Stack

- **Frontend**: React 19 + TypeScript + Vite
- **Backend**: Tauri 2 (Rust)
- **Package Manager**: pnpm

## Quick Start

### Prerequisites

- Node.js 18+
- pnpm 8+
- Rust (for Tauri backend)
- Platform-specific dependencies: [Tauri Prerequisites](https://tauri.app/start/prerequisites/)

### Development

```bash
# Install dependencies
pnpm install

# Start development server
pnpm tauri dev
```

### Build

```bash
# Build for production
pnpm tauri build
```

## Documentation

- [Development Guide](docs/DEVELOPMENT.md) - Local development commands and project structure
- [Agent Playbook](docs/agent-playbook.md) - Multi-agent collaboration workflow
- [Skin Authoring](docs/skin-authoring.md) - Create custom pet skins
- [Security Policy](docs/SECURITY.md) - Privacy and security guidelines

## License

MIT