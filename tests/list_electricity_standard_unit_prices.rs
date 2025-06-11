use octopust::Client;

#[tokio::test]
async fn test_list_electricity_standard_unit_prices() {
    let api_key = std::env::var("OCTOPUS_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("OCTOPUS_API_KEY not set. Skipping test.");
        return;
    }
    let client = Client::new(api_key);
    let result = client.list_electricity_tariff_standard_unit_rates("VAR-22-11-01", "E-1R-VAR-22-11-01-J").await;
    println!("{:?}", result);
    match result{
        Ok(standard_unit_rate_response) => {
            println!("Standard Unit Rate Response {:?}", standard_unit_rate_response);
        }
        Err(e) => {
            eprintln!("Error from standard_unit_rate_response: {:?}", e);
            panic!("test_list_electricity_standard_unit_prices failed")
        }
    }
}