use anyhow::Result;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::device::{SonoffDevice, DevRes};
use crate::switchable::SonoffSwitchable;
use crate::dimmable::SonoffDimmable;

// JSON models
// ===================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DevInfoDataBulb {
    pub switch: String,
    pub ltype: String,
    #[serde(flatten)]
    pub color_type: DevReqBulbColorType,
}

#[derive(Debug, Serialize, Deserialize)]
struct DevReqBulb {
    /// Lamp type ("color" or "white")
    pub ltype: String,
    #[serde(flatten)]
    pub color_type: DevReqBulbColorType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DevReqBulbColorType {
    Color(DevReqBulbColorTypeRGB),
    White(DevReqBulbColorTypeCW),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevReqBulbColorTypeRGB {
    /// Brightness (min=1, max=100)
    pub br: u8,
    /// Red (min=1, max=255)
    pub r: u8,
    /// Green (min=1, max=255)
    pub g: u8,
    /// Blue (min=1, max=255)
    pub b: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevReqBulbColorTypeCW {
    /// Brightness (min=1, max=100)
    pub br: u8,
    /// Color temperature (min=0, max=100)
    pub ct: u8,
}

// Implementation
// ===================================================================

pub struct SonoffBulb {
    dev: SonoffDevice
}

impl From<&SonoffDevice> for SonoffBulb {
    fn from(value: &SonoffDevice) -> Self {
        SonoffBulb { dev: value.to_owned() }
    }
}

#[async_trait]
impl SonoffSwitchable for SonoffBulb {
    fn get_dev(&self) -> &SonoffDevice { &self.dev }

    async fn get_switch(&self) -> Result<bool> {
        let res = self.get_dev().get_info().await?;
        let bulb_info: DevInfoDataBulb = serde_json::from_value(res.per_device_info)?;
        Ok(bulb_info.switch == "on")
    }
}

impl SonoffBulb {
    pub async fn set_bulb(&self, color_type: DevReqBulbColorType) -> Result<DevRes> {
        let ltype = match color_type {
            DevReqBulbColorType::Color(_) => "color",
            DevReqBulbColorType::White(_) => "white",
        }.to_owned();
        let req_obj = DevReqBulb { ltype, color_type };
        self.dev.__request("/dimmable".to_owned(), req_obj).await
    }

    pub async fn color(&self, br: u8, r: u8, g: u8, b: u8) -> Result<DevRes> {
        self.set_bulb(DevReqBulbColorType::Color(DevReqBulbColorTypeRGB { br, r, g, b, })).await
    }

    pub async fn white(&self, br: u8, ct: u8) -> Result<DevRes> {
        self.set_bulb(DevReqBulbColorType::White(DevReqBulbColorTypeCW { br, ct })).await
    }

    pub async fn get_info(&self) -> Result<DevInfoDataBulb> {
        let info = self.dev.get_info().await?;
        Ok(serde_json::from_value(info.per_device_info)?)
    }
}

#[async_trait]
impl SonoffDimmable for SonoffBulb {
    async fn dim(&self, br: u8) -> Result<DevRes> {
        // TODO: dimming must not change color
        self.white(br, 100).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bulb_white() {
        let dev = SonoffDevice::new("http://127.0.0.1:8081");
        let bulb = SonoffBulb::from(&dev);
        bulb.on().await.unwrap();
    }
}
