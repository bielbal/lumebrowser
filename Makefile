.PHONY: all dev build release linux windows clean deps check help

CARGO := cargo
NPM   := npm
TAURI := $(NPM) run tauri

# ─── Targets ──────────────────────────────────────────────────────────────────

help:
	@echo "Lume Browser — Build System"
	@echo ""
	@echo "  make deps        Install all dependencies"
	@echo "  make dev         Start development server (hot-reload)"
	@echo "  make build       Debug build for current platform"
	@echo "  make release     Optimised release build"
	@echo "  make linux       Cross-compile release for Linux (x86_64)"
	@echo "  make windows     Cross-compile release for Windows (x86_64)"
	@echo "  make check       Run type-check + Cargo check"
	@echo "  make clean       Remove build artefacts"

deps:
	$(NPM) install
	cd src-tauri && $(CARGO) fetch

dev: deps
	$(TAURI) dev

build: deps
	$(TAURI) build --debug

release: deps
	$(TAURI) build

linux: deps
	$(TAURI) build --target x86_64-unknown-linux-gnu

windows: deps
	@rustup target add x86_64-pc-windows-msvc 2>/dev/null || true
	$(TAURI) build --target x86_64-pc-windows-msvc

check:
	$(NPM) run check
	cd src-tauri && $(CARGO) check

clean:
	rm -rf node_modules build .svelte-kit
	cd src-tauri && $(CARGO) clean
