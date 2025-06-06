use reqwest::Client as HttpClient;
use crate::error::{ApiError, OctopustError};
use crate::models::{DayUnitRatesResponse, Product, ProductDetail};

pub async fn list_products(
    http: &HttpClient,
    base_url: &str,
) -> Result<Vec<Product>, OctopustError> {
    let url = format!("{}/products/", base_url.trim_end_matches('/'));
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

    let json: serde_json::Value = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse products JSON: {}. Response body: {}",
                e, body_str
            ),
        })
    })?;

    if let Some(arr) = json.get("results").and_then(|v| v.as_array()) {
        let products: Vec<Product> = serde_json::from_value(arr.clone().into()).map_err(|e| {
            OctopustError::Api(ApiError {
                status,
                message: format!(
                    "Failed to parse products array: {}. Response body: {}",
                    e, body_str
                ),
            })
        })?;
        Ok(products)
    } else {
        Err(OctopustError::Api(ApiError {
            status,
            message: format!("Malformed products response: {}", body_str),
        }))
    }
}

pub async fn retrieve_product(
    http: &HttpClient,
    base_url: &str,
    product_code: &str,
) -> Result<ProductDetail, OctopustError> {
    let url = format!("{}/products/{}/", base_url.trim_end_matches('/'), product_code);
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

    let product_detail: ProductDetail = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse product JSON: {}. Response body: {}",
                e, body_str
            ),
        })
    })?;
    Ok(product_detail)
}

pub async fn list_electricity_tariff_day_unit_rates(
    http: &HttpClient,
    base_url: &str,
    product_code: &str,
    tariff_code: &str
) -> Result<DayUnitRatesResponse, OctopustError> {
    let url = format!(
        "{}/products/{}/electricity-tariffs/{}/day-unit-rates/",
        base_url.trim_end_matches('/'),
        product_code,
        tariff_code
    );
    println!("{:?}", url);

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

    let day_unit_rate_response: DayUnitRatesResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
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