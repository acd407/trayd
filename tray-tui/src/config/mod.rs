//! `$XDG_CONFIG_HOME/tray-tui/config.toml`

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::TuiError;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// Unix socket path (default: `$XDG_RUNTIME_DIR/trayd.sock`).
    #[serde(default)]
    pub socket_path: Option<PathBuf>,
}

impl Config {
    /// Load from an explicit path, or discover via XDG, or return defaults.
    ///
    /// - Explicit path given → error if missing or unparsable.
    /// - XDG path exists → parse it; error if unparsable.
    /// - Nothing found → silent defaults.
    pub fn load(explicit: Option<&Path>) -> Result<Self, TuiError> {
        if let Some(path) = explicit {
            let text = std::fs::read_to_string(path)
                .map_err(|e| TuiError::Config(format!("{}: {e}", path.display())))?;
            return toml::from_str(&text)
                .map_err(|e| TuiError::Config(format!("{}: {e}", path.display())));
        }

        let xdg = Self::xdg_path();
        if xdg.exists() {
            let text = std::fs::read_to_string(&xdg)
                .map_err(|e| TuiError::Config(format!("{}: {e}", xdg.display())))?;
            return toml::from_str(&text)
                .map_err(|e| TuiError::Config(format!("{}: {e}", xdg.display())));
        }

        Ok(Self::default())
    }

    /// `$XDG_CONFIG_HOME/tray-tui/config.toml` (falls back to `~/.config/…`).
    pub fn xdg_path() -> PathBuf {
        let base = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/root".into());
            format!("{home}/.config")
        });
        PathBuf::from(base).join("tray-tui").join("config.toml")
    }

    /// Resolve the active socket path: CLI override → config field → `$XDG_RUNTIME_DIR/trayd.sock`.
    pub fn resolve_socket(&self, cli_override: Option<PathBuf>) -> Result<PathBuf, TuiError> {
        if let Some(p) = cli_override {
            return Ok(p);
        }
        if let Some(p) = &self.socket_path {
            return Ok(p.clone());
        }
        let dir = std::env::var("XDG_RUNTIME_DIR")
            .map_err(|_| TuiError::DaemonUnreachable("XDG_RUNTIME_DIR not set".to_owned()))?;
        Ok(PathBuf::from(dir).join("trayd.sock"))
    }
}

#[cfg(test)]
mod tests;
