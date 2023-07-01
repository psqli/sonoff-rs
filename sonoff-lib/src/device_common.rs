use anyhow::Result;
use serde::{Serialize, Deserialize};

use crate::device::{SonoffDevice, DevRes};

// JSON models
// ===================================================================

#[derive(Debug, Serialize)]
pub struct DevInfoReq { }

/// Wi-Fi configuration
#[derive(Debug, Serialize)]
pub struct WifiSetupReq {
    pub ssid: String,
    pub password: String,
}

/// OTA (Over-The-Air) unlocking
#[derive(Debug, Serialize)]
pub struct UnlockOTAReq { }

/// OTA (Over-The-Air) firmware
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOTAReq {
    download_url: String,
    sha256sum: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevInfo {
    pub deviceid: String,
    pub bssid: Option<String>,
    pub ssid: Option<String>,
    pub signal_strength: Option<i32>,
    pub fw_version: Option<String>,
    pub ota_unlock: Option<bool>,
    #[serde(flatten)]
    pub per_device_info: serde_json::Value,
}

// Implementation
// ===================================================================

impl SonoffDevice {
    pub async fn get_info(&self) -> Result<DevInfo> {
        let req_obj = DevInfoReq {};
        self.request("/info", req_obj).await
    }

    pub async fn set_wifi(&self, ssid: String, password: String) -> Result<DevRes> {
        let req_obj = WifiSetupReq { ssid, password, };
        self.__request("/wifi".to_owned(), req_obj).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_info() {
        let dev = SonoffDevice::new("http://127.0.0.1:8081");
        let dev_info = dev.get_info().await.unwrap();
        println!("{}", serde_json::to_string_pretty(&dev_info).unwrap());
    }
}
