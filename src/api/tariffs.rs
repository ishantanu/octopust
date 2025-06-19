use reqwest::Client as HttpClient;
use crate::error::{ApiError, OctopustError};
use crate::models::TariffChargesResponse;

pub async fn list_electricity_tariff_day_unit_rates(
    http: &HttpClient,
    base_url: &str,
    product_code: &str,
    tariff_code: &str
) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/electricity-tariffs/{}/day-unit-rates/",
        base_url.trim_end_matches('/'),
        product_code,
        tariff_code
    );

    let resp = http.get(&url).send().await?;
    let status = resp.status();
    let body_bytes = resp.bytes().await?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    if !status.is_success() {
        return Err(OctopustError::Api(ApiError {
            status,
            message: format!(
                "API returned error status {}: {}",
                status, body_str
            ),
        }));
    }

    let day_unit_rate_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse day unit rates JSON: {}. Response body: {}",
                e, body_str
            ),
        })
    })?;
    Ok(day_unit_rate_response)
}

pub async fn list_electricity_tariff_night_unit_rates(
    http: &HttpClient,
    base_url: &str,
    product_code: &str,
    tariff_code: &str

) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/electricity-tariffs/{}/night-unit-rates/",
        base_url.trim_end_matches('/'),
        product_code,
        tariff_code
    );

    let resp = http.get(&url).send().await?;
    let status = resp.status();
    let body_bytes = resp.bytes().await?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    if !status.is_success() {
        return Err(OctopustError::Api(ApiError {
            status,
            message: format!(
                "API returned error status {}: {}",
                status, body_str
            ),
        }));
    }

    let night_unit_rate_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse night unit rates JSON: {}. Response body: {}",
                e, body_str
            ),
        })
    })?;
    Ok(night_unit_rate_response)
}

pub async fn list_electricity_tariff_standard_unit_rates(
    http: &HttpClient,
    base_url: &str,
    product_code: &str,
    tariff_code: &str

) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/electricity-tariffs/{}/standard-unit-rates/",
        base_url.trim_end_matches('/'),
        product_code,
        tariff_code
    );

    let resp = http.get(&url).send().await?;
    let status = resp.status();
    let body_bytes = resp.bytes().await?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    if !status.is_success() {
        return Err(OctopustError::Api(ApiError {
            status,
            message: format!(
                "API returned error status {}: {}",
                status, body_str
            ),
        }));
    }

    let standard_unit_rate_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse electricity tariff standard unit rates JSON: {}. Response body: {}",
                e, body_str
            ),
        })
    })?;
    Ok(standard_unit_rate_response)
}

pub async fn list_electricity_tariff_standing_charges(
    http: &HttpClient,
    base_url: &str,
    product_code: &str,
    tariff_code: &str

) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/electricity-tariffs/{}/standing-charges/",
        base_url.trim_end_matches('/'),
        product_code,
        tariff_code
    );

    let resp = http.get(&url).send().await?;
    let status = resp.status();
    let body_bytes = resp.bytes().await?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    if !status.is_success() {
        return Err(OctopustError::Api(ApiError {
            status,
            message: format!(
                "API returned error status {}: {}",
                status, body_str
            ),
        }));
    }

    let standing_charges_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse electricity standing charges JSON: {}. Response body: {}",
                e, body_str
            ),
        })
    })?;
    Ok(standing_charges_response)
}

pub async fn list_gas_tariff_standard_unit_rates(
    http: &HttpClient,
    base_url: &str,
    product_code: &str,
    tariff_code: &str
) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/gas-tariffs/{}/standard-unit-rates/",
        base_url.trim_end_matches('/'),
        product_code,
        tariff_code
    );

    let resp = http.get(&url).send().await?;
    let status = resp.status();
    let body_bytes = resp.bytes().await?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    if !status.is_success() {
        return Err(OctopustError::Api(ApiError {
            status,
            message: format!(
                "API returned error status {}: {}",
                status, body_str
            ),
        }));
    }

    let gas_unit_rate_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse gas tariff standard unit rates JSON: {}. Response body: {}",
                e, body_str
            ),
        })
    })?;
    Ok(gas_unit_rate_response)
}

pub async fn list_gas_tariff_standing_charges(
    http: &HttpClient,
    base_url: &str,
    product_code: &str,
    tariff_code: &str
) -> Result<TariffChargesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/gas-tariffs/{}/standing-charges/",
        base_url.trim_end_matches('/'),
        product_code,
        tariff_code
    );

    let resp = http.get(&url).send().await?;
    let status = resp.status();
    let body_bytes = resp.bytes().await?;
    let body_str = String::from_utf8_lossy(&body_bytes);

    if !status.is_success() {
        return Err(OctopustError::Api(ApiError {
            status,
            message: format!(
                "API returned error status {}: {}",
                status, body_str
            ),
        }));
    }

    let gas_standing_charges_response: TariffChargesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse gas tariff standing charges JSON: {}. Response body: {}",
                e, body_str
            ),
        })
    })?;
    Ok(gas_standing_charges_response)
}