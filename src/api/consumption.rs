use reqwest::Client as HttpClient;
use crate::error::{ApiError, OctopustError};
use crate::models::{ConsumptionResponse, MpanInfo};

pub async fn get_electricity_mpan(
    http: &HttpClient,
    base_url: &str,
    mpan: &str
) -> Result<MpanInfo, OctopustError> {
    let url = format!("{}/electricity-meter-points/{}/", base_url.trim_end_matches('/'), mpan);
    let resp = http.get(&url).send().await?;
    let status = resp.status();
    let body_bytes = resp.bytes().await?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    if !status.is_success() {
        return Err(OctopustError::Api(ApiError {
            status,
            message: format!("API returned error status {}: {}", status, body_str),
        }));
    }
    
    let mpan_info: MpanInfo = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse mpan info JSON: {}. Response body: {}",
                e, body_str
            ),
        })
    })?;
    Ok(mpan_info)
}

pub async fn list_electricity_consumption(
    http: &HttpClient,
    base_url: &str,
    mpan: &str,
    serial_number: &str
) -> Result<ConsumptionResponse, OctopustError> {
    let url = format!("{}/electricity-meter-points/{}/meters{}/consumption/", base_url.trim_end_matches('/'), mpan, serial_number);
    let resp = http.get(&url).send().await?;
    let status = resp.status();
    let body_bytes = resp.bytes().await?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    if !status.is_success() {
        return Err(OctopustError::Api(ApiError {
            status,
            message: format!("API returned error status {}: {}", status, body_str),
        }));
    }
    
    let consumption: ConsumptionResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse electricity consumption JSON: {}. Response body: {}",
                e, body_str
            ),
        })
    })?;
    Ok(consumption)
}

pub async fn list_gas_consumption(
    http: &HttpClient,
    base_url: &str,
    mprn: &str,
    serial_number: &str
) -> Result<ConsumptionResponse, OctopustError> {
    let url = format!("{}/gas-meter-points/{}/meters{}/consumption/", base_url.trim_end_matches('/'), mprn, serial_number);
    let resp = http.get(&url).send().await?;
    let status = resp.status();
    let body_bytes = resp.bytes().await?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    if !status.is_success() {
        return Err(OctopustError::Api(ApiError {
            status,
            message: format!("API returned error status {}: {}", status, body_str),
        }));
    }
    
    let consumption: ConsumptionResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse gas consumption JSON: {}. Response body: {}",
                e, body_str
            ),
        })
    })?;
    Ok(consumption)
}