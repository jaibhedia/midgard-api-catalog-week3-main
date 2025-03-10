use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterfaceError {
    #[error("Failed to build URL")]
    UrlConstruction,

    #[error("Network request failed")]
    NetworkRequest,

    #[error("Failed to deserialize data")]
    Deserialization,

    #[error("Missing or invalid data in response")]
    InvalidResponse,
}

#[derive(Clone)]
pub struct Params {
    pub interval: String,
    pub from: DateTime<Utc>,
    pub count: u16,
}

pub struct Interface {
    pub resource: String,
    pub params: Params,
}

impl Interface {
    pub fn new(resource: String, params: Params) -> Self {
        Self { resource, params }
    }

    pub async fn fetch_data<T>(&self) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        // Build URL
        let url = self.build_url()?;
        println!("Full URL: {}", url);

        // Send request to Midgard API
        let resp = reqwest::get(url.clone())
            .await
            .map_err(|_| InterfaceError::NetworkRequest)?
            .json::<Value>()
            .await
            .map_err(|_| InterfaceError::NetworkRequest)
            .context("Failed to parse response JSON")?;

        // Extract & deserialize "intervals" field
        let intervals_value = resp
            .get("intervals")
            .ok_or(InterfaceError::InvalidResponse)
            .context("No 'intervals' key found in response")?;

        let intervals: Vec<T> = serde_json::from_value(intervals_value.clone())
            .map_err(|_| InterfaceError::Deserialization)?;

        Ok(intervals)
    }

    fn build_url(&self) -> Result<Url> {
        let base_url = "https://midgard.ninerealms.com/v2/history";

        let mut url = Url::parse(&format!("{}/{}", base_url, self.resource))
            .map_err(|_| InterfaceError::UrlConstruction)?;

        url.query_pairs_mut()
            .append_pair("interval", &self.params.interval)
            .append_pair("from", &self.params.from.timestamp().to_string())
            .append_pair("count", &self.params.count.to_string());

        Ok(url)
    }
}
