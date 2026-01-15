use crate::error::{AppError, Result};
use super::models::SessionCredentials;
use std::process::Stdio;
use tokio::process::{Child, Command};

pub struct SessionManager;

impl SessionManager {
    pub async fn start_session(
        profile: &str,
        target_id: &str,
        local_port: u16,
        remote_port: u16,
        document_name: &str,
        credentials: &SessionCredentials,
    ) -> Result<Child> {
        if Self::is_port_in_use(local_port) {
            return Err(AppError::PortInUse(local_port));
        }

        let mut cmd = Command::new("aws");

        if !credentials.access_key_id.is_empty() {
            cmd.env("AWS_ACCESS_KEY_ID", &credentials.access_key_id);
        }
        if !credentials.secret_access_key.is_empty() {
            cmd.env("AWS_SECRET_ACCESS_KEY", &credentials.secret_access_key);
        }
        if !credentials.session_token.is_empty() {
            cmd.env("AWS_SESSION_TOKEN", &credentials.session_token);
        }

        cmd.args(&["ssm", "start-session", "--target", target_id]);

        if !document_name.is_empty() {
            cmd.arg("--document-name").arg(document_name);
        }

        cmd.args(&[
            "--parameters",
            &format!("portNumber={},localPortNumber={}", remote_port, local_port),
        ]);

        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

        // Spawn and return the child process handle
        let child = cmd
            .spawn()
            .map_err(|e| AppError::AwsCli(format!("AWS session '{}' failed: {}", profile, e)))?;

        Ok(child)
    }

    fn is_port_in_use(port: u16) -> bool {
        std::net::TcpListener::bind(("127.0.0.1", port)).is_err()
    }
}
