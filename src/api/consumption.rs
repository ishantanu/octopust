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
            message: format!("API returned error status {status}: {body_str}"),
        }));
    }
    
    let mpan_info: MpanInfo = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse mpan info JSON: {e}. Response body: {body_str}",
            ),
        })
    })?;
    Ok(mpan_info)
}

pub async fn list_electricity_consumption(
    http: &HttpClient,
    base_url: &str,
    mpan: &str,
    serial_number: &str,
    period_from: Option<&str>,
    period_to: Option<&str>,
    order_by: Option<&str>,
    page: Option<u32>,
    page_size: Option<u32>
) -> Result<ConsumptionResponse, OctopustError> {
    let url = format!("{}/electricity-meter-points/{}/meters/{}/consumption/", base_url.trim_end_matches('/'), mpan, serial_number);

    // Build query parameters only for values that are Some(...)
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(pf) = period_from {
        params.push(("period_from", pf.to_string()));
    }
    if let Some(pt) = period_to {
        params.push(("period_to", pt.to_string()));
    }
    if let Some(order) = order_by {
        params.push(("order_by", order.to_string()));
    }
    if let Some(p) = page {
        params.push(("page", p.to_string()));
    }
    if let Some(ps) = page_size {
        params.push(("page_size", ps.to_string()));
    }

    let req = http.get(&url).query(&params);
    let resp = req.send().await?;
    let status = resp.status();
    let body_bytes = resp.bytes().await?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    if !status.is_success() {
        return Err(OctopustError::Api(ApiError {
            status,
            message: format!("API returned error status {status}: {body_str}"),
        }));
    }
    
    let consumption: ConsumptionResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse electricity consumption JSON: {e}. Response body: {body_str}"
            ),
        })
    })?;
    Ok(consumption)
}

pub async fn list_gas_consumption(
    http: &HttpClient,
    base_url: &str,
    mprn: &str,
    serial_number: &str,
    period_from: Option<&str>,
    period_to: Option<&str>,
    order_by: Option<&str>,
    page: Option<u32>,
    page_size: Option<u32>
) -> Result<ConsumptionResponse, OctopustError> {
    let url = format!("{}/gas-meter-points/{}/meters/{}/consumption/", base_url.trim_end_matches('/'), mprn, serial_number);
    
    // Build query parameters only for values that are Some(...)
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(pf) = period_from {
        params.push(("period_from", pf.to_string()));
    }
    if let Some(pt) = period_to {
        params.push(("period_to", pt.to_string()));
    }
    if let Some(order) = order_by {
        params.push(("order_by", order.to_string()));
    }
    if let Some(p) = page {
        params.push(("page", p.to_string()));
    }
    if let Some(ps) = page_size {
        params.push(("page_size", ps.to_string()));
    }

    let req = http.get(&url).query(&params);
    let resp = req.send().await?;
    let status = resp.status();
    let body_bytes = resp.bytes().await?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    if !status.is_success() {
        return Err(OctopustError::Api(ApiError {
            status,
            message: format!("API returned error status {status}: {body_str}"),
        }));
    }
    
    let consumption: ConsumptionResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse gas consumption JSON: {e}. Response body: {body_str}"
            ),
        })
    })?;
    Ok(consumption)
}