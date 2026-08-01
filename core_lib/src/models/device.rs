use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub user_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub os_info: String,
    pub mac_address: String,
    pub ip_address: Option<String>,
    pub last_seen: DateTime<Utc>,
    pub is_trusted: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Pc,
    Android,
    Ios,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceContact {
    pub id: String,
    pub user_id: String,
    pub contact_name: String,
    pub device_id: String,
    pub device_info: DeviceInfo,
    pub is_favorite: bool,
    pub last_shared: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceKeys {
    pub device_id: String,
    pub public_key: String,
    pub key_algorithm: String,
}

impl DeviceInfo {
    pub fn from_local_machine() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: String::new(), // Set after auth
            device_name: Self::get_machine_name(),
            device_type: Self::detect_device_type(),
            os_info: std::env::consts::OS.to_string(),
            mac_address: get_mac_address(),
            ip_address: Some(get_local_ip()),
            last_seen: Utc::now(),
            is_trusted: false,
            created_at: Utc::now(),
        }
    }

    fn get_machine_name() -> String {
        hostname::get()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }

    fn detect_device_type() -> DeviceType {
        if cfg!(target_os = "windows") || cfg!(target_os = "linux") {
            DeviceType::Pc
        } else if cfg!(target_os = "macos") {
            DeviceType::Pc
        } else {
            DeviceType::Android
        }
    }
}
