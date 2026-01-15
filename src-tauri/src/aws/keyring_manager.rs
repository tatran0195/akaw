use crate::error::Result;
use super::models::SessionCredentials;
use keyring::Entry;

const SERVICE_NAME: &str = "amf-cli";

pub struct KeyringManager;

impl KeyringManager {
    pub fn store_secret(profile: &str, secret: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, &Self::secret_key(profile))?;
        entry.set_password(secret)?;
        Ok(())
    }

    pub fn get_secret(profile: &str) -> Result<String> {
        let entry = Entry::new(SERVICE_NAME, &Self::secret_key(profile))?;
        let secret = entry.get_password()?;
        Ok(secret)
    }

    pub fn delete_secret(profile: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, &Self::secret_key(profile))?;
        entry.delete_credential()?;
        Ok(())
    }

    pub fn has_secret(profile: &str) -> bool {
        Self::get_secret(profile).is_ok()
    }

    fn secret_key(profile: &str) -> String {
        format!("mfa_secret_{}", profile)
    }

    // Session credentials management
    pub fn store_session_credentials(profile: &str, credentials: &SessionCredentials) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, &Self::session_credentials_key(profile))?;
        let json = serde_json::to_string(credentials)?;
        entry.set_password(&json)?;
        Ok(())
    }

    pub fn get_session_credentials(profile: &str) -> Result<SessionCredentials> {
        let entry = Entry::new(SERVICE_NAME, &Self::session_credentials_key(profile))?;
        let json = entry.get_password()?;
        let credentials: SessionCredentials = serde_json::from_str(&json)?;
        Ok(credentials)
    }

    pub fn delete_session_credentials(profile: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, &Self::session_credentials_key(profile))?;
        entry.delete_credential()?;
        Ok(())
    }

    fn session_credentials_key(profile: &str) -> String {
        format!("session_credentials_{}", profile)
    }
}