use reqwest::Client as HttpClient;
use crate::error::{ApiError, OctopustError};
use crate::models::{ListUnitRatesQuery, TariffChargesResponse};

pub async fn list_electricity_tariff_day_unit_rates(
    http: &HttpClient,
    base_url: &str,
    query: ListUnitRatesQuery<'_>
) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/electricity-tariffs/{}/day-unit-rates/",
        base_url.trim_end_matches('/'),
        query.product_code,
        query.tariff_code
    );

    // Build query parameters only for values that are Some(...)
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(pf) = query.period_from {
        params.push(("period_from", pf.to_string()));
    }
    if let Some(pt) = query.period_to {
        params.push(("period_to", pt.to_string()));
    }
    if let Some(p) = query.page {
        params.push(("page", p.to_string()));
    }
    if let Some(ps) = query.page_size {
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
            message: format!(
                "API returned error status {status}: {body_str}"
            ),
        }));
    }

    let day_unit_rate_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse day unit rates JSON: {e}. Response body: {body_str}"
            ),
        })
    })?;
    Ok(day_unit_rate_response)
}

pub async fn list_electricity_tariff_night_unit_rates(
    http: &HttpClient,
    base_url: &str,
    query: ListUnitRatesQuery<'_>
) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/electricity-tariffs/{}/night-unit-rates/",
        base_url.trim_end_matches('/'),
        query.product_code,
        query.tariff_code
    );

    // Build query parameters only for values that are Some(...)
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(pf) = query.period_from {
        params.push(("period_from", pf.to_string()));
    }
    if let Some(pt) = query.period_to {
        params.push(("period_to", pt.to_string()));
    }
    if let Some(p) = query.page {
        params.push(("page", p.to_string()));
    }
    if let Some(ps) = query.page_size {
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
            message: format!(
                "API returned error status {status}: {body_str}"
            ),
        }));
    }

    let night_unit_rate_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse night unit rates JSON: {e}. Response body: {body_str}"
            ),
        })
    })?;
    Ok(night_unit_rate_response)
}

pub async fn list_electricity_tariff_standard_unit_rates(
    http: &HttpClient,
    base_url: &str,
    query: ListUnitRatesQuery<'_>
) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/electricity-tariffs/{}/standard-unit-rates/",
        base_url.trim_end_matches('/'),
        query.product_code,
        query.tariff_code
    );

    // Build query parameters only for values that are Some(...)
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(pf) = query.period_from {
        params.push(("period_from", pf.to_string()));
    }
    if let Some(pt) = query.period_to {
        params.push(("period_to", pt.to_string()));
    }
    if let Some(p) = query.page {
        params.push(("page", p.to_string()));
    }
    if let Some(ps) = query.page_size {
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
            message: format!(
                "API returned error status {status}: {body_str}"
            ),
        }));
    }

    let standard_unit_rate_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse electricity tariff standard unit rates JSON: {e}. Response body: {body_str}"
            ),
        })
    })?;
    Ok(standard_unit_rate_response)
}

pub async fn list_electricity_tariff_standing_charges(
    http: &HttpClient,
    base_url: &str,
    query: ListUnitRatesQuery<'_>
) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/electricity-tariffs/{}/standing-charges/",
        base_url.trim_end_matches('/'),
        query.product_code,
        query.tariff_code
    );

    // Build query parameters only for values that are Some(...)
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(pf) = query.period_from {
        params.push(("period_from", pf.to_string()));
    }
    if let Some(pt) = query.period_to {
        params.push(("period_to", pt.to_string()));
    }
    if let Some(p) = query.page {
        params.push(("page", p.to_string()));
    }
    if let Some(ps) = query.page_size {
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
            message: format!(
                "API returned error status {status}: {body_str}"
            ),
        }));
    }

    let standing_charges_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse electricity standing charges JSON: {e}. Response body: {body_str}"
            ),
        })
    })?;
    Ok(standing_charges_response)
}

pub async fn list_gas_tariff_standard_unit_rates(
    http: &HttpClient,
    base_url: &str,
    query: ListUnitRatesQuery<'_>
) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/gas-tariffs/{}/standard-unit-rates/",
        base_url.trim_end_matches('/'),
        query.product_code,
        query.tariff_code
    );

    // Build query parameters only for values that are Some(...)
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(pf) = query.period_from {
        params.push(("period_from", pf.to_string()));
    }
    if let Some(pt) = query.period_to {
        params.push(("period_to", pt.to_string()));
    }
    if let Some(p) = query.page {
        params.push(("page", p.to_string()));
    }
    if let Some(ps) = query.page_size {
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
            message: format!(
                "API returned error status {status}: {body_str}"
            ),
        }));
    }

    let gas_unit_rate_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse gas tariff standard unit rates JSON: {e}. Response body: {body_str}"
            ),
        })
    })?;
    Ok(gas_unit_rate_response)
}

pub async fn list_gas_tariff_standing_charges(
    http: &HttpClient,
    base_url: &str,
    query: ListUnitRatesQuery<'_>
) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/gas-tariffs/{}/standing-charges/",
        base_url.trim_end_matches('/'),
        query.product_code,
        query.tariff_code
    );

    // Build query parameters only for values that are Some(...)
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(pf) = query.period_from {
        params.push(("period_from", pf.to_string()));
    }
    if let Some(pt) = query.period_to {
        params.push(("period_to", pt.to_string()));
    }
    if let Some(p) = query.page {
        params.push(("page", p.to_string()));
    }
    if let Some(ps) = query.page_size {
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
            message: format!(
                "API returned error status {status}: {body_str}"
            ),
        }));
    }

    let gas_standing_charges_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse gas tariff standing charges JSON: {e}. Response body: {body_str}"
            ),
        })
    })?;
    Ok(gas_standing_charges_response)
}