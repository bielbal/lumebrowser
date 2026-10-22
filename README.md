# Lume Browser

A lightweight, modern browser built with **Tauri v2**, **SvelteKit** and **TypeScript**.  
Much lighter than Electron — native WebView, tiny binary, fast startup.

## Features

- Multi-tab browsing (child webviews per tab)
- Clean address bar with search (Brave Search default)
- Bookmarks & History panels
- New tab page with quick links + clock
- Find in page (`Ctrl+F`)
- Dark / light theme
- Keyboard-first: `Ctrl+T`, `Ctrl+W`, `Ctrl+R`, `Alt+←/→`, `Ctrl+1-9`

## Requirements

| Tool     | Version   |
|----------|-----------|
| Node.js  | ≥ 20      |
| Rust     | ≥ 1.77    |
| Tauri CLI| 2.x       |

**Linux extra deps:**
```bash
sudo apt install libwebkit2gtk-4.1-dev libxdo-dev \
  libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

## Build

```bash
# Install deps + start dev server
make dev

# Debug build
make build

# Optimised release
make release

# Linux AppImage / .deb / .rpm
make linux

# Windows .msi / .exe (requires cross-compilation target)
make windows

# Type-check
make check
```

Binaries land in `src-tauri/target/release/bundle/`.

## Architecture

```
lumebrowser/
├── src/                  # SvelteKit frontend (browser chrome)
│   ├── routes/+page.svelte   — main shell + keyboard shortcuts
│   └── lib/
│       ├── browser.ts         — Tauri API wrapper + URL sanitiser
│       ├── stores.ts          — reactive state (tabs, bookmarks, history)
│       └── components/
│           ├── TabBar.svelte
│           ├── AddressBar.svelte
│           ├── NewTabPage.svelte
│           └── Sidebar.svelte
└── src-tauri/            # Rust backend
    └── src/lib.rs        — tab lifecycle, navigation, resize handler
```

Each browser tab is a **Tauri child webview** (`WebviewBuilder`) positioned in the content area below the chrome.  
The SvelteKit shell (chrome) communicates with Rust via `invoke()` commands and listens to events via `listen()`.

## License

MIT
