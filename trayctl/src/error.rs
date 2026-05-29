use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CtlError {
    #[error("cannot reach trayd daemon at {0}")]
    DaemonUnreachable(String),
    #[error("IPC error: {0}")]
    Ipc(String),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid dmenu command (empty string): {0:?}")]
    InvalidDmenuCmd(String),
    #[error("failed to spawn dmenu process: {0}")]
    DmenuSpawn(String),
}
