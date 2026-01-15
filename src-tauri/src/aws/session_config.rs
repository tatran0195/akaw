use super::{utils::get_aws_sessions_path, aws_config::AwsProfile};
use crate::{error::{AppError, Result}};
use ini::Ini;

const SESSION_CONFIG_FILE: &str = "sessions";

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub target: String,
    pub local_port: u16,
    pub remote_port: u16,
    pub document_name: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            target: String::new(),
            local_port: 13389,
            remote_port: 3389,
            document_name: "AWS-StartPortForwardingSession".to_string(),
        }
    }
}

pub struct SessionConfigManager;

impl SessionConfigManager {
    pub fn load_config(profile: &str) -> Result<Option<SessionConfig>> {
        let config_path = get_aws_sessions_path()?;

        if !config_path.exists() {
            return Ok(None);
        }

        let ini = Ini::load_from_file(&config_path)
            .map_err(|e| AppError::Custom(format!("Failed to read sessions: {}", e)))?;

        let section = ini.section(Some(profile));

        if section.is_none() {
            return Ok(None);
        }

        let section = section.unwrap();

        let target = section
            .get("target")
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| {
                AppError::Custom(format!("No target defined for profile '{}'", profile))
            })?
            .to_string();

        let config = SessionConfig {
            target,
            local_port: section
                .get("local_port")
                .and_then(|s| s.parse().ok())
                .unwrap_or(13389),
            remote_port: section
                .get("remote_port")
                .and_then(|s| s.parse().ok())
                .unwrap_or(3389),
            document_name: section
                .get("document_name")
                .unwrap_or("AWS-StartPortForwardingSession")
                .to_string(),
        };

        Ok(Some(config))
    }

    pub fn create_config_from_profiles(profiles: Vec<AwsProfile>) -> Result<()> {
        let config_path = get_aws_sessions_path()?;

        if config_path.exists() {
            return Err(AppError::Custom(
                "Configuration file already exists".to_string(),
            ));
        }

        let mut ini = Ini::new();

        for profile in profiles {
            Self::init_session_section(&mut ini, &profile.name);
        }

        ini.write_to_file(&config_path)?;
        Ok(())
    }

    pub fn list_configured_profiles() -> Result<Vec<String>> {
        let config_path = get_aws_sessions_path()?;

        if !config_path.exists() {
            return Ok(Vec::new());
        }

        let ini = Ini::load_from_file(&config_path)
            .map_err(|e| AppError::Custom(format!("Failed to read configuration file: {}", e)))?;

        let profiles: Vec<String> = ini
            .iter()
            .filter_map(|(section, _)| section.map(|s| s.to_string()))
            .collect();

        Ok(profiles)
    }

    pub fn resolve_config(
        profile: &str,
        cli_target: Option<String>,
        cli_local_port: Option<u16>,
        cli_remote_port: Option<u16>,
        cli_document: Option<String>,
    ) -> Result<SessionConfig> {
        let file_config = Self::load_config(profile)?;

        let target = cli_target
            .or_else(|| file_config.as_ref().map(|c| c.target.clone()))
            .ok_or_else(|| {
                AppError::Custom(format!(
                    "No target specified. Add to {} or use --target",
                    SESSION_CONFIG_FILE
                ))
            })?;

        let local_port = cli_local_port
            .or_else(|| file_config.as_ref().map(|c| c.local_port))
            .unwrap_or(13389);

        let remote_port = cli_remote_port
            .or_else(|| file_config.as_ref().map(|c| c.remote_port))
            .unwrap_or(3389);

        let document_name = cli_document
            .or_else(|| file_config.as_ref().map(|c| c.document_name.clone()))
            .unwrap_or_else(|| "AWS-StartPortForwardingSession".to_string());

        Ok(SessionConfig {
            target,
            local_port,
            remote_port,
            document_name,
        })
    }

    fn init_session_section(ini: &mut Ini, name: &str) {
        ini.with_section(Some(name))
            .set("target", "")
            .set("local_port", "")
            .set("remote_port", "")
            .set("document_name", "");
    }

    pub fn update_config(
        profile: &str,
        target: Option<String>,
        local_port: Option<u16>,
        remote_port: Option<u16>,
        document_name: Option<String>,
    ) -> Result<()> {
        let config_path = get_aws_sessions_path()?;

        let mut ini = if config_path.exists() {
            Ini::load_from_file(&config_path)
                .map_err(|e| AppError::Custom(format!("Failed to read sessions: {}", e)))?
        } else {
            Ini::new()
        };

        // Ensure section exists (or created if new)
        // We set values directly.

        if let Some(val) = target {
            ini.set_to(Some(profile), "target".to_string(), val);
        }

        if let Some(val) = local_port {
            ini.set_to(Some(profile), "local_port".to_string(), val.to_string());
        }

        if let Some(val) = remote_port {
            ini.set_to(Some(profile), "remote_port".to_string(), val.to_string());
        }

        if let Some(val) = document_name {
            ini.set_to(Some(profile), "document_name".to_string(), val);
        }

        ini.write_to_file(&config_path)
            .map_err(|e| AppError::Custom(format!("Failed to write config: {}", e)))?;

        Ok(())
    }

    pub fn remove_config(profile: &str) -> Result<()> {
        let config_path = get_aws_sessions_path()?;

        if !config_path.exists() {
            return Ok(());
        }

        let mut ini = Ini::load_from_file(&config_path)
            .map_err(|e| AppError::Custom(format!("Failed to read sessions: {}", e)))?;

        ini.delete(Some(profile));

        ini.write_to_file(&config_path)
            .map_err(|e| AppError::Custom(format!("Failed to write config: {}", e)))?;

        Ok(())
    }
}
