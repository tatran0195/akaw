use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("AWS CLI error: {0}")]
    AwsCli(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOTP error: {0}")]
    Totp(String),

    #[error("QR code error: {0}")]
    QrCode(String),

    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    #[error("Port already in use: {0}")]
    PortInUse(u16),

    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("{0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}