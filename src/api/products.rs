use reqwest::Client as HttpClient;
use crate::error::{ApiError, OctopustError};
use crate::models::{Product, ProductDetail, ProductQuery};

pub async fn list_products(
    http: &HttpClient,
    base_url: &str,
    query: ProductQuery<'_>
) -> Result<Vec<Product>, OctopustError> {
    let url = format!("{}/products/", base_url.trim_end_matches('/'));
    
    // Build query parameters only for values that are Some(...)
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(aat) = query.available_at {
        params.push(("available_at", aat.to_string()));
    }
    if let Some(br) = query.brand {
        params.push(("brand", br.to_string()));
    }
    if let Some(isb) = query.is_business {
        params.push(("is_business", isb.to_string()));
    }
    if let Some(isg) = query.is_green {
        params.push(("is_green", isg.to_string()));
    }
    if let Some(ish) = query.is_historical {
        params.push(("is_historical", ish.to_string()));
    }
    if let Some(ist) = query.is_tracker {
        params.push(("is_tracker", ist.to_string()));
    }
    if let Some(isv) = query.is_variable {
        params.push(("is_variable", isv.to_string()));
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

    let json: serde_json::Value = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse products JSON: {e}. Response body: {body_str}"
            ),
        })
    })?;

    if let Some(arr) = json.get("results").and_then(|v| v.as_array()) {
        let products: Vec<Product> = serde_json::from_value(arr.clone().into()).map_err(|e| {
            OctopustError::Api(ApiError {
                status,
                message: format!(
                    "Failed to parse products array: {e}. Response body: {body_str}"
                ),
            })
        })?;
        Ok(products)
    } else {
        Err(OctopustError::Api(ApiError {
            status,
            message: format!("Malformed products response: {body_str}"),
        }))
    }
}

pub async fn retrieve_product(
    http: &HttpClient,
    base_url: &str,
    product_code: &str,
    retrieve_product: Option<&str>,
) -> Result<ProductDetail, OctopustError> {
    let url = format!("{}/products/{}/", base_url.trim_end_matches('/'), product_code);
    // Build query parameters only for values that are Some(...)
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(rp ) = retrieve_product {
        params.push(("retrieve_product", rp.to_string()));
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

    let product_detail: ProductDetail = serde_json::from_slice(&body_bytes).map_err(|e| {
        OctopustError::Api(ApiError {
            status,
            message: format!(
                "Failed to parse product JSON: {e}. Response body: {body_str}"
            ),
        })
    })?;
    Ok(product_detail)
}