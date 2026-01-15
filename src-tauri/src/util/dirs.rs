use dirs::home_dir;
use std::path::PathBuf;

/// The specific path to the main client configuration file.
pub fn get_user_config_file() -> PathBuf {
    get_user_home_dir().join("config.json")
}

/// The directory for User-context logs.
pub fn get_user_logs_dir() -> PathBuf {
    get_user_home_dir().join("logs")
}

/// The path to the global .wvs directory in the user's home folder.
pub fn get_user_home_dir() -> PathBuf {
    let mut path = home_dir().expect("User home directory should be resolvable");
    path.push(".akaw");
    path
}
