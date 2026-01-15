use super::aws_cli::AwsCli;
use crate::error::{AppError, Result};
use image::DynamicImage;
use rqrr::PreparedImage;
use std::collections::HashMap;
use std::time::Duration;
use totp_rs::{Algorithm, Secret, TOTP};
use url::Url;

pub struct MfaManager;

impl MfaManager {
    pub async fn setup_mfa_device(username: &str, profile: &str) -> Result<(String, String)> {
        let temp_file = format!("qr_temp_{}.png", profile);

        let serial_number = Self::create_virtual_mfa(username, profile, &temp_file).await?;
        let secret = Self::extract_secret_from_qr(&temp_file)?;
        let (code1, code2) = Self::generate_consecutive_codes(&secret).await?;
        AwsCli::enable_mfa_device(username, &serial_number, &code1, &code2, Some(profile)).await?;
        Self::verify_mfa_device(username, profile, &serial_number).await?;

        let _ = std::fs::remove_file(&temp_file);

        Ok((serial_number, secret))
    }

    pub fn import_qr_code(image_path: &str) -> Result<String> {
        let img = image::open(image_path)
            .map_err(|e| AppError::QrCode(format!("Failed to open image: {}", e)))?;

        let qr_data = Self::decode_qr_image(img)?;
        Self::parse_secret_from_uri(&qr_data)
    }

    pub async fn fetch_mfa_serial(username: &str, profile: &str) -> Result<String> {
        let devices = AwsCli::list_mfa_devices(username, Some(profile)).await?;

        devices
            .first()
            .map(|d| d.serial_number.clone())
            .ok_or_else(|| AppError::Custom("No MFA device found".to_string()))
    }

    pub fn generate_totp_code(secret: &str) -> Result<String> {
        let totp = Self::create_totp(secret)?;
        totp.generate_current()
            .map_err(|e| AppError::Totp(e.to_string()))
    }

    pub fn get_time_remaining(secret: &str) -> Result<u64> {
        let totp = Self::create_totp(secret)?;
        Ok(totp.ttl().unwrap_or(0))
    }

    fn create_totp(secret: &str) -> Result<TOTP> {
        let secret_bytes = Secret::Encoded(secret.to_string())
            .to_bytes()
            .map_err(|e| AppError::Totp(format!("Invalid secret: {}", e)))?;

        TOTP::new(Algorithm::SHA1, 6, 1, 30, secret_bytes)
            .map_err(|e| AppError::Totp(e.to_string()))
    }

    async fn generate_consecutive_codes(secret: &str) -> Result<(String, String)> {
        let totp = Self::create_totp(secret)?;

        let code1 = totp
            .generate_current()
            .map_err(|e| AppError::Totp(e.to_string()))?;

        tokio::time::sleep(Duration::from_secs(30)).await;

        let code2 = totp
            .generate_current()
            .map_err(|e| AppError::Totp(e.to_string()))?;

        Ok((code1, code2))
    }

    fn extract_secret_from_qr(file_path: &str) -> Result<String> {
        let img = image::open(file_path)
            .map_err(|e| AppError::QrCode(format!("Failed to open QR: {}", e)))?;

        let qr_data = Self::decode_qr_image(img)?;
        Self::parse_secret_from_uri(&qr_data)
    }

    fn decode_qr_image(img: DynamicImage) -> Result<String> {
        let img_luma = img.to_luma8();
        let mut img_prepared = PreparedImage::prepare(img_luma);
        let grids = img_prepared.detect_grids();

        if grids.is_empty() {
            return Err(AppError::QrCode("No QR code found".to_string()));
        }

        let (_meta, content) = grids[0]
            .decode()
            .map_err(|e| AppError::QrCode(format!("Failed to decode QR: {}", e)))?;

        Ok(content)
    }

    fn parse_secret_from_uri(uri: &str) -> Result<String> {
        let url =
            Url::parse(uri).map_err(|e| AppError::QrCode(format!("Invalid OTP URI: {}", e)))?;

        let params: HashMap<_, _> = url.query_pairs().collect();

        params
            .get("secret")
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::QrCode("Secret not found in QR code".to_string()))
    }

    async fn create_virtual_mfa(username: &str, profile: &str, outfile: &str) -> Result<String> {
        let device = AwsCli::create_virtual_mfa_device(username, outfile, Some(profile)).await?;
        Ok(device.serial_number)
    }

    async fn verify_mfa_device(username: &str, profile: &str, serial: &str) -> Result<()> {
        let devices = AwsCli::list_mfa_devices(username, Some(profile)).await?;

        let found = devices.iter().any(|d| d.serial_number == serial);

        if !found {
            return Err(AppError::Custom("MFA device not found in IAM".to_string()));
        }

        Ok(())
    }
}
