use thiserror::Error;

#[derive(Debug, Error)]
pub enum TuiError {
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("config: {0}")]
    Config(String),
    #[error("cannot reach trayd daemon: {0}")]
    DaemonUnreachable(String),
    #[error("IPC error: {0}")]
    Ipc(String),
}
