use crate::error::TraydBinError;
use crate::ipc::server::IpcServer;
use libtrayd::TrayHost;

pub async fn run() -> Result<(), TraydBinError> {
    let socket_path = crate::ipc::default_socket_path();

    if crate::ipc::ping(&socket_path).await.is_ok() {
        return Err(TraydBinError::AlreadyRunning);
    }

    let host = TrayHost::start().await?;
    let server = IpcServer::new(socket_path, host);
    server.run().await
}

#[cfg(test)]
mod tests;
