use log::{error, info, warn};
use std::sync::{Arc, Mutex, MutexGuard};
use tauri::{AppHandle, Emitter, Runtime};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;

mod config;
pub use config::{SidecarConfig, SidecarConfigBuilder};

pub struct SidecarManager {
    state: Arc<Mutex<Option<CommandChild>>>,
    config: SidecarConfig,
}

impl SidecarManager {
    pub fn new(config: SidecarConfig) -> Self {
        SidecarManager {
            state: Arc::new(Mutex::new(None)),
            config,
        }
    }

    /// Spawns the sidecar if it's not already running
    pub fn spawn<R: Runtime>(&self, app_handle: &AppHandle<R>) -> Result<(), String> {
        let mut process_guard = self.lock_state()?;
        if process_guard.is_some() {
            info!("Sidecar is already running");
            return Ok(());
        }

        let mut command = app_handle
            .shell()
            .sidecar(&self.config.sidecar_name)
            .map_err(|e| format!("Sidecar setup failed: {}", e))?;

        // Apply configuration
        command = command.args(self.config.args());
        if let Some(working_dir) = &self.config.working_directory {
            command = command.current_dir(working_dir);
        }

        let (mut rx, child) = command
            .spawn()
            .map_err(|e| format!("Failed to spawn sidecar: {}", e))?;

        // Store child process
        *process_guard = Some(child);

        // Setup async monitoring
        let manager_clone = self.state.clone();
        let app_handle_clone = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        let line = String::from_utf8_lossy(&line);
                        info!("[Sidecar stdout] {}", line);
                        let _ = app_handle_clone.emit("sidecar-stdout", line.to_string());
                    }
                    CommandEvent::Stderr(line) => {
                        let line = String::from_utf8_lossy(&line);
                        error!("[Sidecar stderr] {}", line);
                        let _ = app_handle_clone.emit("sidecar-stderr", line.to_string());
                    }
                    CommandEvent::Error(e) => {
                        error!("[Sidecar error] {}", e);
                    }
                    _ => {}
                }
            }

            // Process exited - clear state
            let mut child_process = manager_clone.lock().unwrap();
            *child_process = None;
            info!("Sidecar process exited");
        });

        Ok(())
    }

    /// Gracefully shutdown the sidecar process
    pub fn shutdown(&self) -> Result<String, String> {
        let mut guard = self.lock_state()?;
        if let Some(mut process) = guard.take() {
            let command = "sidecar shutdown\n";
            if let Err(e) = process.write(command.as_bytes()) {
                warn!("Failed to send shutdown command: {}. Killing process.", e);
                let _ = process.kill();
                return Err(format!("Failed to shutdown gracefully: {}", e));
            }
            Ok("Shutdown command sent successfully".into())
        } else {
            Err("No running sidecar process to shutdown".into())
        }
    }

    /// Check if the sidecar is currently running
    pub fn is_running(&self) -> bool {
        self.state.lock().unwrap().is_some()
    }

    /// Helper method for safe mutex access
    fn lock_state(&self) -> Result<MutexGuard<Option<CommandChild>>, String> {
        self.state
            .lock()
            .map_err(|e| format!("Mutex access failed: {}", e))
    }

    pub fn process_id(&self) -> Option<u32> {
        self.state
            .lock()
            .ok()
            .and_then(|guard| guard.as_ref().map(|child| child.pid()))
    }
}

impl Drop for SidecarManager {
    fn drop(&mut self) {
        if let Ok(mut guard) = self.state.lock() {
            if let Some(process) = guard.take() {
                let _ = process.kill();
            }
        }
    }
}
