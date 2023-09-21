use std::{collections::HashMap, time::Duration, fmt::Debug};

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

    pub async fn get_with_auth<T: DeserializeOwned + Debug>(url: &str, token: &str) -> anyhow::Result<T> {
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("token {}", token))?,
        );
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static("cnctd_rest"),
        );
    
        let response = client
            .get(url)
            .headers(headers)
            .send()
            .await?;
    
        // Check if the request was successful
        if response.status().is_success() {
            let res: T = response.json().await?;
            // println!("response: {:?}", res);
            Ok(res)
        } else {
            // Print the raw response text for debugging
            let raw_response = response.text().await?;
            println!("Raw response: {}", raw_response);
            Err(anyhow::anyhow!("Received an error response"))
        }
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