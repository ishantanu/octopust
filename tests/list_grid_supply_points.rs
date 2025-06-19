use octopust::Client;

#[tokio::test]
 async fn test_list_grid_supply_points() {
    let api_key = std::env::var("OCTOPUS_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("OCTOPUS_API_KEY not set. Skipping test.");
        return;
    }
    let client = Client::new(api_key);
    let result = client.list_industry_grid_supply_points().await;
    match result {
        Ok(products) => {
            println!("Grid supply points: {:?}", products);
        }
        Err(e) => {
            eprintln!("Error from list_industry_grid_supply_points: {:?}", e);
            panic!("list_industry_grid_supply_points failed");
        }
    }
}