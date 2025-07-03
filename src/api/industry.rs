use reqwest::Client as HttpClient;
use crate::error::{ApiError, OctopustError};
use crate::models::{GridSupplyPointsResponse, ListGridSupplyPointsQuery};

pub async fn list_industry_grid_supply_points(
    http: &HttpClient,
    base_url: &str,
    query: ListGridSupplyPointsQuery<'_>
) -> Result<GridSupplyPointsResponse, OctopustError> {
    let url = format!("{}/industry/grid-supply-points/", base_url.trim_end_matches('/'));
    
    // Build query parameters only for values that are Some(...)
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(p) = query.postcode {
        params.push(("postcode", p.to_string()));
    }
    if let Some(p) = query.page {
        params.push(("page", p.to_string()));
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
    
    let supply_points: GridSupplyPointsResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse industry grid supply JSON: {e}. Response body: {body_str}"
            ),
        })
    })?;
    Ok(supply_points)
}