use octopust::Client;

#[tokio::test]
 async fn test_list_products() {
    let api_key = std::env::var("OCTOPUS_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("OCTOPUS_API_KEY not set. Skipping test.");
        return;
    }
    let client = Client::new(api_key);
    let result = client.list_products().await;
    match result {
        Ok(products) => {
            println!("Products: {:?}", products);
        }
        Err(e) => {
            eprintln!("Error from get_products: {:?}", e);
            panic!("get_products failed");
        }
    }
}

/* async fn test_retrieve_product() {
    let api_key = std::env::var("OCTOPUS_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("OCTOPUS_API_KEY not set. Skipping test.");
        return;
    }
    let client = Client::new(api_key);
    let result = client.retrieve_product("AGILE-24-10-01").await;
    println!("{:?}", result);
    match result {
        Ok(products) => {
            println!("Products: {:?}", products);
        }
        Err(e) => {
            eprintln!("Error from retrieve_products: {:?}", e);
            panic!("get_products failed");
        }
    }
} */

/* async fn test_list_day_unit_prices() {
    let api_key = std::env::var("OCTOPUS_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("OCTOPUS_API_KEY not set. Skipping test.");
        return;
    }
    let client = Client::new(api_key);
    let result = client.list_electricity_tariff_day_unit_rates("VAR-17-01-11", "E-1R-VAR-17-01-11-A").await;
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
} */