use std::{collections::HashMap, time::Duration};

use reqwest::header::{HeaderMap, HeaderName};
use serde::{de::DeserializeOwned, Serialize};


pub struct Rest;

impl Rest {
    pub async fn get<T: DeserializeOwned>(url: &str) -> anyhow::Result<T> {
        let client = reqwest::Client::new();
    
        let res = client
            .get(url)
            .send()
            .await?
            .json::<T>()
            .await?;
    
        Ok(res)
    }

    pub async fn get_with_auth<T: DeserializeOwned>(url: &str, token: &str) -> anyhow::Result<T> {
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("token {}", token))?,
        );
    
        let res = client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json::<T>()
            .await?;
    
        Ok(res)
    }

    pub async fn get_with_custom_headers_and_timeout<T: DeserializeOwned>(
        url: &str, 
        custom_headers: HashMap<String, String>, 
        timeout: Duration
    ) -> anyhow::Result<T>{
        
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()?;
        
        let mut headers = HeaderMap::new();
        for (key, value) in custom_headers {
            let header_name = HeaderName::from_bytes(key.as_bytes())?;  // Consider proper error handling here
            headers.insert(header_name, value.parse()?);  // Consider proper error handling here
        }

        let res = client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json::<T>()
            .await?;
        
        Ok(res)
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(url: &str, body: B) -> anyhow::Result<T> {
        let client = reqwest::Client::new();
    
        let res = client
            .post(url)
            .json(&body)
            .send()
            .await?
            .json::<T>()
            .await?;
    
        Ok(res)
    }

    pub async fn post_with_auth<T: DeserializeOwned, B: Serialize>(url: &str, token: &str, body: B) -> anyhow::Result<T> {
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("token {}", token))?,
        );
    
        let res = client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await?
            .json::<T>()
            .await?;
    
        Ok(res)
    }
}