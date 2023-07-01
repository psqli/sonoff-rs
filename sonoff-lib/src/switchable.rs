use anyhow::Result;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::device::{SonoffDevice, DevRes};

// JSON models
// ===================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct SonoffSwitchReq {
    /// Switch state: "on" or "off"
    pub switch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SonoffSwitchStartupReq {
    /// Startup state: "on", "off", or "stay" for the last known state
    pub startup: String,
}

// Implementation
// ===================================================================

#[async_trait]
pub trait SonoffSwitchable {
    fn get_dev(&self) -> &SonoffDevice;

    async fn get_switch(&self) -> Result<bool>;

    /// Valid state: "on", "off".
    async fn set_switch(&self, state: impl Into<String> + Send) -> Result<DevRes> {
        let req_obj = SonoffSwitchReq { switch: state.into() };
        self.get_dev().__request("/switch", req_obj).await
    }

    async fn on(&self) -> Result<DevRes> {
        self.set_switch("on").await
    }

    async fn off(&self) -> Result<DevRes> {
        self.set_switch("off").await
    }

    async fn toggle(&self) -> Result<DevRes> {
        if self.get_switch().await? {
            self.off().await
        } else {
            self.on().await
        }
    }

    /// Set the state for when the device restarts (e.g. after a power loss).
    /// Valid values are: "on", "off", and "stay" for the previous known state.
    /// 
    /// NOTE: Bulbs do NOT support this.
    async fn set_startup(&self, state: String) -> Result<DevRes> {
        let req_obj = SonoffSwitchStartupReq { startup: state };
        self.get_dev().__request("/startup", req_obj).await
    }
}
