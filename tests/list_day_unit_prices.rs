use octopust::{models::ListUnitRatesQuery, Client};

#[tokio::test]
async fn test_list_day_unit_prices() {
    let api_key = std::env::var("OCTOPUS_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("OCTOPUS_API_KEY not set. Skipping test.");
        return;
    }
    let client = Client::new(api_key);
    let result = client.list_electricity_tariff_day_unit_rates(ListUnitRatesQuery {
        product_code: "VAR-22-11-01",
        tariff_code: "E-2R-VAR-22-11-01-J",
        ..Default::default()
    }).await;
    println!("{:?}", result);
    match result{
        Ok(day_unit_rate_response) => {
            println!("Day Unit Rate Response {:?}", day_unit_rate_response);
        }
        Err(e) => {
            eprintln!("Error from day_unit_rate_response: {:?}", e);
            panic!("test_list_day_unit_prices failed")
        }
    }
}