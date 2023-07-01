use anyhow::Result;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::device::{SonoffDevice, DevRes};
use crate::switchable::SonoffSwitchable;

// JSON models
// ===================================================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonoffSwitchInfo {
    pub switch: String,
    pub startup: String,
    pub pulse: String,
    pub pulse_width: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SonoffSwitchPulseReq {
    pub pulse: String,
    pub pulse_width: u32,
}

// Implementation
// ===================================================================

pub struct SonoffSwitch {
    dev: SonoffDevice
}

impl From<&SonoffDevice> for SonoffSwitch {
    fn from(value: &SonoffDevice) -> Self {
        SonoffSwitch { dev: value.to_owned() }
    }
}

#[async_trait]
impl SonoffSwitchable for SonoffSwitch {
    fn get_dev(&self) -> &SonoffDevice { &self.dev }
}

impl SonoffSwitch {
    /// Only supports multiples of 500ms. Setting `0` deactivates pulse
    pub async fn pulse(&self, milliseconds: u32) -> Result<DevRes> {
        let req_obj = SonoffSwitchPulseReq {
            pulse: if milliseconds == 0 { "off" } else { "on" }.to_owned(),
            pulse_width: milliseconds,
        };
        self.get_dev().__request("/pulse", req_obj).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_switch_on() {
        let dev = SonoffDevice::new("http://127.0.0.1:8081");
        let switch = SonoffSwitch::from(&dev);
        switch.on().await.unwrap();
    }
}
