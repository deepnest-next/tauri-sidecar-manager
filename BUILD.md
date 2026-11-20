# Building tauri-sidecar-manager

The plugin links against Tauri 2, so it needs Tauri's system libraries in addition to a Rust
toolchain.

## External dependencies

| Platform | Install |
|---|---|
| Debian/Ubuntu | `sudo apt install build-essential pkg-config libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libdbus-1-dev` |
| Fedora | `sudo dnf install gcc pkgconf-pkg-config webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel dbus-devel` |
| macOS | Xcode Command Line Tools (`xcode-select --install`) |
| Windows | MSVC Build Tools (C++ workload) and the WebView2 runtime (preinstalled on Windows 11) |

`libdbus-1-dev` is needed once Tauri is on 2.12 (`tao` 0.37 links `libdbus-sys`).

Without them the build stops in `glib-sys` / `gobject-sys` with `failed to run custom build
command` (pkg-config cannot find `glib-2.0`).

## Build and test

```bash
cargo build
cargo clippy --all-targets
cargo test
```

No system packages? Build in a throwaway container:

```bash
docker run --rm -v "$PWD":/src:ro rust:1-bookworm bash -c \
  'apt-get update && apt-get install -y pkg-config libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev && cp -r /src /w && cd /w && cargo test'
```
