use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// List day unit rate
#[derive(Debug, Serialize, Deserialize)]
pub struct DayUnitRatesResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Vec<DayUnitRate>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayUnitRate {
    pub value_exc_vat: f64,
    pub value_inc_vat: f64,
    pub valid_from: String,
    pub valid_to: String,
    pub payment_method: String,
}