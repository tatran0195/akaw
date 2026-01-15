use super::models::{AwsCliOutput, MfaDevice, SessionCredentials, VirtualMfaDevice};
use crate::error::AppError;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;

pub struct AwsCli;

impl AwsCli {
    async fn run_command(
        args: Vec<&str>,
        env: Option<HashMap<String, String>>,
    ) -> crate::error::Result<String> {
        let mut command = Command::new("aws");
        command
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(env_vars) = env {
            for (key, value) in env_vars {
                command.env(key, value);
            }
        }

        let output = command.spawn()?.wait_with_output().await?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(stdout)
        } else {
            let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(AppError::AwsCli(err))
        }
    }

    pub async fn create_virtual_mfa_device(
        username: &str,
        outfile: &str,
        profile: Option<&str>,
    ) -> crate::error::Result<VirtualMfaDevice> {
        let mut args = vec![
            "iam",
            "create-virtual-mfa-device",
            "--virtual-mfa-device-name",
            username,
            "--outfile",
            outfile,
            "--bootstrap-method",
            "QRCodePNG",
        ];
        if let Some(p) = profile {
            args.extend_from_slice(&["--profile", p]);
        }

        let output = Self::run_command(args, None).await?;
        let parsed: AwsCliOutput = serde_json::from_str(&output)?;
        parsed
            .virtual_mfa_device
            .ok_or_else(|| AppError::AwsCli("No virtual MFA device in response".to_string()))
    }

    pub async fn enable_mfa_device(
        username: &str,
        serial_number: &str,
        code1: &str,
        code2: &str,
        profile: Option<&str>,
    ) -> crate::error::Result<()> {
        let mut args = vec![
            "iam",
            "enable-mfa-device",
            "--user-name",
            username,
            "--serial-number",
            serial_number,
            "--authentication-code1",
            code1,
            "--authentication-code2",
            code2,
        ];
        if let Some(p) = profile {
            args.extend_from_slice(&["--profile", p]);
        }

        Self::run_command(args, None).await?;
        Ok(())
    }

    pub async fn get_session_token(
        serial_number: &str,
        token_code: &str,
        profile: Option<&str>,
    ) -> crate::error::Result<SessionCredentials> {
        let mut args = vec![
            "sts",
            "get-session-token",
            "--serial-number",
            serial_number,
            "--token-code",
            token_code,
        ];
        if let Some(p) = profile {
            args.extend_from_slice(&["--profile", p]);
        }

        let output = Self::run_command(args, None).await?;
        let parsed: AwsCliOutput = serde_json::from_str(&output)?;
        let creds = parsed
            .credentials
            .ok_or_else(|| AppError::AwsCli("No credentials in response".to_string()))?;

        Ok(SessionCredentials {
            access_key_id: creds.access_key_id,
            secret_access_key: creds.secret_access_key,
            session_token: creds.session_token,
            expiration: creds.expiration,
        })
    }

    pub async fn list_mfa_devices(
        username: &str,
        profile: Option<&str>,
    ) -> crate::error::Result<Vec<MfaDevice>> {
        let mut args = vec!["iam", "list-mfa-devices", "--user-name", username];
        if let Some(p) = profile {
            args.extend_from_slice(&["--profile", p]);
        }

        let output = Self::run_command(args, None).await?;
        let parsed: AwsCliOutput = serde_json::from_str(&output)?;
        Ok(parsed.mfa_devices.unwrap_or_default())
    }

    pub async fn get_caller_identity(
        profile: Option<&str>,
    ) -> crate::error::Result<serde_json::Value> {
        let mut args = vec!["sts", "get-caller-identity"];
        if let Some(p) = profile {
            args.extend_from_slice(&["--profile", p]);
        }

        let output = Self::run_command(args, None).await?;
        serde_json::from_str(&output).map_err(Into::into)
    }
}
