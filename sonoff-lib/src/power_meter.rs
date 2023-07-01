use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::device::{SonoffDevice, DevRes};

// JSON models
// ===================================================================

#[derive(Debug, Deserialize)]
struct PowerMeterStatus {
    pub switches: Vec<SwitchOutlet>,
    #[serde(flatten)]
    pub pvc_status: PowerMeterPVC,
    #[serde(flatten)]
    pub overload: PowerMeterOverloads,
    pub faultState: DevPowerMeterFault,
}

#[derive(Debug, Deserialize)]
struct SubDeviceFailure {
    pub faultState: DevPowerMeterFault,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerMeterOverloads {
    pub overload_00: Overload,
    pub overload_01: Overload,
    pub overload_02: Overload,
    pub overload_03: Overload,
}

#[derive(Debug, Deserialize, Serialize)]
struct PowerMeterPVC {
    pub current_00: u32,
    pub voltage_00: u32,
    pub actPow_00: u32,
    pub reactPow_00: u32,
    pub apparentPow_00: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SwitchOutlet {
    pub outlet: u32,
    pub switch: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Overload {
    pub minAP: OverloadValue,
    pub maxAP: OverloadValue,
    pub minV: OverloadValue,
    pub maxV: OverloadValue,
    pub maxC: OverloadValue,
    pub delayTime: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OverloadValue {
    pub en: u32,
    pub val: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Threshold {
    pub actPow: Range,
    pub voltage: Range,
    pub current: Range,
}

#[derive(Debug, serde::Deserialize)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct FaultState {
    pub subDevCom: u32,
    /// Sub-device cse7761 communication error. Array elements are Number type.
    /// The quantity is 4. Elements 0-3 are 1-4 channels respectively.
    /// [0,1] 1: Communication is normal. 0: Communication error.
    pub cse7761Com: Vec<u32>,
}

#[derive(Debug, Deserialize)]
struct DevPowerMeterFault {
    #[serde(flatten)]
    pub fault_state: FaultState,
    pub overloadTrig: Vec<OverloadTrigger>,
    pub cse7761Com: Vec<u32>,
    pub overTemp: Vec<u32>,
    pub overLimit: Vec<OverloadTrigger>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OverloadTrigger {
    pub outlet: u32,
    pub rsn: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SwitchStatusChange {
    pub switches: Vec<SwitchOutlet>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPMSwitchesReq {
    pub sub_dev_id: String,
    pub switches: Vec<DevDataSPMSwitch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevDataSPMSwitch {
    pub outlet: u8,
    pub switch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SPMSubdevListReq { }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPMSubdevList {
    pub sub_dev_list: Vec<DevDataSPMSubdev>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevDataSPMSubdev {
    pub sub_dev_id: String,
    #[serde(rename = "type")]
    pub _type: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPMStatusReq {
    pub sub_dev_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPMStatus {
    pub deviceid: String,
    pub sled_online: String,
    pub ssid: String,
    pub bssid: String,
    pub fw_version: String,
    pub sub_chip_fw_ver: String,
    pub signal_strength: i8,
    pub wifi_connected: bool,
}


#[derive(Debug, serde::Deserialize)]
pub struct SPMSubdevStatus {
    pub fwVersion: String,
    pub switches: Vec<SwitchOutlet>,
    #[serde(flatten)]
    pub overload: PowerMeterOverloads,
    pub faultState: FaultState,
    pub threshold: Threshold,
}


// Implementation
// ===================================================================

pub struct SonoffPowerMeter {
    dev: SonoffDevice,
}

impl From<&SonoffDevice> for SonoffPowerMeter {
    fn from(value: &SonoffDevice) -> Self {
        SonoffPowerMeter { dev: value.to_owned() }
    }
}

impl SonoffPowerMeter {
    fn get_dev(&self) -> &SonoffDevice { &self.dev }

    pub async fn set_switches(&self, sub_dev_id: String, switches: Vec<DevDataSPMSwitch>) -> Result<DevRes> {
        let req_obj = SPMSwitchesReq { sub_dev_id, switches };
        self.get_dev().__request("/switches", req_obj).await
    }

    pub async fn get_subdevs(&self) -> Result<SPMSubdevList> {
        let req_obj = SPMSubdevListReq { };
        self.get_dev().request("/subDevList", req_obj).await
    }

    pub async fn status(&self) -> Result<SPMStatus> {
        let req_obj = SPMStatusReq { sub_dev_id: None };
        self.get_dev().request("/getState", req_obj).await
    }

    pub async fn subdev_status(&self, sub_dev_id: String) -> Result<SPMSubdevStatus> {
        let req_obj = SPMStatusReq { sub_dev_id: Some(sub_dev_id) };
        self.get_dev().request("/getState", req_obj).await
    }
}
