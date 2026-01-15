use super::aws_cli::AwsCli;
use super::aws_config::AwsConfig;
use super::keyring_manager::KeyringManager;
use super::mfa::MfaManager;
use super::session::SessionManager;
use super::session_config::SessionConfigManager;
use super::utils::*;
use crate::{
    error::{AppError, Result},
    util::formatter::log_time_fmt,
};
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Serialize, Deserialize)]
pub struct ProfileInfo {
    pub name: String,
    pub region: Option<String>,
    pub has_mfa: bool,
    pub has_config: bool,
    pub mfa_serial: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileListResponse {
    pub profiles: Vec<ProfileInfo>,
    pub has_configurations: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SessionConfig {
    pub target: String,
    pub local_port: u16,
    pub remote_port: u16,
    pub document_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigResponse {
    pub profile: String,
    pub config: Option<SessionConfig>,
    pub config_path: String,
    pub updated: bool,
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
    pub profile: String,
    pub has_mfa_secret: bool,
    pub identity: Option<IdentityInfo>,
    pub mfa_device: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IdentityInfo {
    pub account: String,
    pub username: String,
    pub arn: String,
}

#[derive(Serialize, Deserialize)]
pub struct MfaSetupResponse {
    pub success: bool,
    pub profile: String,
    pub serial: String,
    pub imported: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectResponse {
    pub profile: String,
    pub target: String,
    pub local_port: u16,
    pub remote_port: u16,
    pub document: String,
    pub expiration: String,
    pub using_cached: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CodeResponse {
    pub code: String,
    pub ttl: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveResponse {
    pub profile: String,
    pub success: bool,
}

#[command]
pub async fn list_aws_profiles() -> Result<ProfileListResponse> {
    let profiles = AwsConfig::list_profiles()?;
    let configured = SessionConfigManager::list_configured_profiles()?;

    let mut profile_infos = Vec::new();

    for profile in profiles {
        let has_mfa = KeyringManager::has_secret(&profile.name);
        let has_config = configured.contains(&profile.name);

        let mfa_serial = if has_mfa {
            if let Ok(username) = AwsConfig::get_username(&profile.name).await {
                MfaManager::fetch_mfa_serial(&username, &profile.name)
                    .await
                    .ok()
            } else {
                None
            }
        } else {
            None
        };

        profile_infos.push(ProfileInfo {
            name: profile.name,
            region: profile.region,
            has_mfa,
            has_config,
            mfa_serial,
        });
    }

    Ok(ProfileListResponse {
        profiles: profile_infos,
        has_configurations: !configured.is_empty(),
    })
}

#[command]
pub async fn show_aws_config(
    profile: String,
    target: Option<String>,
    local_port: Option<u16>,
    remote_port: Option<u16>,
    document: Option<String>,
) -> Result<ConfigResponse> {
    if !AwsConfig::profile_exists(&profile) {
        return Err(AppError::ProfileNotFound(profile));
    }

    let updated =
        target.is_some() || local_port.is_some() || remote_port.is_some() || document.is_some();

    if updated {
        SessionConfigManager::update_config(&profile, target, local_port, remote_port, document)?;
    }

    let config_path = get_aws_sessions_path()?;
    let config = SessionConfigManager::load_config(&profile)?;

    Ok(ConfigResponse {
        profile,
        config: config.map(|c| SessionConfig {
            target: c.target,
            local_port: c.local_port,
            remote_port: c.remote_port,
            document_name: c.document_name,
        }),
        config_path: config_path.display().to_string(),
        updated,
    })
}

#[command]
pub async fn init_aws_configs() -> Result<ConfigResponse> {
    let config_path = get_aws_sessions_path()?;
    let profiles = AwsConfig::list_profiles()?;
    SessionConfigManager::create_config_from_profiles(profiles)?;

    Ok(ConfigResponse {
        profile: String::new(),
        config: None,
        config_path: config_path.display().to_string(),
        updated: true,
    })
}

#[command]
pub async fn check_mfa_status(profile: String) -> Result<StatusResponse> {
    if !AwsConfig::profile_exists(&profile) {
        return Err(AppError::ProfileNotFound(profile));
    }

    let has_mfa_secret = KeyringManager::has_secret(&profile);

    let (identity, mfa_device) = match AwsCli::get_caller_identity(Some(&profile)).await {
        Ok(identity_data) => {
            let account = identity_data["Account"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string();
            let arn = identity_data["Arn"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string();
            let username = arn.split('/').last().unwrap_or(&arn).to_string();

            let mfa_serial = MfaManager::fetch_mfa_serial(&username, &profile).await.ok();

            (
                Some(IdentityInfo {
                    account,
                    username: username.to_string(),
                    arn,
                }),
                mfa_serial,
            )
        }
        Err(_) => (None, None),
    };

    Ok(StatusResponse {
        profile,
        has_mfa_secret,
        identity,
        mfa_device,
    })
}

#[command]
pub async fn setup_mfa_device(
    profile: String,
    import_qr: Option<String>,
) -> Result<MfaSetupResponse> {
    if !check_aws_cli() {
        return Err(AppError::Custom("AWS CLI not found".to_string()));
    }

    if !AwsConfig::profile_exists(&profile) {
        return Err(AppError::ProfileNotFound(profile));
    }

    let username = AwsConfig::get_username(&profile).await?;

    let (serial, imported) = if let Some(qr_path) = import_qr {
        let secret = MfaManager::import_qr_code(&qr_path)?;
        let serial = MfaManager::fetch_mfa_serial(&username, &profile).await?;
        KeyringManager::store_secret(&profile, &secret)?;
        (serial, true)
    } else {
        let (serial, secret) = MfaManager::setup_mfa_device(&username, &profile).await?;
        KeyringManager::store_secret(&profile, &secret)?;
        (serial, false)
    };

    Ok(MfaSetupResponse {
        success: true,
        profile,
        serial,
        imported,
    })
}

#[command]
pub async fn connect(
    profile: String,
    target: Option<String>,
    port: Option<u16>,
    remote_port: Option<u16>,
    document: Option<String>,
) -> Result<ConnectResponse> {
    if !check_aws_cli() {
        return Err(AppError::Custom("AWS CLI not found".to_string()));
    }

    if !AwsConfig::profile_exists(&profile) {
        return Err(AppError::ProfileNotFound(profile));
    }

    let config =
        SessionConfigManager::resolve_config(&profile, target, port, remote_port, document)?;
    let username = AwsConfig::get_username(&profile).await?;
    let serial = MfaManager::fetch_mfa_serial(&username, &profile).await?;

    let (credentials, using_cached) = match KeyringManager::get_session_credentials(&profile) {
        Ok(cached_creds) if cached_creds.is_valid() => (cached_creds, true),
        _ => {
            let secret = KeyringManager::get_secret(&profile)?;
            let token_code = MfaManager::generate_totp_code(&secret)?;
            let credentials =
                AwsCli::get_session_token(&serial, &token_code, Some(&profile)).await?;

            let _ = KeyringManager::store_session_credentials(&profile, &credentials);
            (credentials, false)
        }
    };

    let _child = SessionManager::start_session(
        &profile,
        &config.target,
        config.local_port,
        config.remote_port,
        &config.document_name,
        &credentials,
    )
    .await?;

    Ok(ConnectResponse {
        profile,
        target: config.target,
        local_port: config.local_port,
        remote_port: config.remote_port,
        document: config.document_name,
        expiration: credentials.expiration.format(log_time_fmt()).unwrap(),
        using_cached,
    })
}

#[command]
pub async fn generate_totp_code(profile: String) -> Result<CodeResponse> {
    if !AwsConfig::profile_exists(&profile) {
        return Err(AppError::ProfileNotFound(profile));
    }

    let secret = KeyringManager::get_secret(&profile)?;
    let code = MfaManager::generate_totp_code(&secret)?;
    let ttl = MfaManager::get_time_remaining(&secret)?;

    Ok(CodeResponse { code, ttl })
}

#[command]
pub async fn remove_aws_profile(profile: String) -> Result<RemoveResponse> {
    if !AwsConfig::profile_exists(&profile) {
        return Err(AppError::ProfileNotFound(profile));
    }

    KeyringManager::delete_secret(&profile)?;
    let _ = KeyringManager::delete_session_credentials(&profile);
    SessionConfigManager::remove_config(&profile)?;

    Ok(RemoveResponse {
        profile,
        success: true,
    })
}

#[command]
pub async fn remove_mfa_device(profile: String) -> Result<RemoveResponse> {
    if !AwsConfig::profile_exists(&profile) {
        return Err(AppError::ProfileNotFound(profile));
    }

    KeyringManager::delete_secret(&profile)?;
    let _ = KeyringManager::delete_session_credentials(&profile);

    Ok(RemoveResponse {
        profile,
        success: true,
    })
}

#[command]
pub async fn get_profile_names() -> Result<Vec<String>> {
    let profiles = AwsConfig::list_profiles()?;
    Ok(profiles.iter().map(|p| p.name.clone()).collect())
}
