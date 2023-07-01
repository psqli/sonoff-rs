use anyhow::{Result};
use serde::{Serialize, Deserialize};

use crate::device::{SonoffDevice, DevRes};

// JSON models
// ===================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DevDataR3 {
    pub switches: Option<Vec<DevDataR3Switch>>,
    pub configure: Option<Vec<DevDataR3Startup>>,
    pub pulses: Option<Vec<DevDataR3Pulse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevDataR3Switch {
    pub outlet: u8,
    pub switch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevDataR3Startup {
    pub outlet: u8,
    pub startup: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevDataR3Pulse {
    pub outlet: u8,
    pub pulse: String,
    pub switch: String,
    pub width: u32,
}

// Implementation
// ===================================================================

pub struct SonoffMiniR3 {
    dev: SonoffDevice
}

impl From<&SonoffDevice> for SonoffMiniR3 {
    fn from(value: &SonoffDevice) -> Self {
        SonoffMiniR3 { dev: value.to_owned() }
    }
}

impl SonoffMiniR3 {
    fn get_dev(&self) -> &SonoffDevice { &self.dev }

    pub async fn set_switches(&self, switches: Vec<DevDataR3Switch>) -> Result<DevRes> {
        let req_obj = DevDataR3 {
            switches: Some(switches),
            configure: None,
            pulses: None,
        };
        self.get_dev().__request("/switches", req_obj).await
    }

    pub async fn set_startup(&self, startups: Vec<DevDataR3Startup>) -> Result<DevRes> {
        let req_obj = DevDataR3 {
            switches: None,
            configure: Some(startups),
            pulses: None,
        };
        self.get_dev().__request("/startups", req_obj).await
    }

    pub async fn set_pulses(&self, pulses: Vec<DevDataR3Pulse>) -> Result<DevRes> {
        let req_obj = DevDataR3 {
            switches: None,
            configure: None,
            pulses: Some(pulses),
        };
        self.get_dev().__request("/pulses", req_obj).await
    }
}
