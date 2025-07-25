use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default)]
pub struct ProductQuery<'a> {
    pub available_at: Option<&'a str>,
    pub brand: Option<&'a str>,
    pub is_business: Option<&'a bool>,
    pub is_green: Option<&'a bool>,
    pub is_historical: Option<&'a bool>,
    pub is_tracker: Option<&'a bool>,
    pub is_variable: Option<&'a bool>,
    pub page: Option<u32>,
}

#[derive(Default)]
pub struct RetrieveProductQuery<'a> {
    pub product_code: &'a str,
    pub tariffs_active_at: Option<&'a str>,
}

#[derive(Default)]
pub struct ListElectrictyConsumptionQuery<'a> {
    pub mpan: &'a str,
    pub group_by: Option<&'a str>,
    pub serial_number: &'a str,
    pub period_from: Option<&'a str>,
    pub period_to: Option<&'a str>,
    pub order_by: Option<&'a str>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Default)]
pub struct ListUnitRatesQuery<'a> {
    pub product_code: &'a str,
    pub tariff_code: &'a str,
    pub period_from: Option<&'a str>,
    pub period_to: Option<&'a str>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Default)]
pub struct ListGridSupplyPointsQuery<'a> {
    pub postcode: Option<&'a str>,
    pub page: Option<u32>,
}

#[derive(Default)]
pub struct ListGasConsumptionQuery<'a> {
    pub mprn: &'a str,
    pub group_by:  Option<&'a str>,
    pub serial_number: &'a str,
    pub period_from: Option<&'a str>,
    pub period_to: Option<&'a str>,
    pub order_by: Option<&'a str>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductsResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub direction: String,
    pub code: String,
    pub full_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub is_variable: bool,
    pub is_green: bool,
    pub is_tracker: bool,
    pub is_prepay: bool,
    pub is_business: bool,
    pub is_restricted: bool,
    pub term: Option<u32>,
    pub available_from: Option<String>,
    pub available_to: Option<String>,
    pub brand: Option<String>,
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub method: String,
    pub rel: String,
}

/// Retrieve product details

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDetail {
    pub tariffs_active_at: Option<String>,
    pub code: String,
    pub full_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub is_variable: bool,
    pub is_green: bool,
    pub is_tracker: bool,
    pub is_prepay: bool,
    pub is_business: bool,
    pub is_restricted: bool,
    pub term: Option<u32>,
    pub available_from: Option<String>,
    pub available_to: Option<String>,
    pub brand: Option<String>,
    pub links: Vec<Link>,

    pub single_register_electricity_tariffs: Option<HashMap<String, ElectricityTariffType>>,
    pub dual_register_electricity_tariffs: Option<HashMap<String, ElectricityTariffType>>,
    pub single_register_gas_tariffs: Option<HashMap<String, GasTariffType>>,

    pub sample_quotes: Option<HashMap<String, QuoteType>>,
    pub sample_consumption: Option<SampleConsumption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectricityTariffType {
    pub direct_debit_monthly: Option<ElectricityTariff>,
    pub direct_debit_quarterly: Option<ElectricityTariff>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GasTariffType {
    pub direct_debit_monthly: Option<GasTariff>,
    pub direct_debit_quarterly: Option<GasTariff>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectricityTariff {
    pub code: Option<String>,
    pub standard_unit_rate_exc_vat: Option<f64>,
    pub standard_unit_rate_inc_vat: Option<f64>,
    pub standing_charge_exc_vat: Option<f64>,
    pub standing_charge_inc_vat: Option<f64>,
    pub online_discount_exc_vat: Option<f64>,
    pub online_discount_inc_vat: Option<f64>,
    pub dual_fuel_discount_exc_vat: Option<f64>,
    pub dual_fuel_discount_inc_vat: Option<f64>,
    pub exit_fees_exc_vat: Option<f64>,
    pub exit_fees_inc_vat: Option<f64>,
    pub exit_fees_type: Option<String>,
    pub links: Option<Vec<Link>>,
}

pub type GasTariff = ElectricityTariff; // Gas and electricity tariffs have the same structure

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteType {
    pub direct_debit_monthly: Option<QuoteCategory>,
    pub direct_debit_quarterly: Option<QuoteCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteCategory {
    pub electricity_single_rate: Option<Quote>,
    pub electricity_dual_rate: Option<Quote>,
    pub dual_fuel_single_rate: Option<Quote>,
    pub dual_fuel_dual_rate: Option<Quote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    pub annual_cost_inc_vat: Option<f64>,
    pub annual_cost_exc_vat: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleConsumption {
    pub electricity_single_rate: Option<ElectricitySingleRate>,
    pub electricity_dual_rate: Option<ElectricityDualRate>,
    pub dual_fuel_single_rate: Option<DualFuelSingleRate>,
    pub dual_fuel_dual_rate: Option<DualFuelDualRate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectricitySingleRate {
    pub electricity_standard: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectricityDualRate {
    pub electricity_day: Option<u32>,
    pub electricity_night: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DualFuelSingleRate {
    pub electricity_standard: Option<u32>,
    pub gas_standard: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DualFuelDualRate {
    pub electricity_day: Option<u32>,
    pub electricity_night: Option<u32>,
    pub gas_standard: Option<u32>,
}

/// Tariff Charges - day, night, standard, standing, etc
#[derive(Debug, Deserialize)]
pub struct TariffCharge {
    pub value_exc_vat: f64,
    pub value_inc_vat: f64,
    pub valid_from: String,
    pub valid_to: Option<String>,
    pub payment_method: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TariffChargesResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<TariffCharge>,
}

/// Consumption
#[derive(Debug, Deserialize)]
pub struct ConsumptionReading {
    pub consumption: f64,
    pub interval_start: String,
    pub interval_end: String,
}

#[derive(Debug, Deserialize)]
pub struct ConsumptionResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<ConsumptionReading>,
}

// Supply points
#[derive(Debug, Deserialize)]
pub struct GridSupplyPoint {
    pub group_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GridSupplyPointsResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<GridSupplyPoint>,
}

// Get electricity mpan
#[derive(Debug, Deserialize)]
pub struct MpanInfo {
    pub gsp: String,
    pub mpan: String,
    pub profile_class: u8,
}