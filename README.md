# Desktop Pet

A cute desktop mascot application built with Tauri 2 and React.

## Features

- 🐾 **11 Animation States** - idle, walk, click, drag, sleep, wake, hover, happy, sad, surprised, wave
- 🎨 **Customizable Skins** - Load your own skins with JSON configuration
- 🖱️ **Interactive** - Click, drag, and hover interactions
- 🌙 **Auto Sleep** - Pet falls asleep after 30 seconds of inactivity
- 🔄 **Multi-Skin Support** - Switch between installed skins via system tray
- 📐 **Skin Validator** - Automatic validation of skin format and dimensions
- 🖼️ **System Tray** - Quick access menu for settings and skin switching

## Screenshots

<!-- Add screenshots here -->
*Coming soon*

## Installation

Download from [Releases](../../releases) (when available).

## Development

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

See [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) for detailed guide.

## Documentation

- [Agent Playbook](docs/agent-playbook.md) - Multi-agent workflow
- [Skin Authoring](docs/skin-authoring.md) - Create custom skins
- [Development Guide](docs/DEVELOPMENT.md) - Build instructions

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

## License

MIT