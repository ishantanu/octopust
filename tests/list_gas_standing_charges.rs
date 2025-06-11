use octopust::Client;

#[tokio::test]
async fn test_list_gas_standing_charges() {
    let api_key = std::env::var("OCTOPUS_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("OCTOPUS_API_KEY not set. Skipping test.");
        return;
    }
    let client = Client::new(api_key);
    let result = client.list_gas_tariff_standing_charges("VAR-17-01-11", "G-1R-VAR-17-01-11-A").await;
    println!("{:?}", result);
    match result{
        Ok(gas_standing_charges_response) => {
            println!("Gas Standing Charges Response {:?}", gas_standing_charges_response);
        }
        Err(e) => {
            eprintln!("Error from gas_standing_charges_response: {:?}", e);
            panic!("test_list_gas_standing_charges failed")
        }
    }
}