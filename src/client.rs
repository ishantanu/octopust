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
    pub async fn list_products(
        &self,
        query: ProductQuery<'_>
    ) -> Result<Vec<Product>, OctopustError> {
        api::products::list_products(
            &self.http,
            &self.base_url,
        query
        ).await
    }

    /// Retrieve specific product
    pub async fn retrieve_product(
        &self,
        query: RetrieveProductQuery<'_>
    ) -> Result<ProductDetail, OctopustError> {
        api::products::retrieve_product(
            &self.http, 
            &self.base_url, 
            query.product_code,
            query.tariffs_active_at,
        ).await
    }

    /// List electricity tariff day unit rate
    pub async fn list_electricity_tariff_day_unit_rates(
        &self,
        query: ListUnitRatesQuery<'_>
    ) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_electricity_tariff_day_unit_rates(
            &self.http, 
            &self.base_url,
            query.product_code,
            query.tariff_code,
            query.period_from,
            query.period_to,
            query.page_size,
            query.page
        ).await
    }

    /// List electricity tariff night unit rate
    pub async fn list_electricity_tariff_night_unit_rates(
        &self, 
        query: ListUnitRatesQuery<'_>
    ) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_electricity_tariff_night_unit_rates(
            &self.http, 
            &self.base_url,
            query.product_code,
            query.tariff_code,
            query.period_from,
            query.period_to,
            query.page_size,
            query.page
        ).await
    }

    /// List electricity tariff standard unit rate
    pub async fn list_electricity_tariff_standard_unit_rates(
        &self, 
        query: ListUnitRatesQuery<'_>
    ) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_electricity_tariff_standard_unit_rates(
            &self.http, 
            &self.base_url, 
            query.product_code,
            query.tariff_code,
            query.period_from,
            query.period_to,
            query.page_size,
            query.page
        ).await
    }

    /// List electricity tariff standard unit rate
    pub async fn list_electricity_tariff_standing_charges(
        &self, 
        query: ListUnitRatesQuery<'_>
    ) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_electricity_tariff_standing_charges(
            &self.http, 
            &self.base_url, 
            query.product_code,
            query.tariff_code,
            query.period_from,
            query.period_to,
            query.page_size,
            query.page
        ).await
    }

    /// List gas tariff standard unit rate
    pub async fn list_gas_tariff_standard_unit_rates(
        &self, 
        query: ListUnitRatesQuery<'_>
    ) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_gas_tariff_standard_unit_rates(
            &self.http, 
            &self.base_url, 
            query.product_code,
            query.tariff_code,
            query.period_from,
            query.period_to,
            query.page_size,
            query.page
        ).await
    }

    /// List gas tariff standing charges
    pub async fn list_gas_tariff_standing_charges(
        &self, 
        query: ListUnitRatesQuery<'_>
    ) -> Result<TariffChargesResponse, OctopustError> {
        api::tariffs::list_gas_tariff_standing_charges(
            &self.http, 
            &self.base_url, 
            query.product_code,
            query.tariff_code,
            query.period_from,
            query.period_to,
            query.page_size,
            query.page
        ).await
    }


    /// List electricity consumption, with optional query parameters
    pub async fn list_electricity_consumption(
        &self,
        query: ListElectrictyConsumptionQuery<'_>
    ) -> Result<ConsumptionResponse, OctopustError> {
        api::consumption::list_electricity_consumption(
            &self.http,
            &self.base_url,
            query.mpan,
            query.serial_number,
            query.period_from,
            query.period_to,
            query.order_by,
            query.page,
            query.page_size
        ).await
    }


    /// List electricity consumption, with optional query parameters
    pub async fn list_gas_consumption(
        &self,
        query: ListGasConsumptionQuery<'_>
    ) -> Result<ConsumptionResponse, OctopustError> {
        api::consumption::list_gas_consumption(
            &self.http,
            &self.base_url,
            query.mprn,
            query.serial_number,
            query.period_from,
            query.period_to,
            query.order_by,
            query.page,
            query.page_size,
        ).await
    }

    /// List grid supply points
    pub async fn list_industry_grid_supply_points(
        &self,
        query: ListGridSupplyPointsQuery<'_>
    ) -> Result<GridSupplyPointsResponse, OctopustError> {
        api::industry::list_industry_grid_supply_points(
            &self.http, 
            &self.base_url,
            query.postcode,
            query.page
        ).await
    }

    // More endpoint methods would go here...
}