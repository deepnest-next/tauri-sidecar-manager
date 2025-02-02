# Tauri Sidecar Manager

[![crates.io](https://img.shields.io/crates/v/tauri-plugin-sidecar-manager)](https://crates.io/crates/tauri-plugin-sidecar-manager)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust crate for managing Tauri sidecar processes. Handle process spawning, lifecycle management, and I/O streaming with ease. Ideal for running servers in different languages (Python, Node.js etc.) within a Tauri app.

## Features

- 🚀 **Effortless Sidecar Management** - Start/stop processes with configurable options
- 🔒 **Thread-Safe Architecture** - Built on `Arc<Mutex>` for concurrent access
- 📡 **Real-time I/O Streaming** - Capture and forward stdout/stderr to frontend
- ⚡ **Graceful Shutdown** - Dual shutdown strategy (clean command + force kill)
- 🔧 **Builder Pattern Configuration** - Type-safe setup with `SidecarConfigBuilder`
- 📦 **Tauri 2 Ready** - Full compatibility with latest Tauri APIs

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tauri-sidecar-manager = "0.1"
log = "0.4" # Recommended for logging
```

## Usage

### Basic Setup

```rust
// main.rs
use tauri_sidecar_manager::{SidecarManager, SidecarConfig};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let config = SidecarConfig::builder()
                .sidecar_name("my-sidecar")
                .arg("--verbose")
                .working_directory("./bin")
                .build();

            app.manage(SidecarManager::new(config));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_sidecar_command,
            shutdown_sidecar_command,
            is_sidecar_running
        ])
        .run(tauri::generate_context!())
        .expect("Error running application");
}
```

### Frontend Integration

```js
// React example
import { invoke } from "@tauri-apps/api/plugin-prc";

// Start sidecar
const startSidecar = async () => {
  await invoke("start_sidecar_command");
};

// Listen to output
import { listen } from "@tauri-apps/api/event";

listen("sidecar-stdout", (event) => {
  console.log("Sidecar output:", event.payload);
});

listen("sidecar-stderr", (event) => {
  console.error("Sidecar error:", event.payload);
});
```

### Requirements

- single file executable
  - can use pyinstaller for Python or pkg for Javascript
- tauri.conf.json
- capabilities

## Contributing

1. Fork the repository
1. Create feature branch (git checkout -b feat/amazing-feature)
1. Commit changes (git commit -m 'Add amazing feature')
1. Push to branch (git push origin feat/amazing-feature)
1. Open Pull Request
