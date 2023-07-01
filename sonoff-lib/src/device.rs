use anyhow::{Result, anyhow};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};

// JSON models
// ===================================================================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevReq {
    pub device_id: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevRes {
    pub seq: u32,
    pub error: u32,
    pub data: Option<serde_json::Value>,
}

// Implementation
// ===================================================================

#[derive(Clone)]
pub struct SonoffDevice {
    pub id: String,
    pub address: String,
}

impl SonoffDevice {

    pub fn new(address: impl Into<String>) -> SonoffDevice {
        SonoffDevice {
            id: "".to_owned(),
            address: address.into(),
        }
    }

    fn post(&self, url_path: impl AsRef<str>) -> Result<RequestBuilder> {
        let mut url = self.address.to_owned();
        let url_path_str = url_path.as_ref();
        url.push_str(&format!("/zeroconf{url_path_str}"));
        let client = reqwest::Client::builder()
            .http1_title_case_headers()
            .build()?;
        Ok(client.post(url))
    }

    pub async fn __request<Treq>(&self, url_path: impl AsRef<str>, req_type: Treq) -> Result<DevRes>
    where
        Treq: Serialize
    {
        let req_obj = DevReq {
            device_id: self.id.to_owned(),
            data: serde_json::to_value(req_type)?,
        };
        //println!("{}", serde_json::to_string(&req_obj)?); // TODO: for debugging
        let res = self.post(url_path)?.body(serde_json::to_string(&req_obj)?)
            .send().await?
            .error_for_status()? // Remove when debugging
            .text().await?;
        //println!("{}", res);  // TODO: for debugging
        let dev_res: DevRes = serde_json::from_str(&res)?;
        Ok(dev_res)
    }

    pub async fn request<Treq, Tres>(&self, url_path: impl AsRef<str>, req_type: Treq) -> Result<Tres>
    where
        Treq: Serialize,
        Tres: DeserializeOwned,
    {
        let dev_res = self.__request(url_path, req_type).await?;
        let Some(data) = dev_res.data else {
            return Err(anyhow!("Bad response from device"))
        };
        Ok(serde_json::from_value(data)?)
    }
}
