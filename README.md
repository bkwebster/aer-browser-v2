# 🌬️ Aer Browser

A modern, fast web browser built with Rust, Tauri, and React, powered by the Chromium Embedded Framework (CEF).

## Architecture

```
React UI (Controls) ←→ Tauri Bridge ←→ Rust Backend ←→ CEF Engine
                                              ↓
                                          DuckDB Storage
```

## Features

### ✅ Current Features
- Modern React-based UI with browser chrome
- Navigation controls (back, forward, refresh)
- Smart address bar with search integration
- Separate webview window system
- Cross-platform support (macOS, Windows, Linux)

### 🔄 In Development
- CEF (Chromium Embedded Framework) integration
- Embedded browser view within main window
- DuckDB integration for persistent storage

### 📋 Planned Features
- History and bookmarks management
- Tab system with session persistence
- Downloads manager
- Find-in-page functionality
- Basic password management
- Developer tools integration

## Tech Stack

- **Frontend**: React + TypeScript + Tailwind CSS
- **Build Tool**: Vite
- **Backend**: Rust + Tauri
- **Browser Engine**: CEF (Chromium Embedded Framework)
- **Database**: DuckDB
- **Package Manager**: Bun (preferred), npm (fallback)

## Development

### Prerequisites
- Rust 1.77.2+
- Node.js 18+
- Bun (recommended) or npm

### Setup
```bash
# Clone the repository
git clone <repository-url>
cd aer-browser

# Install UI dependencies
cd ui && bun install

# Build and run in development mode
bun dev                           # Start React dev server
cd ../src-tauri && cargo tauri dev  # Start Tauri app in dev mode
```

### Build for Production
```bash
cd ui && bun build                  # Build React app
cd ../src-tauri && cargo tauri build  # Build Tauri app
```

## Project Structure

```
aer-browser/
├── ui/                    # React frontend
│   ├── src/
│   │   ├── App.tsx       # Main browser UI
│   │   └── ...
│   ├── package.json
│   └── vite.config.ts
├── src-tauri/            # Rust backend
│   ├── src/
│   │   ├── lib.rs        # Main Tauri logic
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── WARP.md              # Session tracking & development notes
└── README.md
```

## Session Tracking

Development progress and session state are tracked in `WARP.md` for continuity across development sessions.

## Contributing

This is a personal project currently in early development. The architecture is designed to be:
- **Modular**: Clean separation between UI, backend, and browser engine
- **Extensible**: Easy to add new features and browser capabilities  
- **Local-first**: All data stored locally with DuckDB
- **Fast**: Built with performance in mind using Rust and CEF

## License

[License TBD]

---

**Status**: Early Development (Phase 2)  
**Last Updated**: 2025-09-02
