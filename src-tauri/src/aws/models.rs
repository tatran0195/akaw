use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: OffsetDateTime,
}

impl SessionCredentials {
    /// Check if credentials are still valid (not expired with 5-minute buffer)
    pub fn is_valid(&self) -> bool {
        let now = OffsetDateTime::now_utc();
        let buffer = Duration::minutes(5);
        self.expiration > now + buffer
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AwsCliOutput {
    #[serde(rename = "VirtualMFADevice")]
    pub virtual_mfa_device: Option<VirtualMfaDevice>,
    #[serde(rename = "Credentials")]
    pub credentials: Option<AwsCredentials>,
    #[serde(rename = "MFADevices")]
    pub mfa_devices: Option<Vec<MfaDevice>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualMfaDevice {
    #[serde(rename = "SerialNumber")]
    pub serial_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AwsCredentials {
    #[serde(rename = "AccessKeyId")]
    pub access_key_id: String,
    #[serde(rename = "SecretAccessKey")]
    pub secret_access_key: String,
    #[serde(rename = "SessionToken")]
    pub session_token: String,
    #[serde(rename = "Expiration")]
    pub expiration: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MfaDevice {
    #[serde(rename = "UserName")]
    pub user_name: String,
    #[serde(rename = "SerialNumber")]
    pub serial_number: String,
    #[serde(rename = "EnableDate")]
    pub enable_date: OffsetDateTime,
}
