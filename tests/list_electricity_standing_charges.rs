use octopust::Client;

#[tokio::test]
async fn test_electricity_standing_charges() {
    let api_key = std::env::var("OCTOPUS_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("OCTOPUS_API_KEY not set. Skipping test.");
        return;
    }
    let client = Client::new(api_key);
    let result = client.list_electricity_tariff_standing_charges("VAR-22-11-01", "E-2R-VAR-22-11-01-J").await;
    println!("{:?}", result);
    match result{
        Ok(electricity_standing_charges_response) => {
            println!("Electricity Standing Charges Response {:?}", electricity_standing_charges_response);
        }
        Err(e) => {
            eprintln!("Error from electricity_standing_charges_response: {:?}", e);
            panic!("test_electricity_standing_charges failed")
        }
    }
}