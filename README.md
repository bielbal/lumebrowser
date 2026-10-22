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
make deps      # Install all dependencies
make dev       # Hot-reload dev server
make build     # Debug build
make release   # Optimised release
make linux     # Linux AppImage / .deb / .rpm
make windows   # Windows .msi / .exe
make check     # Type-check + Cargo check
make clean     # Remove build artefacts
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

## License

MIT
