use crate::error::OctopustError;
use crate::models::*;
use crate::api;
use reqwest::{Client as HttpClient, header};
use std::sync::Arc;
use base64::engine::general_purpose;
use base64::Engine as _;

/// Main API client for Octopus Energy.
#[derive(Clone)]
pub struct Client {
    #[allow(dead_code)]
    api_key: String,
    http: Arc<HttpClient>,
    base_url: String,
}

impl Client {
    /// Create a new client with your API key.
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        let api_key = api_key.into();
        let mut headers = header::HeaderMap::new();
        let auth = general_purpose::STANDARD.encode(format!("{}:", api_key));
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Basic {}", auth)).unwrap(),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Basic {}", auth)).unwrap(),
        );
        let http = HttpClient::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Client {
            api_key: "".into(), // already embedded in headers
            http: Arc::new(http),
            base_url: "https://api.octopus.energy/v1/".to_string(),
        }
    }

    /// Get all products.
    pub async fn list_products(&self) -> Result<Vec<Product>, OctopustError> {
        api::products::list_products(&self.http, &self.base_url).await
    }

    /// Retrieve specific product
    pub async fn retrieve_product(&self, product_code: &str) -> Result<ProductDetail, OctopustError> {
        api::products::retrieve_product(&self.http, &self.base_url, product_code).await
    }

    /// List electricity tariff day unit rate
    pub async fn list_electricity_tariff_day_unit_rates(&self, product_code: &str, tariff_code: &str) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_electricity_tariff_day_unit_rates(&self.http, &self.base_url, product_code, tariff_code).await
    }

    /// List electricity tariff night unit rate
    pub async fn list_electricity_tariff_night_unit_rates(&self, product_code: &str, tariff_code: &str) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_electricity_tariff_night_unit_rates(&self.http, &self.base_url, product_code, tariff_code).await
    }

    /// List electricity tariff standard unit rate
    pub async fn list_electricity_tariff_standard_unit_rates(&self, product_code: &str, tariff_code: &str) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_electricity_tariff_standard_unit_rates(&self.http, &self.base_url, product_code, tariff_code).await
    }

    /// List electricity tariff standard unit rate
    pub async fn list_electricity_tariff_standing_charges(&self, product_code: &str, tariff_code: &str) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_electricity_tariff_standing_charges(&self.http, &self.base_url, product_code, tariff_code).await
    }

    /// List gas tariff standard unit rate
    pub async fn list_gas_tariff_standard_unit_rates(&self, product_code: &str, tariff_code: &str) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_gas_tariff_standard_unit_rates(&self.http, &self.base_url, product_code, tariff_code).await
    }

    /// List gas tariff standing charges
    pub async fn list_gas_tariff_standing_charges(&self, product_code: &str, tariff_code: &str) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_gas_tariff_standing_charges(&self.http, &self.base_url, product_code, tariff_code).await
    }

    /// List electricity consumption
    pub async fn list_electricity_consumption(&self, mpan: &str, serial_number: &str) -> Result<ConsumptionResponse, OctopustError> {
        api::consumption::list_electricity_consumption(&self.http, &self.base_url,mpan, serial_number).await
    }

    /// List gas consumption
    pub async fn list_gas_consumption(&self, mprn: &str, serial_number: &str) -> Result<ConsumptionResponse, OctopustError> {
        api::consumption::list_gas_consumption(&self.http, &self.base_url,mprn, serial_number).await
    }

    /// List grid supply points
    pub async fn list_industry_grid_supply_points(&self) -> Result<GridSupplyPointsResponse, OctopustError> {
        api::industry::list_industry_grid_supply_points(&self.http, &self.base_url).await
    }

    // More endpoint methods would go here...
}