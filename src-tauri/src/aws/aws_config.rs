use super::aws_cli::AwsCli;
use super::utils::{get_aws_config_path, get_aws_credentials_path};
use crate::error::{AppError, Result};
use ini::Ini;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AwsProfile {
    pub name: String,
    pub region: Option<String>,
    pub output: Option<String>,
}

pub struct AwsConfig;

impl AwsConfig {
    pub fn list_profiles() -> Result<Vec<AwsProfile>> {
        let config_path = get_aws_config_path()?;

        if !config_path.exists() {
            return Err(AppError::Custom(
                "AWS config not found. Run 'aws configure' first.".to_string(),
            ));
        }

        let conf = Ini::load_from_file(&config_path)
            .map_err(|e| AppError::Custom(format!("Failed to read config: {}", e)))?;

        let mut profiles = Vec::new();

        for (section, properties) in conf.iter() {
            if let Some(section_name) = section {
                let profile_name = if section_name == "default" {
                    "default".to_string()
                } else {
                    section_name
                        .strip_prefix("profile ")
                        .unwrap_or(section_name)
                        .to_string()
                };

                profiles.push(AwsProfile {
                    name: profile_name,
                    region: properties.get("region").map(|s| s.to_string()),
                    output: properties.get("output").map(|s| s.to_string()),
                });
            }
        }

        if let Ok(creds_path) = get_aws_credentials_path() {
            if creds_path.exists() {
                if let Ok(creds) = Ini::load_from_file(&creds_path) {
                    for (section, _) in creds.iter() {
                        if let Some(section_name) = section {
                            if !profiles.iter().any(|p| p.name == section_name) {
                                profiles.push(AwsProfile {
                                    name: section_name.to_string(),
                                    region: None,
                                    output: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        if profiles.is_empty() {
            return Err(AppError::Custom(
                "No AWS profiles found. Run 'aws configure' first.".to_string(),
            ));
        }

        Ok(profiles)
    }

    pub fn get_profile(name: &str) -> Result<AwsProfile> {
        let profiles = Self::list_profiles()?;

        profiles
            .into_iter()
            .find(|p| p.name == name)
            .ok_or_else(|| AppError::ProfileNotFound(name.to_string()))
    }

    pub fn profile_exists(name: &str) -> bool {
        Self::get_profile(name).is_ok()
    }

    pub async fn get_username(profile: &str) -> Result<String> {
        let identity = AwsCli::get_caller_identity(Some(profile)).await?;

        let arn = identity["Arn"]
            .as_str()
            .ok_or_else(|| AppError::Custom("No Arn in identity response".to_string()))?;

        let username = arn
            .split('/')
            .last()
            .ok_or_else(|| AppError::Custom("Cannot extract username from ARN".to_string()))?
            .to_string();

        Ok(username)
    }
}
