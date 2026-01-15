use crate::error::{AppError, Result};
use dirs::home_dir;
use std::path::PathBuf;

pub fn check_aws_cli() -> bool {
    std::process::Command::new("aws")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn get_aws_config_path() -> Result<PathBuf> {
    let home =
        home_dir().ok_or_else(|| AppError::Custom("Home directory not found".to_string()))?;
    Ok(PathBuf::from(home).join(".aws").join("config"))
}

pub fn get_aws_credentials_path() -> Result<PathBuf> {
    let home =
        home_dir().ok_or_else(|| AppError::Custom("Home directory not found".to_string()))?;
    Ok(PathBuf::from(home).join(".aws").join("credentials"))
}

pub fn get_aws_sessions_path() -> Result<PathBuf> {
    let home =
        home_dir().ok_or_else(|| AppError::Custom("Home directory not found".to_string()))?;
    Ok(PathBuf::from(home).join(".aws").join("sessions"))
}
