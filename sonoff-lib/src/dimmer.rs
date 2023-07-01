use anyhow::{Result};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::device::{SonoffDevice, DevRes};
use crate::dimmable::SonoffDimmable;
use crate::switchable::SonoffSwitchable;

// JSON models
// ===================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DevInfoDataDimmer {
    pub switch: String,
    pub startup: String,
    pub brightness: u8,
    pub mode: u8,
    pub brightmin: u8,
    pub brightmax: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevReqDimmer {
    pub switch: String,
    pub brightness: u8,
    pub mode: Option<u8>,
    pub brightmin: Option<u8>,
    pub brightmax: Option<u8>,
}

// Implementation
// ===================================================================

pub struct SonoffDimmer {
    dev: SonoffDevice
}

impl From<&SonoffDevice> for SonoffDimmer {
    fn from(value: &SonoffDevice) -> Self {
        SonoffDimmer { dev: value.to_owned() }
    }
}

impl SonoffDimmer {
    pub async fn get_info(&self) -> Result<DevInfoDataDimmer> {
        let info = self.dev.get_info().await?;
        Ok(serde_json::from_value(info.per_device_info)?)
    }
}

#[async_trait]
impl SonoffSwitchable for SonoffDimmer {
    fn get_dev(&self) -> &SonoffDevice { &self.dev }
}

#[async_trait]
impl SonoffDimmable for SonoffDimmer {
    async fn dim(&self, br: u8) -> Result<DevRes> {
        let req_obj = DevReqDimmer {
            switch: "on".to_owned(), // must be "on"
            brightness: br,
            mode: None,
            brightmin: None,
            brightmax: None,
        };
        self.dev.request("/dimmable".to_owned(), req_obj).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dimmer_half() {
        let dev = SonoffDevice::new("http://127.0.0.1:8081");
        let dimmer = SonoffDimmer::from(&dev);
        dimmer.on().await.unwrap();
        dimmer.dim(50).await.unwrap();
    }
}
